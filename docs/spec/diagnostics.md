# Diagnostics Spec Notes

**Status:** Draft  
**Purpose:** Companion specification for RFC-0008

---

## Scope

This note captures implementation-facing expectations for compiler and tooling
diagnostics.

## Core Expectations

- Diagnostics should identify the root cause first.
- Diagnostics should include a primary span.
- Diagnostics should offer a useful hint when one is available.
- Diagnostics should preserve enough structure for IDE and CLI rendering.

## Canonical Examples

```lang
fn divide(a: Int, b: Int) -> Int {
    a / b
}
```

## Implementation Notes

- Diagnostic objects should carry category, severity, spans, and suggestions.
- CLI output should have a concise default mode and a verbose mode.
- Language server output should reuse the same structured diagnostic payloads.
- Tests should cover inference, ownership, effect, refinement, and parser
  diagnostics.
