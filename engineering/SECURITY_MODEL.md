# Genesis Engine — Security and Privacy Model (planned)

**Date:** 2026-11-27. **Evidence base:** `research/notes/plugin-security.md`, `research/notes/competitive-data-rights.md`, `research/notes/geospatial.md`, spec §34. Threat model for design; no security controls have been implemented yet.

## 1. Threat register and required mitigations

| Threat | Evidence | Required mitigation | Confidence |
| --- | --- | --- | --- |
| Malicious plugin / host compromise or DoS | WASI has no ambient authority; Wasmtime `WasiCtxBuilder` defaults to closed stdin, discarded stdio, no env/args/preopens. Recent Windows sandbox escape (GHSA-f2pq-268w-886c, fixed 48.0.10/49.0.7/50.0.3) and host-memory issues fixed in 51.0.0. | Require Wasmtime **≥51.0.0**; build `WasiCtx` explicitly, never `inherit_env`/`inherit_stdio`; link only needed WIT interfaces; per-invocation memory/table/instance caps (`StoreLimitsBuilder::memory_size` has **no default**); epoch interruption + async timeout, not fuel as default; advisory monitoring in CI; evil-plugin fixtures. | High |
| Malicious project file / zip-slip / path traversal | CWE-22; Python tarfile docs warn no filter blocks all dangerous features. | Resolve entries under a canonical root; reject absolute paths, `..`, Windows drive prefixes (`C:foo`), symlinks/hardlinks/devices by default; cap entry count/total size/ratio; extract to temp and atomically promote only after validation. | High |
| Decompression bomb | CWE-409. | Stream with compressed/uncompressed byte caps, entry count, ratio, per-entry output; abort and delete temp on breach. Thresholds to be calibrated. | High |
| Malicious model file (pickle / ONNX external data) | Python pickle docs: "not secure"; PyTorch `weights_only=True` default since 2.6 narrows RCE but is not complete. ONNX assurance case says reference implementation not safe for untrusted input; `base_dir=""` skips containment checks. | Never unpickle untrusted files in Studio; `weights_only=True`, no unsafe globals, size/alloc caps, sandboxed worker; prefer safetensors/ONNX with verified provenance; always set ONNX `base_dir`, canonicalize/contain external data, reject symlinks/hardlinks/absolute paths, `O_NOFOLLOW` where available, `onnx.checker.check_model`. | High |
| External worker execution | Windows Job Objects and Linux seccomp verified; seccomp "isn't a sandbox". | argv-only execution (never a shell), scrubbed environment, restricted cwd, per-job CPU/memory/time caps, kill whole process tree on timeout, least-privilege user. | Medium-high (macOS docs thin) |
| Untrusted downloaded datasets / models | CWE-22/409; npm provenance does not guarantee absence of malicious code. | Download → verify digest/signature/attestation → quarantine → hardened extract → validate → import; manifest with digest, size, source URL, licence, attestation. | Medium-high |
| Credential leakage | Wasmtime defaults no env/inherited stdio; GitHub secret scanning covers history, issues, PRs, discussions, wikis, gists. | No secrets in source, prompts, logs, tickets, commits; enable secret scanning + push protection; do not call `inherit_env`; scoped short-lived tokens via host WIT only when necessary; log-redaction tests; rotate on alert. | High |
| Remote worker compromise | OS limit/kill primitives and attestations exist; no complete remote-worker threat model in sources. | Treat remote worker as hostile: authenticated channel, allowlisted egress, no long-lived credentials in env, per-job isolation, audit of inputs/outputs, verified binary attestations (design inference). | Medium |
| Supply-chain attack | `Cargo.lock` records exact versions; npm provenance via Sigstore; SLSA v1.0 retired, v1.2 current; `gh attestation verify` exists. | Commit lockfiles, `--locked`, pin git deps by full SHA, pin Actions to SHAs, SBOM, provenance/attestation for released plugins, 2FA/trusted publishing; cargo provenance status to verify. | High |
| Prompt injection / hostile metadata in AI agents | OWASP LLM01:2025 (indirect injection via external files), LLM03/04/06/10. | Treat all downloaded metadata/content as untrusted data, not instructions; segregate in prompts; deterministic schema validation of model outputs; privilege control in code; human approval for privileged actions; cap tokens/tool calls/cost; adversarial corpus. | High |
| Data rights violations | Google Maps ToS §3.2.3(a)–(c) and Cesium ion ToS prohibit derivation, offline use and AI training; ODbL §4.4/§4.6. | Lawful-source adapter boundary: reject Google Maps/Street View/3D Tiles and Cesium ion content as reconstruction inputs; carry attribution + licence election + derivative status in provenance; legal review of ODbL treatment of generated project files. | High |

## 2. Non-negotiable rules

1. Never print or persist tokens, keys or credentials.
2. Never call `inherit_env` or `inherit_stdio` on a plugin.
3. Never invoke a shell for a worker; argv arrays only.
4. Never derive or train from content whose terms prohibit it (hard-coded source policy, not user discretion alone).
5. Never mark a ticket verified without an independent reviewer (spec §39).

## 3. Open items before implementation

- Verify exact supported Wasmtime LTS versions and resolve the WASI TCP/UDP default documentation conflict defensively.
- Retrieve macOS sandbox primary docs and prototype per-OS worker profiles.
- Verify cargo/crates.io provenance support and `cargo audit`/`cargo deny` primary docs.
- Confirm thresholds for archive quotas from real project sizes.
- Complete OWASP Agentic Top 10 2026 review.
- Legal review: ODbL §4.4/§4.6 applied to generated project files and exported scenes; AGPL tools as external processes; OpenMVS IBFS research-only component; vendor EULAs.
