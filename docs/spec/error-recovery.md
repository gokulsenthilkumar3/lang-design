# Error Recovery Spec Notes

**Status:** Draft  
**Purpose:** Companion specification for RFC-0008

---

## Scope

This note captures implementation-facing expectations for syntax and semantic
error recovery.

## Core Expectations

- Parser should recover after local syntax errors.
- Checker should continue after recoverable semantic errors.
- Recovery should aim to produce more than one useful error per file.
- Recovery must never panic on malformed input.

## Canonical Examples

```lang
fn bad_example(a: Int, b: Int) -> Int {
    a + 
}
```

## Implementation Notes

- Recovery should prefer safe declaration and block boundaries.
- Missing delimiters should not prevent later top-level parsing.
- Type checker should short-circuit only when continuing would be misleading.
- Tests should include truncated files, nested syntax errors, and mixed error
  cases.

