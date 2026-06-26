# RFC-0001: Language Vision and Non-Goals

**Status:** Draft  
**Author:** Gokul Senthilkumar  
**Created:** 2026-06-26  
**Updated:** 2026-06-26  

---

## Summary

This RFC defines the vision, goals, and explicit non-goals of the language. It serves as the north-star document that all subsequent RFCs must align with. If a proposed feature is in tension with this document, RFC-0001 takes precedence unless it is itself revised via the RFC process.

---

## Motivation

Language design involves thousands of individual decisions. Without an explicit vision document, decisions accumulate inconsistently and the language loses coherence. This RFC defines the constraints that make the language recognisably itself.

---

## Vision Statement

> A systems-capable language where memory is safe by default, verification is gradual and opt-in, the compiler understands intent, and the entire toolchain ships as one binary.

The target developer is someone who currently uses Rust for correctness, Python for iteration speed, and Go for deployment simplicity — and has to context-switch between all three. This language should make that switch unnecessary.

---

## Goals

### G1 — Memory Safety Without Annotation Burden
The compiler enforces memory safety through affine ownership and region inference. The programmer declares intent at API boundaries; the compiler infers the rest. Writing safe code should feel like writing Go, not like writing Rust.

### G2 — Gradual Formalism
Informal code is valid code. A function with no proof obligations compiles and runs. Proof obligations are opt-in, per-function, using refinement types. A codebase can contain both informal and formally verified functions with no artificial boundary.

### G3 — Intent-Aware Compiler
The type system encodes not just the shape of data but its purpose and invariants. The compiler uses this to produce error messages that explain the semantic problem, not just the type mismatch. In later phases, this enables AI-assisted refactoring and test generation.

### G4 — Unified Toolchain
One binary provides: compiler, build system, package manager, REPL, formatter, and language server. There is no separate configuration file format for the build system. There is no separate tool to install for formatting.

### G5 — AI as a First-Class Target
Tensor shape, graph structure, and inference batch types are in the standard library, not in a third-party crate. The GPU codegen backend is maintained as part of the language, not as an external library.

### G6 — WebAssembly as a Primary Target
WASM is not a cross-compilation afterthought. It is a first-class compilation target with the same support level as x86-64. The browser playground runs the actual compiler, compiled to WASM.

---

## Non-Goals

### NG1 — Not a scripting language
This language is not optimised for one-liner shell scripts or ad-hoc automation. Python and Bash exist for that. The language targets programs that need to be maintained, tested, and deployed.

### NG2 — Not a pure functional language
The language has algebraic effects and an effect system, but it is not Haskell. Mutation is permitted and explicit. The effect system makes mutation visible, not illegal.

### NG3 — Not a research language
This language is intended for production use. Features that are theoretically interesting but have no clear path to practical implementation within the roadmap timeline will not be included in v1.0.

### NG4 — Not a competitor to Python in its core domain
Data scientists using Python notebooks for exploratory analysis are not the primary target. The language targets the infrastructure, tooling, and inference serving layer that those notebooks depend on.

### NG5 — Not a replacement for SQL
Relational data querying is a solved problem. The language will have ergonomic database client libraries, not a built-in query language.

---

## Success Metric

RFC-0001 is working if, when a new RFC is proposed, the community can quickly determine whether it aligns with the vision by referencing this document. If RFC-0001 cannot resolve the question, this RFC should be updated rather than ignoring the gap.

---

## Open Questions

- Should the language have a name before RFC-0002, or should naming be deferred to Phase 9?
- Should NG3 (not a research language) explicitly exclude dependent types, or leave the door open for a research edition?
- How do we define "production use" precisely enough to use as a filter for feature proposals?

---

## Alternatives Considered

**Alternative: No vision document, decide per-RFC.** Rejected. Without a shared north star, RFC debates become politics rather than engineering.

**Alternative: A much shorter vision (one paragraph).** Rejected for this RFC. The non-goals are as important as the goals. A short vision omits the non-goals and leaves them ambiguous.
