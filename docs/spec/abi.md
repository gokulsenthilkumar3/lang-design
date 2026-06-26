# ABI Spec Notes

**Status:** Draft  
**Purpose:** Companion specification for RFC-0010

---

## Scope

This note captures implementation-facing expectations for ABI policy.

## Core Expectations

- ABI versioning should be explicit.
- Unstable ABI must be labeled as such.
- Public runtime interfaces should document compatibility implications.
- ABI changes should require review.

## Canonical Examples

```text
abi = "unstable"
```

## Implementation Notes

- ABI metadata should be emitted with build artefacts when applicable.
- Public runtime entry points should be grouped by versioned interface.
- Tests should cover compatibility-sensitive boundaries.
- Release tooling should refuse to imply ABI stability that has not been
  declared.

