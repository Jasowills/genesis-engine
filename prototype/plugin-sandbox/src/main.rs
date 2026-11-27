//! GE-0005 demo runner: executes every fixture under one engine/epoch ticker.
use anyhow::Result;
use ge_plugin_sandbox_spike::{
    bind_probe_listener, build_engine, build_linker, fixture_path, run_module, EpochTicker,
    RunConfig,
};
use wasmtime::Module;

fn main() -> Result<()> {
    let engine = build_engine()?;
    let _ticker = EpochTicker::start(&engine);
    let linker = build_linker(&engine)?;
    // Real listener: a probe TCP failure cannot be blamed on a closed port.
    let (_listener, port) = bind_probe_listener()?;
    let config = RunConfig {
        probe_port: Some(port),
        ..RunConfig::default()
    };
    println!("[setup         ] live host TcpListener on 127.0.0.1:{port}");

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_time()
        .build()?;

    runtime.block_on(async {
        for (label, binary) in [
            ("normal", "normal"),
            ("probe", "probe"),
            ("infinite_loop", "infinite_loop"),
            ("memory_grow", "memory_grow"),
        ] {
            let module = Module::from_file(&engine, fixture_path(binary))?;
            let outcome = run_module(&engine, &linker, &module, &config).await?;
            println!(
                "[{label:14}] elapsed={:?} error={:?} reports={:?}",
                outcome.elapsed, outcome.error, outcome.reports
            );
        }
        Ok(())
    })
}
