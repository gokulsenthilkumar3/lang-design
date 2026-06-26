# Formatter Spec Notes

**Status:** Draft  
**Purpose:** Companion specification for RFC-0007

---

## Scope

This note tracks implementation-facing expectations for the formatter:

- Canonical layout for source code.
- Idempotent output.
- Stable formatting for declarations, expressions, and blocks.

## Core Expectations

- Formatting should be deterministic.
- Formatting should be idempotent.
- Formatting should not change program meaning.
- Formatting should make ownership, types, and effects easy to scan.

## Canonical Examples

```lang
fn add(a: Int, b: Int) -> Int {
    a + b
}
```

## Implementation Notes

- Formatter should prefer one canonical layout for each syntax form.
- Round-trip tests should verify that formatting a formatted file is a no-op.
- Formatter should align signatures, effect clauses, and block bodies
  predictably.
- Formatting rules should be kept in sync with the syntax RFCs.

