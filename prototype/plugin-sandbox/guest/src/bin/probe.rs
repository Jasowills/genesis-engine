//! Capability probe fixture: checks env vars, preopens, TCP and DNS, then
//! writes sentinel text to stdout/stderr (which the host deliberately discards).
use ge_plugin_sandbox_guest_fixtures::{probe_port, report};
use std::io::Write;
use std::net::{TcpStream, ToSocketAddrs};

fn main() {
    // 10 + number of inherited env vars (capped at 9); expect 10 => zero env.
    let env_count = std::env::vars().count().min(9);
    report(10 + env_count as i32);

    // 20 => no preopened root; 21 => root unexpectedly readable.
    report(if std::fs::read_dir("/").is_err() { 20 } else { 21 });
    // 30 => no preopened current directory; 31 => cwd unexpectedly readable.
    report(if std::fs::read_dir(".").is_err() { 30 } else { 31 });

    // The host binds a real loopback listener on an ephemeral port and passes
    // it here, so failure cannot be blamed on "no listener". 39 => no port
    // supplied (setup problem), 40 => policy denied, 41 => connect succeeded.
    let port = probe_port();
    if port <= 0 {
        report(39);
    } else {
        report(
            if TcpStream::connect(("127.0.0.1", port as u16)).is_ok() {
                41
            } else {
                40
            },
        );
    }

    // 50 => name lookup failed/empty, 51 => resolved. Note: WASI preview1 has
    // no standard DNS interface, so the meaning of this result is qualified in
    // RESULTS.md rather than asserted as a policy guarantee.
    report(
        if ("localhost", 0)
            .to_socket_addrs()
            .is_ok_and(|mut addrs| addrs.next().is_some())
        {
            51
        } else {
            50
        },
    );

    let _ = std::io::stdout().write_all(b"GE_SENTINEL_STDOUT\n");
    let _ = std::io::stderr().write_all(b"GE_SENTINEL_STDERR\n");
}
