//! Integration tests for the GE-0005 throwaway sandbox prototype.
//!
//! Timing assertions are deliberately generous: we assert that execution was
//! terminated and print the measured elapsed time for the record, rather than
//! demanding an exact millisecond.

use ge_plugin_sandbox_spike::{
    bind_probe_listener, build_engine, build_linker, fixture_path, run_module, EpochTicker,
    RunConfig, MEMORY_LIMIT_BYTES, REPORT_CAP, RUN_TIMEOUT,
};
use std::time::Duration;
use wasmtime::Module;

/// Upper bound allowing for scheduling jitter around the wall-clock backstop.
fn generous_bound() -> Duration {
    RUN_TIMEOUT + Duration::from_secs(2)
}

#[tokio::test]
async fn normal_plugin_completes_and_returns_value() {
    let engine = build_engine().unwrap();
    let _ticker = EpochTicker::start(&engine);
    let linker = build_linker(&engine).unwrap();
    let module = Module::from_file(&engine, fixture_path("normal")).unwrap();

    let outcome = run_module(&engine, &linker, &module, &RunConfig::default())
        .await
        .unwrap();

    println!(
        "normal: elapsed={:?} error={:?} reports={:?}",
        outcome.elapsed, outcome.error, outcome.reports
    );
    assert!(outcome.succeeded(), "normal guest should return normally");
    assert_eq!(outcome.reports, vec![42]);
    assert!(outcome.elapsed < generous_bound());
}

#[tokio::test]
async fn infinite_loop_is_interrupted_within_bound() {
    let engine = build_engine().unwrap();
    let _ticker = EpochTicker::start(&engine);
    let linker = build_linker(&engine).unwrap();
    let module = Module::from_file(&engine, fixture_path("infinite_loop")).unwrap();

    let outcome = run_module(&engine, &linker, &module, &RunConfig::default())
        .await
        .unwrap();

    println!(
        "infinite_loop: elapsed={:?} error={:?} reports={:?}",
        outcome.elapsed, outcome.error, outcome.reports
    );
    assert!(
        outcome.error.is_some(),
        "runaway guest must be trapped or timed out"
    );
    assert!(
        outcome.elapsed <= generous_bound(),
        "guest must not outlive the wall-clock backstop"
    );
}

#[tokio::test]
async fn memory_growth_is_capped_by_store_limits() {
    let engine = build_engine().unwrap();
    let _ticker = EpochTicker::start(&engine);
    let linker = build_linker(&engine).unwrap();
    let module = Module::from_file(&engine, fixture_path("memory_grow")).unwrap();

    let outcome = run_module(&engine, &linker, &module, &RunConfig::default())
        .await
        .unwrap();

    println!(
        "memory_grow: elapsed={:?} error={:?} reports={:?}",
        outcome.elapsed, outcome.error, outcome.reports
    );
    assert_eq!(outcome.reports.len(), 2, "guest should report both values");
    assert!(
        outcome.reports[0] < 0,
        "64 MiB grow request must be denied by the 8 MiB cap"
    );
    assert!(
        usize::try_from(outcome.reports[1]).unwrap() * 65_536 <= MEMORY_LIMIT_BYTES,
        "final memory size must stay inside the cap"
    );
    assert!(outcome.succeeded(), "denied grow returns -1, not a trap");
}

#[tokio::test]
async fn memory_growth_traps_when_configured_to() {
    let engine = build_engine().unwrap();
    let _ticker = EpochTicker::start(&engine);
    let linker = build_linker(&engine).unwrap();
    let module = Module::from_file(&engine, fixture_path("memory_grow")).unwrap();
    let config = RunConfig {
        trap_on_grow_failure: true,
        ..RunConfig::default()
    };

    let outcome = run_module(&engine, &linker, &module, &config).await.unwrap();

    println!(
        "memory_grow (trap_on_grow_failure): elapsed={:?} error={:?}",
        outcome.elapsed, outcome.error
    );
    assert!(outcome.error.is_some(), "growth failure should trap");
}

#[tokio::test]
async fn wasi_context_has_no_env_stdio_preopens_and_probe_codes() {
    let engine = build_engine().unwrap();
    let _ticker = EpochTicker::start(&engine);
    let linker = build_linker(&engine).unwrap();
    let module = Module::from_file(&engine, fixture_path("probe")).unwrap();
    // Hold a real listener: a TCP connect failure is now a policy denial, not
    // an artefact of connecting to a port with nobody listening.
    let (_listener, port) = bind_probe_listener().unwrap();
    println!("probe: host listener live on 127.0.0.1:{port}");
    let config = RunConfig {
        probe_port: Some(port),
        ..RunConfig::default()
    };

    let outcome = run_module(&engine, &linker, &module, &config)
        .await
        .unwrap();

    println!(
        "probe: elapsed={:?} error={:?} reports={:?}",
        outcome.elapsed, outcome.error, outcome.reports
    );
    assert!(outcome.succeeded());
    // 10 env vars, 20 root unreadable, 30 cwd unreadable, 40 = TCP connect
    // failed despite a live host listener. DNS (50/51) is recorded but not
    // asserted. IMPORTANT: wasm32-wasip1 Rust std returns Unsupported for
    // both sockets and DNS, so 40/50 are not proof of host policy denial;
    // see RESULTS.md for the permissive control experiment.
    assert_eq!(&outcome.reports[..4], &[10, 20, 30, 40]);
    assert_eq!(outcome.reports.len(), 5);
    assert!(
        outcome.reports[4] == 50 || outcome.reports[4] == 51,
        "DNS probe must report a defined value"
    );

    // Static guard: no ambient-authority opt-ins anywhere in the host setup.
    // The stdout/stderr sentinel bytes in the fixture never reach the host
    // terminal because inherit_stdio was never called (see cargo run output).
    let source = include_str!("../src/lib.rs");
    for forbidden in [
        ".inherit_env()",
        ".inherit_stdio()",
        ".inherit_network()",
        ".preopened_dir(",
    ] {
        assert!(
            !source.contains(forbidden),
            "host must not call {forbidden}"
        );
    }
    // Defensive network policy is configured explicitly (static/supplemental
    // evidence, not runtime proof of enforcement in WASIp1).
    for required in [
        ".allow_tcp(false)",
        ".allow_udp(false)",
        ".allow_ip_name_lookup(false)",
        ".socket_addr_check(",
    ] {
        assert!(source.contains(required), "host must configure {required}");
    }
}

#[tokio::test]
async fn report_hostcall_is_bounded() {
    let engine = build_engine().unwrap();
    let _ticker = EpochTicker::start(&engine);
    let linker = build_linker(&engine).unwrap();
    let module = Module::from_file(&engine, fixture_path("report_flood")).unwrap();

    let outcome = run_module(&engine, &linker, &module, &RunConfig::default())
        .await
        .unwrap();

    println!(
        "report_flood: elapsed={:?} error={:?} reports.len={} first={:?} last={:?}",
        outcome.elapsed,
        outcome.error,
        outcome.reports.len(),
        outcome.reports.first(),
        outcome.reports.last()
    );
    assert!(outcome.succeeded());
    assert_eq!(
        outcome.reports.len(),
        REPORT_CAP,
        "100 guest reports must be capped at {REPORT_CAP}"
    );
    assert_eq!(outcome.reports, (0..REPORT_CAP as i32).collect::<Vec<_>>());
}
