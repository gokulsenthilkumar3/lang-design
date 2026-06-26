# RFC-0006: Effect System

**Status:** Draft\
**Author:** Gokul Senthilkumar\
**Created:** 2026-06-27\
**Updated:** 2026-06-27

---

## Summary

This RFC defines the effect system for the language. Effects make side
effects explicit, capability requirements visible, and asynchronous or mutable
behaviour part of the function signature instead of an invisible convention.
The design is based on algebraic effects and handlers, but tuned for a
systems-oriented language with clear capability boundaries.

---

## Motivation

The language vision requires that important behaviours remain visible at the
boundary. Memory safety alone is not enough; IO, mutation, concurrency, and
other external interactions must also be explicit. The effect system gives us
a principled way to model those interactions without forcing the entire
language into a pure-functional style.

---

## Detailed Design

### Core Rules

- Functions declare the effects they may perform.
- Callers must satisfy the required capabilities.
- Handlers can localize and reinterpret specific effects.
- Effects are part of the type signature and checking rules.

### Syntax

```lang
fn write_log(path: Path, message: Str) -> Result[(), IOError]
    requires IO
{
    // ...
}

fn with_logging[T](action: fn() -> T) -> T
    handles IO
{
    // ...
}

fn update_state(counter: Counter) -> Counter
    requires Mut
{
    // ...
}
```

### Semantics

#### Effect Declarations

- Effects are declared on function signatures.
- A function may not perform an undeclared effect.
- Pure functions have no declared effects.

#### Capabilities

- Capability requirements are checked at call sites.
- A caller can only invoke an effectful function if the required capability is
  in scope.
- Capabilities are explicit values or declarations, not hidden global state.

#### Handlers

- Handlers intercept a specific effect within a lexical scope.
- Handled effects may be resumed or transformed depending on the handler.
- The handler semantics must stay compatible with deterministic systems code.

#### Concurrency and Async

- Async tasks are represented through effectful execution rather than an
  implicit runtime contract.
- Await-like behaviour should remain explicit in the source.

### Type System Impact

- Effect annotations participate in function identity.
- The checker must track capability availability through nested scopes.
- Effect polymorphism must remain compatible with generics and inference.

### Toolchain Impact

- Diagnostics should name the missing capability or undeclared effect.
- The language server should surface effect requirements in hover and inline
  annotations.
- The formatter should keep effect clauses aligned with function signatures.

---

## Benchmark Targets

| Metric | Target | Rationale |
| --- | --- | --- |
| Effect handler dispatch overhead | < 5 ns | Effects must remain lightweight |
| Async task switch latency | < 1 us | Concurrency must stay practical |
| Pure function effect overhead | 0 ns | Pure code should remain free of effect cost |

---

## Innovation Backlog

| Option | Description | Tradeoff |
| --- | --- | --- |
| Checked exceptions | Model effects as special return values | Simpler to teach, less expressive |
| Implicit async runtime | Hide concurrency behind a scheduler | Easier to use, less explicit and harder to reason about |
| **Chosen** | Algebraic effects with explicit capabilities | Best matches the project goals and the syntax philosophy |

---

## Drawbacks

- Effect inference can be complex.
- Handlers add another conceptual layer for users.
- The design must avoid turning capabilities into a hidden global state model.

---

## Alternatives Considered

**Checked exceptions only.** Rejected because it is narrower than the project
needs and does not model concurrency well.

**Pure async/await without effects.** Rejected because it hides important
behaviour from the type system.

**Full monadic IO as the primary model.** Rejected because the language aims
for more accessible syntax.

---

## Open Questions

- What is the minimal set of built-in capabilities for v1?
- Should handler syntax be block-based or expression-based?
- How should effects interact with trait-like abstraction?
- Should the language support effect polymorphism in the first release?

---

## Success Metric

This RFC is successful if programs can clearly express IO, mutation, and
concurrency requirements, while pure code remains simple and effect-free.

