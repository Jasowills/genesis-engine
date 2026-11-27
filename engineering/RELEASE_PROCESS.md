# Genesis Engine — Release Process (PROPOSED)

**Date:** 2026-11-27. No release has occurred. No signing identity, repository, domain or paid account has been acquired. All purchases require explicit owner approval.

## 1. Release stages (spec §52)

Research/architecture prototype → core data/geometry foundation → first E2E vertical slice → early developer preview → public alpha → public beta → stable. Roadmap milestones M0–M5 in `ROADMAP.md` map to these stages.

## 2. Per-release gate

Each release must publish: scope and non-goals; supported OS/platform matrix; known limitations; installation and migration notes; test evidence; security/licence review; reproducible build information where practical. No stable support claim for untested features. A ticket is never released on "code written" alone — every code-changing ticket must be `verified` with independent review (spec §39).

## 3. Build tracks

1. **Reproducible unsigned core/CLI** — `SOURCE_DATE_EPOCH`, `--locked`, two identical builds compared by hash.
2. **Signed installers** — Tauri bundler with mandatory signature verification; macOS Developer ID + notarisation (hardened runtime, entitlements designed early because plugins/JIT/library validation are affected); Windows Store MSIX re-signing or Azure Artifact Signing / OV / EV certificate; Linux AppImage/deb/RPM plus updater endpoint.

Signed/notarised artefacts are not byte-identical — track provenance separately.

## 4. Key custody and cost (verified)

- Tauri updater private key loss blocks all future updates; design custody and rollback before first release.
- Apple Developer Program **$99/membership year**; notarisation required for Developer-ID software built after 2019-06-01.
- Windows: Store developer account free; Azure Artifact Signing ~**$9.99/mo** (regional eligibility limits); OV **$150–300/yr**; EV **$400+/yr** (no longer bypasses SmartScreen since 2024).
- GitHub: public standard runners free; private $0.006 Linux / $0.010 Windows / $0.062 macOS per minute; GPU larger runners Team/Enterprise only. Budget alerts at 90%/100%.

## 5. CI obligations

Native runners for macOS/Windows signing; Linux for high-volume tests; ARM64 where available (macOS ARM64 runners have no static UDID and no nested virtualisation). Pin Actions to full SHAs. Commit lockfiles; build with `--locked`. Secret scanning + push protection. Publish SBOM and attestations where supported. Maintain artifact retention short and fixtures in LFS/object storage (artifacts $0.25/GB-month beyond allowance; cache $0.07/GB-month).

## 6. Licence publication gate (blocks release)

- Final core / plugin SDK / docs / fixtures / sample assets / model weights licences approved by owner (D-014).
- Dependency licence audit completed, including GDAL binary dependency mix (`gdal --license`), GEOS dynamic-linking posture, AGPL/MPL/GPL workers kept external, and no non-commercial AI weights in the default path.
- OSM attribution visible in Studio and export; decision recorded on whether OSM-derived structured data is published and, if so, ODbL §4.6 machine-readable offer/alteration file.
- No secrets, large model weights, restricted datasets, private imagery or generated city-scale output in the repository.

## 7. Release checklist (to be expanded per milestone)

- [ ] All tickets in the milestone are `verified` with independent reviewer approval
- [ ] Full test run recorded with commands, OS, tool versions and results
- [ ] Cross-platform matrix evidence present (no local-only claim)
- [ ] Known limitations documented, including no survey-grade claim without checkpoints
- [ ] Security model and licence audit reviewed
- [ ] Reproducible core build hash compared across two builds
- [ ] Signing identities and updater keys available; no secrets in repo or logs
- [ ] Owner approval to publish recorded

## 8. Post-deployment verification

Installer smoke test on each supported OS; updater dry run; capability report correctness; crash/error telemetry (local-only, opt-in); documented rollback path. Continuous post-release probes are a separate later practice, not a release gate.
