# RFC-0004: Type System

**Status:** Draft\
**Author:** Gokul Senthilkumar\
**Created:** 2026-06-27\
**Updated:** 2026-06-27

---

## Summary

This RFC defines the core type system for the language: Hindley-Milner
inference as the default model, affine ownership as the safety layer,
refinement types as an opt-in verification layer, effect typing as the
mechanism for visible capabilities, and first-class tensor shape types for
AI-oriented workloads.

---

## Motivation

The grammar RFC tells us how code looks, but not how it behaves. We need a
coherent type model before parser and compiler work starts, because ownership,
refinement, effects, and tensor shapes all interact. Without this RFC, later
implementation work would be forced to guess at the rules.

---

## Detailed Design

### Type System Layers

The type system is intentionally layered:

1. Core Hindley-Milner inference handles the common case.
2. Affine ownership guarantees memory safety without GC.
3. Refinement types add opt-in proofs for local correctness.
4. Effect types make capabilities and side effects visible.
5. Tensor shape types support array-heavy and AI workloads.

### Syntax

```lang
fn add(a: Int, b: Int) -> Int {
    a + b
}

fn swap[T](value: (T, T)) -> (T, T) {
    (value.1, value.0)
}

#[verify]
fn divide(a: Int, b: Int) -> Int
    requires b != 0
{
    a / b
}

fn read_config(path: Path) -> Result[Str, IOError]
    requires IO
{
    // ...
}

fn matmul(a: Tensor[Float32, (M, K)], b: Tensor[Float32, (K, N)]) -> Tensor[Float32, (M, N)] {
    // ...
}
```

### Semantics

#### Hindley-Milner Core

- The compiler infers types in ordinary functions by default.
- Type annotations are required at public boundaries, recursive knots, and
  places where inference would otherwise be ambiguous.
- Type inference must remain decidable in the core language.

#### Affine Ownership

- Values are affine by default: they may be used at most once unless explicitly
  shared through an allowed reference or copyable primitive.
- Moves consume the source binding.
- Borrowing is inferred in common cases; explicit lifetime annotations are not
  the default user experience.
- The compiler must reject aliasing patterns that would break memory safety.

#### Refinement Types

- Refinement types are opt-in per function or block.
- A refinement obligation can only reference values and predicates visible at
  the boundary.
- The compiler should treat refinement failures as structured proof failures,
  not as ordinary type mismatches.

#### Effects and Capabilities

- Any operation that may touch IO, concurrency, mutation, or external systems
  must declare the required capability.
- Effect annotations become part of function identity and call-site checking.
- Effect handlers will be specified in the effect-system RFC, but the type
  system must reserve the necessary surface syntax now.

#### Tensor Shapes

- Tensor shapes participate in inference where possible.
- Shape variables are part of the type, not a runtime detail.
- Shape mismatches must produce direct diagnostics that name the conflicting
  dimensions.

### Type System Impact

- The parser must understand generic parameter syntax and refinement clauses.
- The checker must unify inference, ownership, refinement, and effect rules
  without exposing internal solver terminology to users.
- Error messages should explain the semantic cause and point to the offending
  construct.

### Toolchain Impact

- The formatter must preserve canonical type annotation layout.
- The language server should expose inferred types, ownership notes, and effect
  requirements.
- The compiler needs diagnostic categories for inference, ownership, and
  refinement failures.

---

## Benchmark Targets

| Metric | Target | Rationale |
| --- | --- | --- |
| Type check: 100k LOC | < 5 seconds | Core compile-time usability target |
| Refinement check: simple predicate | < 100ms | Verification must stay interactive |
| Tensor shape inference: per matmul | < 1ms | Shape reasoning must not dominate compile time |
| Type error: actionable message rate | > 90% | Diagnostics are part of the design |
| Functions needing lifetime annotations | < 10% | Ownership should feel inference-first |

---

## Innovation Backlog

| Option | Description | Tradeoff |
| --- | --- | --- |
| HM + explicit annotations everywhere | Keep inference minimal and require more user markup | Simpler compiler, heavier syntax burden |
| Full dependent types | Express arbitrarily rich properties in types | Powerful but too heavy for the initial product |
| **Chosen** | HM core with affine ownership, opt-in refinement, and effect typing | Balances practicality, safety, and gradual formalism |

---

## Drawbacks

- The solver is more complex than plain HM.
- Ownership inference can become hard to explain if diagnostics are weak.
- Refinements and effects will require careful phase gating to avoid scope creep.

---

## Alternatives Considered

**Plain HM only.** Rejected because it cannot express the memory and effect
model needed by the project.

**Rust-style explicit borrows everywhere.** Rejected because the project goal
is to reduce annotation burden in common code.

**Dependent types as the default.** Rejected because the design goal is
gradual formalism, not a proof-heavy language.

---

## Open Questions

- What subset of refinements is decidable and practical for v1?
- How much ownership inference is enough before explicit annotations become
  necessary?
- Should tensor shapes support symbolic arithmetic in the first release?
- How should effect polymorphism interact with generic functions?

---

## Success Metric

This RFC is successful if most ordinary code type-checks without annotations,
ownership rules prevent unsafe aliasing, refinements stay opt-in, and the
compiler can explain failures in plain language.

