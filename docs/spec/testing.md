# Testing Spec Notes

**Status:** Draft  
**Purpose:** Companion specification for RFC-0010

---

## Scope

This note captures implementation-facing expectations for the testing discipline
around the language platform.

## Core Expectations

- Tests should cover parser, type system, diagnostics, toolchain, and
  reproducibility behavior.
- Regression tests should be mandatory for compatibility-sensitive changes.
- Fuzzing should be part of the hardening story.
- Coverage targets should be measured and visible.

## Canonical Examples

```text
lang test
lang fuzz
```

## Implementation Notes

- Test runner should integrate with the unified toolchain.
- Compatibility-sensitive failures should be easy to triage.
- Fuzzing and regression suites should run in CI when available.
- Tests should include both happy-path and malformed-input cases.
