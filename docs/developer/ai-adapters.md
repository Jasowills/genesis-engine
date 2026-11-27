# AI segmentation and depth adapters (GE-0026)

Domain code depends on the typed `InferenceAdapter` port, never on an inference runtime. Requests and results carry explicit pixel dimensions and source object IDs; every output includes `InferenceEvidence` with model id, version, licence, adapter name, parameters, completion state and limitations. Outputs can therefore never be silently attributed to an unnamed model.

- `FakeAdapter` is a deterministic in-process implementation exercising the contract, cancellation and evidence. It performs **no inference**.
- `ExternalProviderStub` refuses all calls unless a consent token is supplied; even then it returns `transport not implemented`. There is no network client in Core and no paid call by default.
- `ModelDescriptor::validate` rejects non-commercial licences, missing identity and malformed expected weight digests. No weight bytes are loaded, deserialised or executed.

## Scope limit

Canonical `CoreError` variants let consumers discriminate cancellation (`Cancelled`), licence refusal (`LicenseBlocked`) and provider unavailability (`ProviderUnavailable`) without string matching. `InferenceEvidence` is a reduced evidence shape: input/output checksums, runtime/execution provider, precision and a provenance chain to source imagery are follow-up fields.

Named models (SAM ViT-B, YOLOX, Depth Anything V2 Small) are **not** loaded; this requires an owner cost/licence decision (GE-0006, D-022-adjacent). Real ONNX inference, weight checksum verification against a loaded artefact, GPU execution providers and external provider transport are follow-up work. No non-commercial weight is used. Do not describe this slice as working model inference.

## Verification

`cargo test --workspace --locked` includes five AI adapter tests: typed outputs with per-run evidence, cancellation on both ports, non-commercial weight rejection, external provider disabled by default, and invalid request/descriptor rejection.
