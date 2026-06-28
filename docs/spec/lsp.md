# LSP Spec Notes

**Status:** Draft  
**Purpose:** Companion specification for RFC-0007 and RFC-0008

---

## Scope

This note captures implementation-facing expectations for the language server.

## Core Expectations

- The language server should reuse compiler analysis.
- Diagnostics should be shared with the compiler.
- Hover, completion, definition, and references should be available early.
- LSP responses should be stable and predictable.

## Canonical Examples

```text
hover
completion
go to definition
find references
```

## Implementation Notes

- LSP should use cached semantic analysis when possible.
- Completion should not require full recompilation on every keystroke.
- Diagnostics should match CLI output for the same source state.
- Tests should cover latency and correctness for common editor interactions.
