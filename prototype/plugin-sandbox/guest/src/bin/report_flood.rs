//! Hostcall-abuse fixture: sends 100 reports; host must retain at most 64.
fn main() {
    for value in 0..100 {
        ge_plugin_sandbox_guest_fixtures::report(value);
    }
}
