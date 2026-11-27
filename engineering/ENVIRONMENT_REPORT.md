# Genesis Engine — Environment Report

**Status:** Verified by direct inspection on 2026-11-27 (not inferred from the specification).
**Purpose:** Phase 0 working-state record. All values below were read from the local machine in this session.

## 1. Working directory and repository state

| Item | Verified observation |
| --- | --- |
| Absolute path | `/Users/jasonamadi/genesis-engine` |
| Directory contents at first inspection | `GENESIS_ENGINE_SPEC.md` only (71,724 bytes, 2,015 lines) |
| Git repository | **No.** `git rev-parse --is-inside-work-tree` returned `fatal: not a git repository`; no `.git` directory. |
| Branch / remotes / uncommitted changes | Not applicable — no Git history exists to overwrite. |
| Genesis-named repositories under authenticated GitHub account | None found in the first 100 repos of `Jasowills` (read-only query; no repo created). |

Nothing was deleted or overwritten at inspection time. No `git init`, repository creation or push had been performed then. The owner later approved a public `genesis-engine` repository (D-027, 2026-11-27); GitHub setup is now the current Phase 7 action, still subject to the approval checklist in `DECISIONS.md` §3a and independent review.

## 2. Toolchains present (versions printed by the tools)

| Tool | Version |
| --- | --- |
| git | 2.50.1 (Apple Git-155), `/usr/bin/git` |
| gh | 2.97.0 (2026-07-31), `/usr/local/bin/gh` |
| rustc | 1.98.1 (48a229cea 2026-09-01) |
| cargo | 1.98.1 (797e8a9bc 2026-08-05) |
| rustup | 1.29.1 (d95a37b6a 2026-08-13) |
| Node.js | v24.16.0 |
| npm | 11.18.0 |
| pnpm | 11.12.0 |
| bun | 1.4.0 |
| yarn | 4.13.0 |
| Python | 3.15.2 (`/usr/local/.../python3`) |
| pip | 26.2.2 |
| Go | go1.27.1 darwin/amd64 |
| CMake | 4.4.4 |
| Ninja | 1.13.2 |
| Docker | 29.8.2, build 7fc2dff |

Not installed: `uv`, `conda`. Installed Rust targets: `x86_64-apple-darwin` only (no `aarch64-apple-darwin`, no `wasm32-*`).

## 3. Host platform and hardware

| Item | Verified observation |
| --- | --- |
| OS | macOS 26.5.2, build 25F84 |
| Architecture | `x86_64` (Intel) |
| CPU | Intel Core i7-9750H @ 2.60 GHz, 12 logical cores |
| Memory | 17,179,869,184 bytes (16 GiB) |
| Disk (`/`) | 466 GiB total, 12 GiB used, 202 GiB available, 6% used |
| GPUs | Intel UHD Graphics 630 (up to 1,536 MB dynamic) and AMD Radeon Pro 5300M (4 GB) |
| Metal SDK | 26.5 (`xcrun --sdk macosx --show-sdk-version`) |
| NVIDIA / CUDA | None detected locally |

## 4. GitHub access

`gh auth status` reported: logged in to github.com as **`Jasowills`** (keyring), active account, HTTPS protocol. The token's numeric value is deliberately **not** recorded here. Granted scopes observed: `delete_repo`, `gist`, `read:org`, `repo`, `workflow`.

**Security note:** the `delete_repo` scope is powerful. No repository creation, deletion, setting change, or push will be attempted without explicit owner approval.

## 5. Environment limitations that affect the plan

1. **Single-platform verification.** This host can only run macOS/x86_64 natively. Windows, Linux, ARM64, Apple Silicon and CUDA paths cannot be verified on this machine. They must be covered by CI runners, containers or a future owner-provided machine — and must not be advertised as tested until they are.
2. **No local WASM target.** The Wasmtime/plugin feasibility question needs `wasm32-wasip1` (or equivalent) installed before any proof of concept; currently only the native target exists.
3. **GPU backends unverified.** Metal on Apple Silicon, CUDA, Vulkan and DirectX are untested here. Local GPU is integrated Intel + AMD; driver/backend capability detection must be designed for graceful degradation.
4. **Python environment.** No `uv` or `conda`; use `python3 -m venv` + `pip` if a Python worker is approved.
5. **Skill provenance.** The installed skills directory is not a Git checkout, but provenance was verified from the official plugins marketplace: upstream `mattpocock/skills.git`, pinned SHA `0ab1b63a410a03d3627979a109c8695de27af954`, MIT, with the local `to-tickets/SKILL.md` hash matching upstream exactly. See `engineering/WORKFLOW.md`.
6. **Memory budget.** 16 GiB on this host constrains any local reconstruction/AI proof of concept; large-area benchmarks must be deferred to CI or a bigger machine.

## 6. What has NOT happened yet

As of this report no product code, crates, UI, datasets or CI configuration exist. Research is complete (`research/RESEARCH_REGISTER.md`). Additional owner decisions were recorded on 2026-11-27: seven accepted ADRs (D-025), Apache-2.0 **core** licence (D-026), public repository (D-027) and hard source deny (D-028). Git history and issues are created only by the approved GitHub setup. SDK/docs/fixture/model-weight licences, hardware promises and first-slice scope remain open.
