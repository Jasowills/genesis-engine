//! GE-0004 throwaway capability probe — native GPU evidence only.
//!
//! SCOPE / IMPORTANT DISTINCTION
//! -----------------------------
//! This binary enumerates NATIVE GPU adapters through the `wgpu` crate. It is
//! evidence about the host's native graphics stack (Metal / Vulkan / DX12 /
//! OpenGL as exposed by wgpu). It is NOT evidence about WebGL2 or WebGPU
//! availability inside a webview (WebView2, WKWebView, WebKitGTK) or inside
//! Electron's bundled Chromium. Those remain UNVERIFIED; see RESULTS.md.
//!
//! No device, queue, surface, pipeline or other GPU resource is created: only
//! an `Instance` plus adapter enumeration. If no adapter exists the probe
//! still exits 0 and emits a structured report with an empty adapter list.

use std::env;

/// Minimal JSON string escaping so the report stays machine-readable without
/// pulling in serde/serde_json as extra dependencies.
fn json_str(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

fn main() {
    // Compile-time-only information: which backends wgpu was built with, and
    // exactly which wgpu version Cargo resolved (from build.rs + Cargo.lock).
    let compiled = wgpu::Instance::enabled_backend_features();
    let compiled_backends_str = format!("{:?}", compiled);
    let wgpu_version = env!("PROBE_WGPU_VERSION");
    let requested = wgpu::Backends::all();

    let mut lines: Vec<String> = Vec::new();
    lines.push("{".to_string());
    lines.push(format!("  \"ticket\": {},", json_str("GE-0004")));
    lines.push(format!(
        "  \"scope\": {},",
        json_str("native wgpu adapter enumeration only; NOT webview WebGL2/WebGPU evidence")
    ));
    lines.push(format!("  \"wgpu_version\": {},", json_str(wgpu_version)));
    lines.push(format!("  \"os\": {},", json_str(env::consts::OS)));
    lines.push(format!("  \"arch\": {},", json_str(env::consts::ARCH)));
    lines.push(format!("  \"family\": {},", json_str(env::consts::FAMILY)));
    lines.push(format!(
        "  \"compiled_backends\": {},",
        json_str(&compiled_backends_str)
    ));
    lines.push(format!(
        "  \"requested_backends\": {},",
        json_str(&format!("{:?}", requested))
    ));

    if compiled.is_empty() {
        // Nothing usable was compiled in; report rather than let
        // Instance::new() panic. Still exit 0 for CI robustness.
        lines.push("  \"adapter_count\": 0,".to_string());
        lines.push("  \"status\": \"no_backend_compiled\",".to_string());
        lines.push("  \"adapters\": [],".to_string());
        lines.push(
            "  \"note\": \"No wgpu backend compiled for this target; native GPU capability UNVERIFIED.\""
                .to_string(),
        );
        lines.push("}".to_string());
        println!("{}", lines.join("\n"));
        return;
    }

    // Only an Instance is created. No device/queue/surface is requested.
    let mut desc = wgpu::InstanceDescriptor::new_without_display_handle();
    desc.backends = requested;
    let instance = wgpu::Instance::new(desc);

    // wgpu 30 made adapter enumeration async. On native backends it resolves
    // immediately; pollster::block_on is the tiny standard executor.
    let adapters = pollster::block_on(instance.enumerate_adapters(requested));

    lines.push(format!("  \"adapter_count\": {},", adapters.len()));
    lines.push(format!(
        "  \"status\": {},",
        json_str(if adapters.is_empty() {
            "no_adapter"
        } else {
            "ok"
        })
    ));

    lines.push("  \"adapters\": [".to_string());
    for (i, a) in adapters.iter().enumerate() {
        let info = a.get_info();
        let features = a.features();
        let limits = a.limits();
        lines.push("    {".to_string());
        lines.push(format!(
            "      \"index\": {}, \"backend\": {}, \"device_type\": {:?},",
            i,
            json_str(info.backend.to_str()),
            info.device_type
        ));
        lines.push(format!(
            "      \"name\": {}, \"driver\": {}, \"driver_info\": {},",
            json_str(&info.name),
            json_str(&info.driver),
            json_str(&info.driver_info)
        ));
        lines.push(format!(
            "      \"vendor\": {}, \"device\": {}, \"device_pci_bus_id\": {}, \"subgroup_min_size\": {}, \"subgroup_max_size\": {},",
            json_str(&format!("0x{:x}", info.vendor)),
            json_str(&format!("0x{:x}", info.device)),
            json_str(&info.device_pci_bus_id),
            info.subgroup_min_size,
            info.subgroup_max_size
        ));
        // Exact enabled feature names as reported by this wgpu build (avoids
        // hardcoding variant names that change across wgpu releases).
        lines.push(format!(
            "      \"features\": {},",
            json_str(&format!("{:?}", features))
        ));
        lines.push(format!(
            "      \"limits\": {{\"max_texture_dimension_2d\": {}, \"max_buffer_size\": {}, \"max_storage_buffer_binding_size\": {}, \"max_bind_groups\": {}, \"max_compute_workgroups_per_dimension\": {}}}",
            limits.max_texture_dimension_2d,
            limits.max_buffer_size,
            limits.max_storage_buffer_binding_size,
            limits.max_bind_groups,
            limits.max_compute_workgroups_per_dimension
        ));
        let comma = if i + 1 == adapters.len() { "" } else { "," };
        lines.push(format!("    }}{}", comma));
    }
    lines.push("  ]".to_string());
    if adapters.is_empty() {
        lines.push(
            "  \"note\": \"No adapter: graceful degradation. Native GPU capability is UNVERIFIED on this host/runner.\""
                .to_string(),
        );
    }
    lines.push("}".to_string());

    println!("{}", lines.join("\n"));
}
