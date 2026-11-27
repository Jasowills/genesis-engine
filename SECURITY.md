# Security Policy

## Supported versions

No stable release exists yet. The approved slice is complete and independently reviewed (30 of 32 tickets verified); signed releases were declined by the owner and none has been published. Once releases begin, supported versions will be listed here with dates.

## Reporting a vulnerability

**Do not open a public GitHub issue for a security vulnerability.** Report it privately to the repository owner (`Jasowills` on GitHub) or via GitHub's private vulnerability reporting when enabled. Include: affected component/version, reproduction steps, impact, and any suggested mitigation. You should receive an acknowledgement; we will coordinate disclosure after a fix is available.

Never include real credentials, tokens or personal data in a report or in any commit, ticket or log.

## Scope

- Genesis Core and Studio code, project-file parsing, archive extraction, model loading, worker process execution, plugin runtime, and AI/data adapters.
- Supply-chain integrity of dependencies, lockfiles, release artefacts and update signatures.

## Known design constraints (see `engineering/SECURITY_MODEL.md`)

- Untrusted plugins run in a deny-by-default Wasmtime/WASI sandbox with explicit capability grants; no `inherit_env`/`inherit_stdio`.
- Archives must be path-contained and size/ratio-quota bounded; untrusted model files are never unpickled in-process; external workers use argv-only execution with scrubbed environments.
- Downloaded metadata and dataset descriptions are treated as untrusted data by any AI agent (OWASP LLM01:2025).
- Google Maps/Street View/Photorealistic 3D Tiles and Cesium ion content are denied as reconstruction inputs by terms of service, enforced in code.

## Recognition

Reporters may be credited in the changelog with their consent.
