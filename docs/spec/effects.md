# Effect System Spec Notes

**Status:** Draft  
**Purpose:** Companion specification for RFC-0006

---

## Scope

This note tracks the implementation-facing expectations for effect typing.

## Core Expectations

- Effects should appear in signatures.
- Capability checks should happen at call sites.
- Handlers should be lexical and explicit.
- Pure code should not pay an effect tax.

## Canonical Examples

```lang
fn read_and_parse(path: Path) -> Result[Config, IOError]
    requires IO
{
    // ...
}
```

## Implementation Notes

- Checker should track capabilities through nested scopes.
- Handler resolution should be deterministic.
- Diagnostics should mention the missing effect or capability directly.

