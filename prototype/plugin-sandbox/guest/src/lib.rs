//! Shared host probes for the throwaway adversarial fixtures.
//!
//! The host exports `genesis_probe.report(i32)` and `genesis_probe.port()`;
//! fixtures use them to report observations and to obtain a real, live host
//! listener port, without relying on inherited stdio.

#[link(wasm_import_module = "genesis_probe")]
extern "C" {
    #[link_name = "report"]
    fn host_report(value: i32);
    #[link_name = "port"]
    fn host_port() -> i32;
}

/// Send one integer observation to the host.
pub fn report(value: i32) {
    unsafe { host_report(value) }
}

/// Return the live host listener port supplied by the host (0 if none).
pub fn probe_port() -> i32 {
    unsafe { host_port() }
}
