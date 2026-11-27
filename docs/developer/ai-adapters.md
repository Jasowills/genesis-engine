# AI segmentation and depth adapters (GE-0026)

Domain code depends on the typed `InferenceAdapter` port, never on an inference runtime. Requests and results carry explicit pixel dimensions and source object IDs; every output includes `InferenceEvidence` with model id, version, licence, adapter name, parameters, completion state and limitations. Outputs can therefore never be silently attributed to an unnamed model.

- `FakeAdapter` is a deterministic in-process implementation exercising the contract, cancellation and evidence. It performs **no inference**.
- `ExternalProviderStub` refuses all calls unless a consent token is supplied; even then it returns `transport not implemented`. There is no network client in Core and no paid call by default.
- `ModelDescriptor::validate` records separate code and weight licences, rejects non-commercial licences on either, checks a vendor-neutral execution-provider priority list and adapter settings, and rejects missing identity or malformed expected weight digests. No weight bytes are loaded, deserialised or executed.

## Scope limit

Canonical `CoreError` variants let consumers discriminate cancellation (`Cancelled`), licence refusal (`LicenseBlocked`) and provider unavailability (`ProviderUnavailable`) without string matching. `InferenceEvidence` is a reduced evidence shape: input/output checksums, runtime/execution provider, precision and a provenance chain to source imagery are follow-up fields.

Named models (SAM ViT-B quantised encoder, YOLOX-S, Depth Anything V2 Small) are owner-approved (D-033), checksum-verified and run in the GE-0006 evidence prototype on ORT 1.23.2. Core itself still performs **no** inference; real ONNX inference inside Core, weight checksum verification against a loaded artefact, GPU execution providers and external provider transport are follow-up work. No non-commercial weight is used. Do not describe this slice as working model inference.

## Verification

`cargo test --workspace --locked` includes seven AI adapter tests (including denied-source gating and duplicate licence checks): typed outputs with per-run evidence, cancellation on both ports, non-commercial weight rejection, external provider disabled by default, and invalid request/descriptor rejection.
