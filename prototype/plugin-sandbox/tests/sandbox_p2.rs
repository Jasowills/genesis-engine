//! GE-0005 typed WASI 0.2 probe: prove host network-policy denial with a real
//! component that imports `wasi:sockets` (unlike wasm32-wasip1, whose std has
//! no sockets and whose Preview 1 adapter does not define `sock_connect`).

use ge_plugin_sandbox_spike::bind_probe_listener;
use wasmtime::component::{Component, Linker, ResourceTable};
use wasmtime::{Engine, Store};
use wasmtime_wasi::p2::bindings::Command;
use wasmtime_wasi::p2::pipe::MemoryOutputPipe;
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder, WasiCtxView, WasiView};

struct P2State {
    wasi: WasiCtx,
    table: ResourceTable,
}

impl WasiView for P2State {
    fn ctx(&mut self) -> WasiCtxView<'_> {
        WasiCtxView {
            ctx: &mut self.wasi,
            table: &mut self.table,
        }
    }
}

const COMPONENT: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/guest/target/wasm32-wasip2/release/network_probe_p2.wasm"
);

#[tokio::test]
async fn wasip2_tcp_connect_is_denied_by_host_policy() {
    let engine = Engine::default();
    let component = Component::from_file(&engine, COMPONENT).unwrap();
    let (_listener, port) = bind_probe_listener().unwrap();

    let stdout = MemoryOutputPipe::new(4096);
    let mut builder = WasiCtxBuilder::new();
    builder.arg("network_probe_p2");
    builder.arg(port.to_string());
    builder.stdout(stdout.clone());
    // Deny by default and with an explicit address check (defence in depth).
    builder.allow_tcp(false);
    builder.socket_addr_check(|_, _| Box::pin(async { false }));

    let mut linker = Linker::new(&engine);
    wasmtime_wasi::p2::add_to_linker_async(&mut linker).unwrap();
    let mut store = Store::new(
        &engine,
        P2State {
            wasi: builder.build(),
            table: ResourceTable::new(),
        },
    );

    let command = Command::instantiate_async(&mut store, &component, &linker)
        .await
        .unwrap();
    let program_result = command.wasi_cli_run().call_run(&mut store).await.unwrap();
    assert!(
        program_result.is_ok(),
        "guest should exit normally after denial"
    );

    let output = String::from_utf8_lossy(&stdout.contents()).into_owned();
    println!("p2 policy evidence: {output:?}");
    assert!(
        output.contains("P2_CONNECT_DENIED"),
        "guest must observe a denied TCP connect, got: {output}"
    );
    assert!(
        !output.contains("P2_CONNECT_OK"),
        "policy allowed the connect"
    );
}

#[tokio::test]
async fn allowed_wasip2_tcp_connect_succeeds_control() {
    let engine = Engine::default();
    let component = Component::from_file(&engine, COMPONENT).unwrap();
    let (_listener, port) = bind_probe_listener().unwrap();

    let stdout = MemoryOutputPipe::new(4096);
    let mut builder = WasiCtxBuilder::new();
    builder.arg("network_probe_p2");
    builder.arg(port.to_string());
    builder.stdout(stdout.clone());
    builder.allow_tcp(true);
    builder.inherit_network();

    let mut linker = Linker::new(&engine);
    wasmtime_wasi::p2::add_to_linker_async(&mut linker).unwrap();
    let mut store = Store::new(
        &engine,
        P2State {
            wasi: builder.build(),
            table: ResourceTable::new(),
        },
    );
    let command = Command::instantiate_async(&mut store, &component, &linker)
        .await
        .unwrap();
    let result = command.wasi_cli_run().call_run(&mut store).await.unwrap();
    assert!(result.is_ok());

    let output = String::from_utf8_lossy(&stdout.contents()).into_owned();
    println!("p2 control evidence: {output:?}");
    assert!(
        output.contains("P2_CONNECT_OK"),
        "control guest must connect when policy permits it: {output}"
    );
}
