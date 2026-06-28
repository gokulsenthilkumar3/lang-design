# RFC-0010: Stability and Compatibility

**Status:** Draft\
**Author:** Gokul Senthilkumar\
**Created:** 2026-06-27\
**Updated:** 2026-06-27

---

## Summary

This RFC defines the stability and compatibility policy for the language
platform. It covers ABI expectations, reproducible builds, forward-compatibility
rules, and the testing discipline required to keep the toolchain trustworthy as
the design matures into implementation.

---

## Motivation

The design repository is already shaping the core language and toolchain. The
next risk is not inventing more syntax; it is accidentally creating a future
implementation that changes meaning too often, cannot be reproduced, or has no
clear compatibility contract. This RFC gives the project a release-minded
stability policy before implementation starts.

---

## Detailed Design

### Core Rules

- Language semantics must not change silently once accepted.
- Compatibility boundaries must be documented before they are relied upon.
- Reproducible builds are a requirement, not an optional optimization.
- ABI promises should be explicit and versioned.
- Tests and benchmarks must gate changes that affect compatibility.

### Syntax

```lang
version "0.1"

compatibility {
    language = "preview"
    abi = "unstable"
    reproducible_builds = true
}
```

### Semantics

#### Language Stability

- Accepted RFCs define the compatibility contract for future implementation.
- A semantic change that breaks accepted meaning requires a new RFC.
- Syntax changes that alter parsing or diagnostics should be treated as
  compatibility-relevant.

#### ABI

- ABI promises are versioned separately from source-level semantics.
- The initial ABI may be unstable, but instability must be explicit.
- Public runtime interfaces should carry compatibility notes.

#### Reproducibility

- Build outputs should be reproducible for the same source, manifest, and
  lockfile inputs.
- The toolchain should expose enough metadata for independent verification.
- Non-deterministic sources of variation should be minimized or isolated.

#### Testing Discipline

- Core RFC decisions must be backed by focused tests or benchmarks where
  possible.
- Regression tests should cover semantics, diagnostics, parser recovery, and
  toolchain behavior.
- Compatibility-sensitive changes should require explicit review.

### Type System Impact

- Stability guarantees extend to type-checking behavior and diagnostics
  semantics.
- Changes that alter inference results or error categorization should be
  considered compatibility-impacting.

### Toolchain Impact

- The compiler must surface version and compatibility metadata.
- Build outputs should be accompanied by reproducibility data where relevant.
- Release tooling should reject changes that break documented stability rules.

---

## Benchmark Targets

| Metric | Target | Rationale |
| --- | --- | --- |
| Reproducible build: bit-identical output | 100% | Independent rebuilds must match |
| Compiler test coverage | > 95% line coverage | High confidence in compatibility-sensitive code |
| Stdlib test coverage | > 95% line coverage | Stable runtime behavior needs broad coverage |
| Compiler fuzzing: crashes | 0 after 10B inputs | Fuzzing is a hardening gate |
| CVE response time | < 14 days | Security response must be timely |

---

## Innovation Backlog

| Option | Description | Tradeoff |
| --- | --- | --- |
| No explicit compatibility policy | Let stability emerge organically | Fast to start, risky and confusing later |
| Immediate stable ABI guarantee | Promise ABI stability from day one | Clear, but unrealistic before implementation matures |
| **Chosen** | Versioned stability policy with explicit compatibility boundaries | Matches the project’s staged roadmap |

---

## Drawbacks

- Stability rules add process overhead.
- Some implementation experiments will need to remain private or temporary.
- Compatibility language can slow change if used too early without nuance.

---

## Alternatives Considered

**No stability RFC.** Rejected because release behavior would otherwise be
handled ad hoc.

**Freeze everything now.** Rejected because the language is still in the design
phase and needs room to converge.

**Treat ABI as the only compatibility concern.** Rejected because source-level
semantics and diagnostics also matter.

---

## Decisions

The following open questions from the original draft have been resolved:

### D1 — Stable ABI Timeline

**Decision:** The first stable ABI will be declared post-v1.0. The v1.0 ABI is explicitly unstable and tied to the compiler version.
**Rationale:** A stable ABI restricts compiler optimizations (like struct field reordering) too early in the language's life. C-FFI (`#[repr(C)]`) serves as the stable bridge for interoperability.

### D2 — Diagnostic Contracts

**Decision:** Error codes are contractual; diagnostic prose is best-effort.
**Rationale:** External tools can rely on an error code (e.g., `E0123`) always representing the same semantic violation. The exact wording and suggestions can evolve without being considered a breaking change.

### D3 — Reproducibility Metadata

**Decision:** Reproducibility metadata is exposed via a `build-info.json` artefact.
**Rationale:** Generated alongside the binary, this file details the exact compiler version, lockfile hash, and target triple used to produce the output for verification.

### D4 — Compatibility Review Triggers

**Decision:** Any change that causes previously compiling safe code to stop compiling requires a compatibility review.
**Rationale:** Adding new syntax or relaxing restrictions is a routine RFC update. Breaking existing valid code requires a deprecation period and formal compatibility review.

---

## Open Questions

All open questions have been resolved in the Decisions section above.

---

## Success Metric

This RFC is successful if future implementation can evolve without surprising
users, if build outputs are reproducible, and if stability expectations are
visible enough to guide both contributors and reviewers.
