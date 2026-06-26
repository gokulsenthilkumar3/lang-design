# Type System Spec Notes

**Status:** Draft  
**Purpose:** Companion specification for RFC-0004

---

## Scope

This note tracks implementation-oriented expectations for the type system:

- What the checker must infer.
- What must remain explicit.
- What kinds of diagnostics the toolchain must surface.

## Core Expectations

- Ordinary code should infer cleanly.
- Ownership errors should mention the move or borrow that failed.
- Refinement failures should name the violated predicate.
- Effect errors should name the missing capability.
- Shape errors should show the mismatched dimensions.

## Canonical Examples

```lang
let values = List[Int]()
let total = sum(values)

#[verify]
fn clamp(value: Int, low: Int, high: Int) -> Int
    requires low <= high
{
    if value < low { low } else if value > high { high } else { value }
}
```

## Implementation Notes

- Solver should separate inference, ownership, refinement, and effect passes.
- Diagnostics should be grouped by root cause, not by internal pass name.
- Test fixtures should cover successful inference and deliberate failure cases.

