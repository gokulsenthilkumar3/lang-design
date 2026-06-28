# Build System Spec Notes

**Status:** Draft  
**Purpose:** Companion specification for RFC-0007 and RFC-0009

---

## Scope

This note captures implementation-facing expectations for the build system
inside the unified toolchain.

## Core Expectations

- Builds should be driven from the project manifest.
- Build targets should be explicit.
- Incremental rebuilds should reuse cached artefacts.
- Build behaviour should be deterministic for a given manifest and lockfile.

## Canonical Examples

```text
lang build
lang build --target main
lang test
```

## Implementation Notes

- Build graph should be shared across compiler and package resolution.
- Incremental state should be invalidated by source and manifest changes.
- Build output should make dependency and target selection visible.
- Integration tests should cover clean, incremental, and locked builds.
