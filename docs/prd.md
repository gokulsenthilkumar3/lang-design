# Product Requirements Document

**Product:** lang-design language platform  
**Status:** Draft  
**Owner:** Project maintainers  
**Last Updated:** 2026-06-27

---

## 1. Problem Statement

Modern teams regularly split work across Rust for safety, Python for iteration
speed, and Go for deployment simplicity. That split creates friction:

- Safety and performance live in one toolchain.
- Fast experimentation lives in another.
- Operational simplicity lives in a third.

This project defines a language platform that reduces that context switching
by designing one systems-capable language with a unified toolchain and a clear
RFC process.

## 2. Product Vision

Build a language that is:

- Memory safe by default.
- Gradual in formalism, so verification is opt-in.
- Intent-aware in its compiler diagnostics.
- Shipped with one binary for compiler, build, package, formatter, REPL, and
  language server.
- Designed for both systems software and AI-adjacent workloads.

## 3. Target Users

### Primary

- Systems engineers who want Rust-level safety with less annotation burden.
- Platform engineers who want a simpler build, package, and deployment story.
- Language and tooling engineers who want a coherent compiler workflow model.

### Secondary

- Researchers and contributors evaluating ownership, effects, and gradual
  verification choices.
- Early adopters who want to prototype syntax, semantics, and toolchain
  behaviour before implementation begins.

## 4. Goals

### G1. Establish a stable language vision

The repository must make the language identity obvious and stable enough for
later RFCs to evaluate against.

### G2. Define the core technical contract

The project must describe the intended memory model, type system, effect
system, syntax philosophy, and toolchain scope before implementation starts.

### G3. Make the design process reproducible

Every major design decision must be traceable through RFCs, research notes,
and reviewable docs.

### G4. Keep the scope buildable

The roadmap must be staged so each phase has a clear deliverable, dependency,
and acceptance gate.

## 5. Non-Goals

- Shipping compiler code in this repository.
- Chasing feature breadth before core semantics are stable.
- Becoming a scripting language or a pure research language.
- Replacing SQL, Python notebooks, or general-purpose web frameworks.
- Adding implementation details that are not backed by an accepted RFC.

## 6. Product Requirements

### 6.1 Language Requirements

- Memory safety must be enforced without a garbage collector.
- Ownership and regions must be inferable in common cases.
- Verification must be gradual and opt-in, not mandatory.
- Effects must be explicit at boundaries and visible in signatures.
- Syntax must be whitespace-insensitive and low ceremony for common cases.
- Type conversions must never happen silently.

### 6.2 Toolchain Requirements

- The toolchain must be unified into one binary.
- Compiler, build, package, formatter, LSP, and REPL must share one model.
- The build system must not require a separate config format.
- Diagnostics must be designed as part of the language experience.

### 6.3 AI and Systems Requirements

- Tensor and graph types must be first-class language concepts.
- WASM must be a first-class compilation target.
- The language must remain suitable for systems software and latency-sensitive
  deployment environments.

### 6.4 Documentation Requirements

- Every design decision must have an RFC or research note.
- Public docs must state current status clearly.
- Roadmap artifacts must distinguish vision, research, and implementation.

## 7. Success Metrics

- A new RFC author can tell within 5 minutes whether a proposal fits the
  vision.
- Every core design topic has a named RFC or research note.
- The roadmap has unambiguous phase dependencies and gates.
- Readers can understand the platform story without reading every RFC.
- The repo maintains a clean separation between design docs and future code.

## 8. Key Deliverables

- RFC-0001 and RFC-0002 as vision and syntax principles.
- RFCs for grammar, type system, memory model, and effect system.
- A benchmark targets document.
- A roadmap with phase dependencies and acceptance criteria.
- A visual roadmap diagram for quick orientation.
- Documentation structure that scales into implementation planning.

## 9. Risks

- Scope creep can turn the language into a research project.
- The vision may be too ambitious without strict phase gates.
- Toolchain integration may be under-specified if docs stay too abstract.
- Syntax and semantics may drift if examples are not kept aligned with RFCs.

## 10. Mitigations

- Keep the RFC process mandatory for design decisions.
- Require a written acceptance gate for every roadmap phase.
- Maintain a single source of truth for current status.
- Update diagrams and examples whenever the vision changes.

## 11. Open Questions

- What is the minimum viable RFC set for a first implementation repo?
- Which syntax decisions should be frozen before grammar work begins?
- What research evidence is needed before locking the ownership model?
- Should the initial implementation target native and WASM in parallel?

## 12. Related Documents

- [README](../README.md)
- [Roadmap](../ROADMAP.md)
- [RFC-0001](../rfcs/RFC-0001-language-vision-and-non-goals.md)
- [RFC-0002](../rfcs/RFC-0002-syntax-philosophy.md)

