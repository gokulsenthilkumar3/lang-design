# Reproducibility Spec Notes

**Status:** Draft  
**Purpose:** Companion specification for RFC-0010

---

## Scope

This note captures implementation-facing expectations for reproducible builds.

## Core Expectations

- Same inputs should produce the same outputs.
- Lockfiles should pin dependency graphs.
- Build metadata should be stable enough for independent verification.
- Non-deterministic inputs should be isolated or eliminated.

## Canonical Examples

```text
reproducible_builds = true
```

## Implementation Notes

- Build output hashes should match across independent rebuilds when inputs are
  unchanged.
- Toolchain metadata should record the manifest and lockfile inputs used.
- Integration tests should compare repeated builds of the same project.
- Failure output should explain which input caused drift.
