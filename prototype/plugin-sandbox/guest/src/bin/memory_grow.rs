//! Evil fixture: requests 1024 pages (64 MiB) in one `memory.grow`.
//! The host cap is 8 MiB, so growth must be denied (or trap when configured).
fn main() {
    let grown = core::arch::wasm32::memory_grow(0, 1024);
    ge_plugin_sandbox_guest_fixtures::report(grown as i32);
    ge_plugin_sandbox_guest_fixtures::report(core::arch::wasm32::memory_size(0) as i32);
}
