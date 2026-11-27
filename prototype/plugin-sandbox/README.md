# GE-0005 plugin runtime feasibility spike — throwaway evidence prototype

**This is throwaway evidence, not product code.** Do not import these modules,
types, or build artifacts into Genesis Engine. It exists only to produce
observable evidence for ticket GE-0005 and ADR-0006 on this machine.

## What it is

A small Rust host that runs untrusted core-Wasm modules under Wasmtime 51.0.0
with deny-by-default capabilities, explicit resource caps and bounded
cancellation. The guest fixtures are built separately for `wasm32-wasip1`:

| Fixture | Behaviour |
| --- | --- |
| `normal` | Reports `42` through a host probe function and returns. |
| `probe` | Reports env count, preopen checks and TCP denial; writes stdout/stderr sentinels. |
| `infinite_loop` | Never returns; must be stopped by epoch interruption / timeout. |
| `memory_grow` | Requests 64 MiB in one `memory.grow`; the 8 MiB cap must deny it. |
| `report_flood` | Sends 100 hostcall reports; host must retain at most 64. |
| `socket_call` | Explicit `wasi_snapshot_preview1::sock_connect` import; demonstrates the import is undefined in this adapter (negative evidence for GE-0005). |
| `network_probe_p2` | WASI 0.2 component using `std::net::TcpStream::connect`; host tests `tests/sandbox_p2.rs` prove `allow_tcp(false)` denies a connect to a live listener while the permissive control succeeds. |

Host design points:

- One `Engine` with `epoch_interruption(true)`; a timer thread increments
  epochs every 25 ms (`EPOCH_DEADLINE_TICKS = 2`, ~50 ms deadline).
- Per-plugin `Store` with an explicit `StoreLimitsBuilder::memory_size(8 MiB)`
  cap (plus instance/table/memory count caps). `trap_on_grow_failure` is a
  test knob.
- `WasiCtxBuilder::new()` with **no** env, args, preopens or inherited stdio;
  `allow_tcp(false)`, `allow_udp(false)`, `allow_ip_name_lookup(false)` and a
  `socket_addr_check` that rejects every address (defensive against the stale
  `new()` doc about TCP defaults).
- Host probe functions: `genesis_probe.report(i32)` (hard-capped at 64 retained
  values) and `genesis_probe.port()` (a real, live loopback listener port so
  the TCP probe cannot fail merely because nobody was listening).
- `tokio::time::timeout(5 s, call_async(...))` as the wall-clock backstop for
  guests blocked in hostcalls, which epochs do not cover.

**Caveat:** on `wasm32-wasip1`, Rust std has no socket/DNS support at all, so
the probe's TCP/DNS codes are **not** runtime proof of policy denial. The typed
WASI 0.2 fixture (`network_probe_p2` + `tests/sandbox_p2.rs`) supplies that
proof with a live listener and a permissive control; see `RESULTS.md`.

## Reproduce

```sh
# from this directory
cargo build --target wasm32-wasip1 --release --manifest-path guest/Cargo.toml
cargo build --target wasm32-wasip2 --release --bin network_probe_p2 --manifest-path guest/Cargo.toml
cargo test
cargo test --test sandbox_p2 -- --nocapture
cargo run
```

Measured commands, outputs, results and limitations are in `RESULTS.md`.

## Scope boundary

This prototype uses **core Wasm + WASI Preview 1** (`wasm32-wasip1`), because
the guest target is already installed and the fixtures are intentionally small.
Typed WASI 0.2 is validated for network-policy denial only (three OSes in CI);
component overhead, blocking-hostcall cancellation, empty-write/invalid-import
fixtures, plugin manifests/signing/rollback are **not** validated here. No secrets are
used or stored anywhere in this directory.
