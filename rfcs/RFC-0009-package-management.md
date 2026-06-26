# RFC-0009: Package Management

**Status:** Draft\
**Author:** Gokul Senthilkumar\
**Created:** 2026-06-27\
**Updated:** 2026-06-27

---

## Summary

This RFC defines the package-management model for the language: package
identity, versioning, dependency resolution, lockfiles, publication, and
integrity metadata. It is designed to work as part of the unified toolchain,
not as a separate ecosystem bolted onto the compiler after the fact.

---

## Motivation

The unified toolchain only works if project dependencies are reproducible and
easy to manage. The language needs a clear story for local projects, published
packages, and locked builds. Without that, users would still need a separate
ecosystem layer to make the toolchain practical.

---

## Detailed Design

### Core Rules

- Every package has a stable name and version.
- Dependency resolution must be deterministic.
- Lockfiles pin the exact resolved dependency graph.
- Published packages must include enough metadata for reproducible builds.
- Package integrity should be verifiable.

### Syntax

```lang
project app {
    name = "app"
    version = "0.1.0"

    dependency std {
        version = "^0.1"
    }

    dependency serde {
        version = "^1.2"
        source = "registry"
    }
}
```

### Semantics

#### Package Identity

- Package identity is defined by name plus version.
- Versions follow semantic versioning-style expectations unless a package
  declares a different compatibility policy.
- The package manager must reject ambiguous identity collisions.

#### Resolution

- Resolution must be reproducible across machines.
- The lockfile captures the resolved graph used for builds and tests.
- Resolution should prefer exact matches when a lockfile is present.

#### Publication

- Publishing requires explicit metadata, version, and integrity information.
- The package manager should not publish an incomplete package definition.
- Published artefacts should be traceable back to their source manifest.

#### Integrity and Trust

- Packages should carry integrity metadata suitable for verification.
- The registry may enforce signing or checksum policies.
- The toolchain must be able to distinguish trusted and untrusted sources.

### Type System Impact

- Package manifests should be parsed into strongly typed configuration data.
- Dependency metadata must remain consistent with module and import semantics.

### Toolchain Impact

- Build commands must use the same package resolution logic as publish/install
  commands.
- Formatter and LSP workflows must respect the resolved project manifest.
- Error messages must identify the package and version involved in resolution
  failures.

---

## Benchmark Targets

| Metric | Target | Rationale |
| --- | --- | --- |
| Package registry: publish time | < 30 seconds | Publishing should feel routine |
| Package registry: install (cached) | < 1 second | Cached dependency reuse should be instant |
| Dependency resolution determinism | 100% | Same inputs should produce the same graph |
| Lockfile reproducibility | 100% | Builds must be pinned and repeatable |
| Package publish validation | 100% of required metadata present | Incomplete packages should be rejected |

---

## Innovation Backlog

| Option | Description | Tradeoff |
| --- | --- | --- |
| Registry-free source discovery | Depend on ad hoc git/path sources only | Simple early, poor ecosystem cohesion |
| Flexible package identity | Allow many aliases and identity forms | Easier migration, harder reproducibility |
| **Chosen** | Versioned packages with deterministic lockfiles | Best supports reproducible builds and trust |

---

## Drawbacks

- Package tooling must balance flexibility with reproducibility.
- Registry and lockfile design can become complex quickly.
- Trust and integrity features add more metadata to the ecosystem model.

---

## Alternatives Considered

**Ad hoc dependency references.** Rejected because they undermine reproducible
builds.

**Registry-only with no local packages.** Rejected because local iteration is a
core part of the development workflow.

**No lockfile.** Rejected because deterministic builds are a product
requirement.

---

## Open Questions

- Should package signing be mandatory for publish or optional for v1?
- How should local path dependencies interact with lockfiles?
- What compatibility policy should the registry assume by default?
- Should package metadata include minimum supported compiler version?

---

## Success Metric

This RFC is successful if users can publish, install, and build packages with a
deterministic dependency graph and without needing separate ecosystem tools.

