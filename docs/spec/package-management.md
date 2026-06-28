# Package Management Spec Notes

**Status:** Draft  
**Purpose:** Companion specification for RFC-0009

---

## Scope

This note captures implementation-facing expectations for package resolution,
lockfiles, and publication.

## Core Expectations

- Package identity should be explicit.
- Resolution should be deterministic.
- Lockfiles should pin the dependency graph.
- Publish workflows should validate metadata before release.

## Canonical Examples

```lang
project sample {
    name = "sample"
    version = "0.1.0"
}
```

## Implementation Notes

- Resolver should prefer the lockfile when it exists.
- Build commands should share resolution logic with publish and install.
- Package errors should mention the package name and version directly.
- Tests should cover local, registry, and mixed-source dependencies.
