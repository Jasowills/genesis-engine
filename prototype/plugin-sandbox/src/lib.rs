//! GE-0005 throwaway evidence prototype: deny-by-default Wasmtime sandbox.
//!
//! This is NOT product code. It exercises core Wasm + WASI Preview 1 modules
//! built by the `guest/` crate. The typed WASI 0.2 component/WIT path is not
//! validated here (see RESULTS.md).

use anyhow::Result;
use std::net::TcpListener;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};
use wasmtime::*;
use wasmtime_wasi::p1::{self, WasiP1Ctx};
use wasmtime_wasi::WasiCtxBuilder;

/// Explicit per-plugin cap. `StoreLimitsBuilder::memory_size` has no default.
pub const MEMORY_LIMIT_BYTES: usize = 8 * 1024 * 1024;
/// How often the timer thread increments the engine epoch.
pub const EPOCH_TICK: Duration = Duration::from_millis(25);
/// Epochs a guest may pass before it is trapped (2 ticks => ~50 ms).
pub const EPOCH_DEADLINE_TICKS: u64 = 2;
/// Wall-clock backstop; epochs do not cover guests blocked inside hostcalls.
pub const RUN_TIMEOUT: Duration = Duration::from_secs(5);
/// Hard cap on hostcall reports retained per invocation (host-side DoS guard).
pub const REPORT_CAP: usize = 64;

/// Store-local state: WASI context, explicit resource caps, host probe reports.
pub struct PluginState {
    wasi: WasiP1Ctx,
    limits: StoreLimits,
    reports: Arc<Mutex<Vec<i32>>>,
    /// Live host listener port offered to the probe fixture. `None`/0 means
    /// no listener was supplied, which the guest reports distinctly.
    probe_port: Option<u16>,
}

/// Knobs used by the binary and tests. Kept explicit rather than relying on
/// Wasmtime permissive defaults.
#[derive(Clone)]
pub struct RunConfig {
    pub memory_limit_bytes: usize,
    pub trap_on_grow_failure: bool,
    pub timeout: Duration,
    /// Port of a real, live host `TcpListener`. Callers must keep the listener
    /// alive for the duration of the run; see `bind_probe_listener`.
    pub probe_port: Option<u16>,
    /// Epoch deadline for this run. Keep the default low for adversarial
    /// fixtures; benign capability probes need a larger budget to finish.
    pub epoch_deadline_ticks: u64,
}

impl Default for RunConfig {
    fn default() -> Self {
        Self {
            memory_limit_bytes: MEMORY_LIMIT_BYTES,
            trap_on_grow_failure: false,
            timeout: RUN_TIMEOUT,
            probe_port: None,
            epoch_deadline_ticks: EPOCH_DEADLINE_TICKS,
        }
    }
}

/// Bind a real loopback listener on an ephemeral port and return
/// `(listener, port)`. The listener must be held by the caller so that a
/// successful TCP connect would be indistinguishable from a policy denial
/// only if the guest actually tries it.
pub fn bind_probe_listener() -> Result<(TcpListener, u16)> {
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let port = listener.local_addr()?.port();
    Ok((listener, port))
}

/// Measured outcome of one guest invocation.
pub struct RunOutcome {
    pub elapsed: Duration,
    /// `None` means the guest returned normally; otherwise a trap/timeout
    /// description (recorded, not asserted to an exact millisecond).
    pub error: Option<String>,
    /// Values sent by the guest through `genesis_probe.report`.
    pub reports: Vec<i32>,
}

impl RunOutcome {
    pub fn succeeded(&self) -> bool {
        self.error.is_none()
    }
}

/// Timer thread that drives epoch interruption. Stops on drop.
pub struct EpochTicker {
    stop: Arc<AtomicBool>,
    handle: Option<JoinHandle<()>>,
}

impl EpochTicker {
    pub fn start(engine: &Engine) -> Self {
        let stop = Arc::new(AtomicBool::new(false));
        let thread_stop = stop.clone();
        let engine = engine.clone();
        let handle = thread::spawn(move || {
            while !thread_stop.load(Ordering::Relaxed) {
                thread::sleep(EPOCH_TICK);
                engine.increment_epoch();
            }
        });
        Self {
            stop,
            handle: Some(handle),
        }
    }
}

impl Drop for EpochTicker {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}

/// Engine with async support (so `tokio::time::timeout` can cancel) and epoch
/// interruption enabled.
pub fn build_engine() -> Result<Engine> {
    let mut config = Config::new();
    // Wasmtime 51 enables async support unconditionally (the old
    // `Config::async_support` toggle is deprecated and a no-op), so
    // `tokio::time::timeout` can cancel `call_async` futures.
    config.epoch_interruption(true);
    Ok(Engine::new(&config)?)
}

/// WASI Preview 1 linker plus the small custom host function used as a probe
/// channel. No network interfaces other than WASI's own are linked.
pub fn build_linker(engine: &Engine) -> Result<Linker<PluginState>> {
    let mut linker = Linker::new(engine);
    p1::add_to_linker_async(&mut linker, |state: &mut PluginState| &mut state.wasi)?;
    linker.func_wrap(
        "genesis_probe",
        "report",
        |mut caller: Caller<'_, PluginState>, value: i32| {
            let mut reports = caller
                .data_mut()
                .reports
                .lock()
                .expect("report mutex poisoned");
            if reports.len() >= REPORT_CAP {
                // Hard cap: discard the new report and explicitly truncate
                // (defensive against a guest flooding this hostcall).
                reports.truncate(REPORT_CAP);
            } else {
                reports.push(value);
            }
        },
    )?;
    linker.func_wrap(
        "genesis_probe",
        "port",
        |caller: Caller<'_, PluginState>| caller.data().probe_port.unwrap_or(0) as i32,
    )?;
    Ok(linker)
}

/// Build an explicitly deny-by-default WASI context and per-store caps.
fn build_state(config: &RunConfig) -> PluginState {
    let mut builder = WasiCtxBuilder::new();
    // Defensive explicit denial: WASI 48+ denies sockets by default, but the
    // docs historically conflicted, and inherit_network() only sets the
    // address check. Deny all three knobs and reject every address.
    builder
        .allow_tcp(false)
        .allow_udp(false)
        .allow_ip_name_lookup(false)
        .socket_addr_check(|_, _| Box::pin(async { false }));
    // Deliberately NOT called: inherit_env, env, args, inherit_stdio,
    // preopened_dir, inherit_network.
    let wasi = builder.build_p1();

    let limits = StoreLimitsBuilder::new()
        .memory_size(config.memory_limit_bytes)
        .instances(1)
        .tables(1)
        .memories(1)
        .trap_on_grow_failure(config.trap_on_grow_failure)
        .build();

    PluginState {
        wasi,
        limits,
        reports: Arc::new(Mutex::new(Vec::new())),
        probe_port: config.probe_port,
    }
}

/// Run one WASI command module (`_start`) under the configured caps.
pub async fn run_module(
    engine: &Engine,
    linker: &Linker<PluginState>,
    module: &Module,
    config: &RunConfig,
) -> Result<RunOutcome> {
    let state = build_state(config);
    let reports = state.reports.clone();
    let mut store = Store::new(engine, state);
    store.limiter(|state| &mut state.limits);
    store.set_epoch_deadline(config.epoch_deadline_ticks);

    let instance = linker.instantiate_async(&mut store, module).await?;
    let start = instance.get_typed_func::<(), ()>(&mut store, "_start")?;

    let started = Instant::now();
    let result = tokio::time::timeout(config.timeout, start.call_async(&mut store, ())).await;
    let elapsed = started.elapsed();

    let error = match result {
        Ok(Ok(())) => None,
        Ok(Err(trap)) => Some(format!("trap: {trap:#}")),
        Err(_) => Some(format!("timeout after {:?}", config.timeout)),
    };

    let final_reports = reports.lock().expect("report mutex poisoned").clone();
    Ok(RunOutcome {
        elapsed,
        error,
        reports: final_reports,
    })
}

/// Path to a release fixture produced by `cargo build --target wasm32-wasip1
/// --release --manifest-path guest/Cargo.toml`.
pub fn fixture_path(binary_name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("guest/target/wasm32-wasip1/release")
        .join(format!("{binary_name}.wasm"))
}
