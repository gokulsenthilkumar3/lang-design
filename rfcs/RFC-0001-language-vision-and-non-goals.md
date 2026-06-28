# RFC-0001: Language Vision and Non-Goals

**Status:** Draft\
**Author:** Gokul Senthilkumar\
**Created:** 2026-06-26\
**Updated:** 2026-06-28

---

## Summary

This RFC defines the vision, goals, and explicit non-goals of the language.
It serves as the north-star document that all subsequent RFCs must align with.
If a proposed feature is in tension with this document, RFC-0001 takes
precedence unless it is itself revised via the RFC process.

---

## Motivation

Language design involves thousands of individual decisions. Without an
explicit vision document, decisions accumulate inconsistently and the language
loses coherence. This RFC defines the constraints that make the language
recognisably itself.

---

## Vision Statement

> A systems-capable language where memory is safe by default, verification is
> gradual and opt-in, the compiler understands intent, and the entire toolchain
> ships as one binary.

The target developer is someone who currently uses Rust for correctness,
Python for iteration speed, and Go for deployment simplicity — and has to
context-switch between all three. This language should make that switch
unnecessary.

---

## Goals

### G1 — Memory Safety Without Annotation Burden

The compiler enforces memory safety through affine ownership and region
inference. The programmer declares intent at API boundaries; the compiler
infers the rest. Writing safe code should feel like writing Go, not like
writing Rust.

### G2 — Gradual Formalism

Informal code is valid code. A function with no proof obligations compiles and
runs. Proof obligations are opt-in, per-function, using refinement types. A
codebase can contain both informal and formally verified functions with no
artificial boundary.

### G3 — Intent-Aware Compiler

The type system encodes not just the shape of data but its purpose and
invariants. The compiler uses this to produce error messages that explain the
semantic problem, not just the type mismatch. In later phases, this enables
AI-assisted refactoring and test generation.

### G4 — Unified Toolchain

One binary provides: compiler, build system, package manager, REPL, formatter,
and language server. There is no separate configuration file format for the
build system. There is no separate tool to install for formatting.

### G5 — AI as a First-Class Target

Tensor shape, graph structure, and inference batch types are in the standard
library, not in a third-party crate. The GPU codegen backend is maintained as
part of the language, not as an external library.

### G6 — WebAssembly as a Primary Target

WASM is not a cross-compilation afterthought. It is a first-class compilation
target with the same support level as x86-64. The browser playground runs the
actual compiler, compiled to WASM.

---

## Non-Goals

### NG1 — Not a scripting language

This language is not optimised for one-liner shell scripts or ad-hoc
automation. Python and Bash exist for that. The language targets programs that
need to be maintained, tested, and deployed.

### NG2 — Not a pure functional language

The language has algebraic effects and an effect system, but it is not Haskell.
Mutation is permitted and explicit. The effect system makes mutation visible,
not illegal.

### NG3 — Not a research language

This language is intended for production use. Features that are theoretically
interesting but have no clear path to practical implementation within the
roadmap timeline will not be included in v1.0.

### NG4 — Not a competitor to Python in its core domain

Data scientists using Python notebooks for exploratory analysis are not the
primary target. The language targets the infrastructure, tooling, and inference
serving layer that those notebooks depend on.

### NG5 — Not a replacement for SQL

Relational data querying is a solved problem. The language will have ergonomic
database client libraries, not a built-in query language.

---

## Success Metric

RFC-0001 is working if, when a new RFC is proposed, the community can quickly
determine whether it aligns with the vision by referencing this document. If
RFC-0001 cannot resolve the question, this RFC should be updated rather than
ignoring the gap.

---

## Decisions

The following open questions from the original draft are now resolved. This
section records each decision and the rationale, so future RFC reviewers do
not need to re-litigate them.

### D1 — Language Name

**Decision:** Naming is formally deferred to Phase 9 (Documentation and
Community). In the interim, source code examples and tooling use the
extension `.lang` and the informal working name "lang". The name decision
does not block any RFC from RFC-0002 onwards; all subsequent RFCs should
refer to "the language" in prose rather than a proper noun.

**Rationale:** Choosing a permanent name before the community and the design
are stable creates unnecessary lock-in and is a distraction during Phase 0.
Every major language (Go was "6g", Rust was "graydon", Swift was "Codename
Swift") deferred the public name until near the first public release.

### D2 — Scope of NG3 (Research Exclusion)

**Decision:** Dependent types are **explicitly excluded from v1.0** of the
language. NG3 means: any feature that has no clear path to a practical
implementation within the Phase 0–7 roadmap timeline will not be included in
v1.0. A community-maintained "research edition" is permitted after v1.0 if
there is sufficient interest, but it is not a core deliverable.

**Rationale:** Dependent types require a fundamentally different checker and
proof obligation model. The opt-in refinement type system (RFC-0004) already
covers the practical slice of the use cases that motivate dependent types.
Keeping the door open to dependent types in the core language would delay v1.0
by years without a proportional user benefit.

### D3 — Definition of "Production Use"

**Decision:** A feature is considered to target production use if all of the
following are true:

1. It can be implemented within the roadmap phases without blocking a prior
   phase's gate.
2. It has a concrete user story in at least one of the three primary user
   segments (systems engineers, platform engineers, language and tooling
   engineers).
3. It does not require ongoing external dependency on research results that
   are not yet available (e.g., unproven inference algorithms, unpublished
   proof systems).
4. Its correctness can be validated by the compiler test suite and/or the
   benchmark targets in its RFC.

Any feature proposal that cannot satisfy all four criteria is deferred or
rejected, not included in the core language.

---

## Open Questions

All open questions from the initial RFC-0001 draft have been resolved in the
Decisions section above. If new questions arise during the review process,
they will be added here with a corresponding D-numbered decision when
closed.

---

## Alternatives Considered

**Alternative: No vision document, decide per-RFC.** Rejected. Without a
shared north star, RFC debates become politics rather than engineering.

**Alternative: A much shorter vision (one paragraph).** Rejected for this RFC.
The non-goals are as important as the goals. A short vision omits the
non-goals and leaves them ambiguous.
