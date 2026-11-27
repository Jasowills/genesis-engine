//! Direct WASI Preview 1 socket-import probe. Unlike Rust std (which has no
//! sockets on wasm32-wasip1), this declares the host import explicitly, so a
//! host-side decision can be observed rather than assumed.

use ge_plugin_sandbox_guest_fixtures::{probe_port, report};

#[link(wasm_import_module = "wasi_snapshot_preview1")]
extern "C" {
    #[link_name = "sock_connect"]
    fn wasi_sock_connect(fd: u32, address: *const u8, length: u32) -> u32;
}

fn main() {
    let mut address = [0_u8; 16];
    address[0] = 1; // AF_INET
    let port = probe_port().max(0) as u16;
    address[2..4].copy_from_slice(&port.to_le_bytes());
    address[4..8].copy_from_slice(&[127, 0, 0, 1]);
    let errno = unsafe { wasi_sock_connect(0, address.as_ptr(), address.len() as u32) };
    report(70 + errno as i32);
}
