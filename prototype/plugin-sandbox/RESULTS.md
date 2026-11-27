# GE-0005 plugin runtime feasibility spike — measured results

**Research / execution date:** 2026-11-27 (independent-review fixes applied).
**Status:** prototype evidence only (not product code, not an implementation
claim). No tickets, ADRs, research docs or GitHub items were modified.

## Environment

```text
$ rustc --version && cargo --version && rustup target list --installed && uname -a
rustc 1.98.1 (48a229cea 2026-09-01)
cargo 1.98.1 (797e8a9bc 2026-08-05)
wasm32-wasip1
x86_64-apple-darwin
Darwin Jasons-MacBook-Pro-2.local 25.5.0 Darwin Kernel Version 25.5.0: Tue Jun  9 22:26:23 PDT 2026; root:xnu-12377.121.10~1/RELEASE_X86_64 x86_64
```

## Commands and outputs (verbatim, final runs)

Guest build (includes the five retained fixtures):

```text
$ cargo build --target wasm32-wasip1 --release --manifest-path guest/Cargo.toml
    Finished `release` profile [optimized] target(s) in 0.01s
$ ls -lh guest/target/wasm32-wasip1/release/*.wasm
-rwxr-xr-x@ 1 jasonamadi  staff    35K Nov 27 09:27 guest/target/wasm32-wasip1/release/infinite_loop.wasm
-rwxr-xr-x@ 1 jasonamadi  staff    35K Nov 27 09:27 guest/target/wasm32-wasip1/release/memory_grow.wasm
-rwxr-xr-x@ 1 jasonamadi  staff    35K Nov 27 09:27 guest/target/wasm32-wasip1/release/normal.wasm
-rwxr-xr-x@ 1 jasonamadi  staff    61K Nov 27 09:27 guest/target/wasm32-wasip1/release/probe.wasm
-rwxr-xr-x@ 1 jasonamadi  staff    35K Nov 27 09:27 guest/target/wasm32-wasip1/release/report_flood.wasm
```

Tests:

```text
$ cargo test
   Compiling ge-plugin-sandbox-spike v0.1.0 (/Users/jasonamadi/genesis-engine/prototype/plugin-sandbox)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.72s
     Running unittests src/lib.rs (target/debug/deps/ge_plugin_sandbox_spike-d318dd80aeb7d0fe)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/debug/deps/ge_plugin_sandbox_spike-2e3cd26323a1f62d)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/sandbox.rs (target/debug/deps/sandbox-4bfcca0321d90833)

running 6 tests
test normal_plugin_completes_and_returns_value ... ok
test memory_growth_is_capped_by_store_limits ... ok
test memory_growth_traps_when_configured_to ... ok
test report_hostcall_is_bounded ... ok
test wasi_context_has_no_env_stdio_preopens_and_probe_codes ... ok
test infinite_loop_is_interrupted_within_bound ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.60s

   Doc-tests ge_plugin_sandbox_spike

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Measured timings (test output is otherwise captured by the harness):

```text
$ cargo test -- --nocapture
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.39s
     Running unittests src/lib.rs (target/debug/deps/ge_plugin_sandbox_spike-d318dd80aeb7d0fe)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/debug/deps/ge_plugin_sandbox_spike-2e3cd26323a1f62d)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/sandbox.rs (target/debug/deps/sandbox-4bfcca0321d90833)

running 6 tests
normal: elapsed=196.328µs error=None reports=[42]
memory_grow (trap_on_grow_failure): elapsed=436.961µs error=Some("trap: error while executing at wasm backtrace:\n    0:    0x26c - <unknown>!<wasm function 8>\n    1:    0x25b - <unknown>!<wasm function 7>\n    2:    0x309 - <unknown>!<wasm function 9>\n    3:    0x237 - <unknown>!<wasm function 6>: forcing trap when growing memory to 68222976 bytes")
report_flood: elapsed=138.581µs error=None reports.len=64 first=Some(0) last=Some(63)
test report_hostcall_is_bounded ... ok
test normal_plugin_completes_and_returns_value ... ok
test memory_growth_traps_when_configured_to ... ok
memory_grow: elapsed=89.025µs error=None reports=[-1, 17]
test memory_growth_is_capped_by_store_limits ... ok
infinite_loop: elapsed=59.642105ms error=Some("trap: error while executing at wasm backtrace:\n    0:    0x267 - <unknown>!<wasm function 7>\n    1:    0x244 - <unknown>!<wasm function 6>\n    2:    0x30e - <unknown>!<wasm function 8>\n    3:    0x220 - <unknown>!<wasm function 5>: wasm trap: interrupt") reports=[]
test infinite_loop_is_interrupted_within_bound ... ok
probe: host listener live on 127.0.0.1:55539
probe: elapsed=493.447µs error=None reports=[10, 20, 30, 40, 50]
test wasi_context_has_no_env_stdio_preopens_and_probe_codes ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.34s

   Doc-tests ge_plugin_sandbox_spike

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Demo run (note the real host listener bound before the guest runs):

```text
$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.38s
     Running `target/debug/ge-plugin-sandbox-spike`
[setup         ] live host TcpListener on 127.0.0.1:55549
[normal        ] elapsed=146.164µs error=None reports=[42]
[probe         ] elapsed=572.64µs error=None reports=[10, 20, 30, 40, 50]
[infinite_loop ] elapsed=47.874167ms error=Some("trap: error while executing at wasm backtrace:\n    0:    0x267 - <unknown>!<wasm function 7>\n    1:    0x244 - <unknown>!<wasm function 6>\n    2:    0x30e - <unknown>!<wasm function 8>\n    3:    0x220 - <unknown>!<wasm function 5>: wasm trap: interrupt") reports=[]
[memory_grow   ] elapsed=97.148µs error=None reports=[-1, 17]
```

Neither `GE_SENTINEL_STDOUT` nor `GE_SENTINEL_STDERR` appears in `cargo run`
output: stdio was discarded (no `inherit_stdio()` call).

## Review-fix evidence: live-listener TCP/DNS probes and ambiguity

Structural fix applied: the host binds a real loopback `TcpListener` on an
ephemeral port (`bind_probe_listener`) and keeps it alive for the run; the port
is supplied to the guest via `genesis_probe.port()` (`func_wrap`), and the
guest connects to that live port. Report codes: 39 no port supplied, 40 TCP
failed, 41 TCP succeeded, 50 DNS failed/empty, 51 DNS resolved. All defensive
config is retained (`allow_tcp(false)`, `allow_udp(false)`,
`allow_ip_name_lookup(false)`, reject-all `socket_addr_check`).

**Temporary control experiment** (files `tests/tmp_dns_control.rs` and
`guest/src/bin/tcp_detail.rs` were created, run, and then removed; same probe
fixture and same live listener, but with a permissive
`WasiCtxBuilder` using `inherit_stdio()`, `allow_tcp(true)`, `allow_udp(true)`,
`allow_ip_name_lookup(true)` and a socket_addr_check accepting every address):

```text
$ cargo test --test tmp_dns_control -- --nocapture
   Compiling ge-plugin-sandbox-spike v0.1.0 (/Users/jasonamadi/genesis-engine/prototype/plugin-sandbox)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.90s
     Running tests/tmp_dns_control (target/debug/deps/tmp_dns_control-47fd991d1027237b)

running 1 test
PERMISSIVE_CONTROL port=55377 reports=[10, 20, 30, 40, 50]
test permissive_control ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.43s
```

```text
$ cargo build --target wasm32-wasip1 --release --manifest-path guest/Cargo.toml
   Compiling ge-plugin-sandbox-guest-fixtures v0.1.0 (/Users/jasonamadi/genesis-engine/prototype/plugin-sandbox/guest)
    Finished `release` profile [optimized] target(s) in 1.23s
$ cargo test --test tmp_dns_control -- --nocapture
   Compiling ge-plugin-sandbox-spike v0.1.0 (/Users/jasonamadi/genesis-engine/prototype/plugin-sandbox)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.01s
     Running tests/tmp_dns_control (target/debug/deps/tmp_dns_control-47fd991d1027237b)

running 1 test
TCPDETAIL CONNECT_ERR port=55432 err=Error { kind: Unsupported, message: "operation not supported on this platform" } kind=Unsupported raw=None
TCPDETAIL DNS_ERR err=Error { kind: Unsupported, message: "operation not supported on this platform" } kind=Unsupported raw=None
PERMISSIVE_CONTROL port=55432 reports=[]

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.45s
```

**Conclusion:** with sockets fully allowed, the same guest still reports 40
(TCP) and 50 (DNS); the guest error is `ErrorKind::Unsupported` for both.
`wasm32-wasip1` Rust std has no socket/name-lookup implementation in this
toolchain. Therefore the live-listener fix removes the "no listener" ambiguity,
but the codes **cannot prove host network-policy denial**: runtime network
denial is **UNVERIFIED** on the core-Wasm/WASIp1 path. Host configuration is
explicitly deny-by-default as coded; the static test guard checks this but is
supplemental, not runtime proof.

## Measured results

| Check | Observed | Evidence |
| --- | --- | --- |
| Normal plugin completes | Yes; `reports=[42]` | `cargo run`, `normal_plugin_completes_and_returns_value` |
| Env vars inherited | Zero | probe report `10` |
| Preopened dirs | None readable (`/`, `.` both fail) | probe reports `20`, `30` |
| Stdio | Discarded | no sentinels in output; no `inherit_stdio()` in `src/lib.rs` |
| TCP probe (live host listener) | 40 (failed) even with permissive config; `Unsupported` in guest | `cargo run`, permissive control above; **not policy proof** |
| DNS probe | 50 (failed/empty) in both configs; `Unsupported` in guest | permissive control above; **UNVERIFIED** |
| Infinite loop | Epoch trap in ~48–60 ms; 5 s backstop not reached | `wasm trap: interrupt` |
| 64 MiB `memory.grow` vs 8 MiB cap | Denied (`-1`); memory stayed 17 pages = 1,114,112 bytes | `reports=[-1, 17]` |
| `trap_on_grow_failure(true)` | Traps: `forcing trap when growing memory to 68222976 bytes` | test output |
| 100 hostcall reports | Retained exactly 64 (`0..63`), no growth | `report_hostcall_is_bounded`, `reports.len=64` |
| Test suite | 6 passed, 0 failed | `cargo test` |

Timing assertions remain non-brittle: tests assert termination and a generous
7 s upper bound, and print measured elapsed time; no exact millisecond is
required.

## Residual caveats / honest UNVERIFIED items

- The **5 s `tokio::time::timeout` backstop is configured but not exercised**:
  epoch interruption fires first (~50 ms) in every run. Blocking-hostcall
  cancellation remains UNVERIFIED in practice.
- The **static source guard is supplemental, not proof**: it checks text in
  `src/lib.rs`, not WASI runtime behaviour.
- **WIT / typed WASI 0.2 component path is out of scope and UNVERIFIED** here
  (core Wasm + WASI Preview 1 only); no WIT world, `bindgen!`, or component
  instantiation was built.
- **Empty-write and invalid-import fixtures are out of scope and UNVERIFIED**
  (only normal, probe, infinite-loop, memory-grow and report-flood fixtures).
- **Windows/Linux coverage pending**: macOS x86_64 only; the ticket's
  all-three-OSes criterion is not met.
- Component overhead percentage not measured; no manifest/signing/rollback;
  bulk-op (`memory.copy`) preemption not measured; CVE/advisory monitoring and
  multi-version compatibility not tested.
- Wasmtime 51 note: `Config::async_support(true)` is deprecated and a no-op;
  it was removed (local compile observation, not a sourced product fact).

## Conclusion (evidence only)

On the tested host, explicit `StoreLimits` and epoch interruption reliably
bounded runaway Wasm and denied a 64 MiB allocation, and the hostcall report
buffer is now hard-capped. Env/preopen/stdio denial is supported by guest
observations. Network-policy denial is configured but not runtime-proven on
this WASIp1 path. No final architecture decision is made here.

## GE-0005 follow-up (2026-11-27): why network-policy denial stays UNVERIFIED

A seventh fixture (`guest/src/bin/socket_call.rs`) declares the WASI Preview 1
`sock_connect` import **explicitly**, bypassing Rust std (which has no sockets
on `wasm32-wasip1`). Wasmtime's Preview 1 adapter does not define that import:

```
unknown import: `wasi_snapshot_preview1::sock_connect` has not been defined
```

Test `wasi_preview1_lacks_sock_connect_import` records this link error. So no
`wasm32-wasip1` guest can reach a host TCP policy decision — through std or
through a raw WASI import. This **confirms** the UNVERIFIED label; it must not
be read as proof that a connect attempt was denied. Closing the gap needs the
typed WASI 0.2 component path (`wasi:sockets`) on a supported target, which is
separate follow-up work (ADR-0006 PoC gate).
