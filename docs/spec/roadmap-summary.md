# Roadmap Summary

**Status:** Draft  
**Purpose:** Short orientation for the design roadmap

---

## What This Roadmap Says

The project is intentionally staged. Each phase closes a specific design gap
before the next layer begins:

1. Core language specification.
2. Parser and type-system implementation planning.
3. IR, code generation, runtime, and tooling design.
4. Ecosystem, hardening, and long-term adoption.

## How To Read It

- Phase 0 is about answering "what is the language?"
- Phases 1 to 3 are about making the syntax and semantics implementable.
- Phases 4 to 7 are about making the toolchain and ecosystem practical.
- Phases 8 to 12 are about trust, documentation, and adoption.

## Current Focus

- Keep RFC-0001 through RFC-0010 aligned with the vision.
- Keep benchmark targets tied to specific RFCs.
- Keep the design repository clear enough that implementation can begin later
  without reopening the core questions.

## Milestones

- Vision, syntax, type system, memory model, effects.
- Toolchain, diagnostics, package management, stability.
- Parser, formatter, build, LSP, and testing expectations.
- Documentation and handoff materials for future implementation work.

## Use Case

Use this document as the quick entry point when you want the high-level shape
of the project without reading every RFC.
