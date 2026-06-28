# Parser Spec Notes

**Status:** Draft  
**Purpose:** Companion specification for RFC-0003 and RFC-0007

---

## Scope

This note tracks implementation-facing expectations for the parser:

- The grammar it must accept.
- The recovery behavior it must provide.
- The diagnostics it must expose when input is malformed.

## Core Expectations

- Parser should be hand-written recursive descent.
- Parser should continue after syntax errors.
- Parser should surface all recoverable errors in one pass.
- Parser should preserve source spans for all major AST nodes.

## Canonical Examples

```lang
fn add(a: Int, b: Int) -> Int {
    a + b
}
```

## Implementation Notes

- Recovery should resume at declaration or block boundaries.
- AST nodes should carry source location data.
- Parse tests should cover both valid and malformed samples from RFC-0003.
- Parser and formatter should share enough structure to support round-trip
  tests.
