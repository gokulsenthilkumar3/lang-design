# Memory Model Spec Notes

**Status:** Draft  
**Purpose:** Companion specification for RFC-0005

---

## Scope

This note records the implementation-facing expectations for ownership and
borrowing.

## Core Expectations

- Ownership should be clear from the source code.
- Moves should invalidate the source binding.
- Borrows should be easy to explain in diagnostics.
- Region inference should cover ordinary temporary values.

## Canonical Examples

```lang
fn duplicate_name(user: User) -> (User, User) {
    let copy = user.clone()
    (copy, user)
}
```

## Implementation Notes

- Checker should separate ownership validity from general type unification.
- Diagnostics should identify the first illegal use after a move.
- Tests should cover move, borrow, region, and resource handle cases.
