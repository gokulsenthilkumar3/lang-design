# Architecture Overview

**Status:** Draft  
**Purpose:** High-level view of the language platform architecture

---

## Layered View

The design is organized in layers so each decision has a place:

1. Vision and governance.
2. Syntax and grammar.
3. Type system, memory model, and effects.
4. Toolchain and diagnostics.
5. Package management, build, LSP, and formatter.
6. Stability, reproducibility, and testing.
7. Documentation and community handoff.

## Core Data Flow

- Source code is parsed into an AST.
- The AST is checked by the type system and memory model rules.
- Effect and capability requirements are validated.
- Diagnostics are emitted consistently across compiler and tooling.
- Build and package commands operate on the same project model.

## Design Boundaries

- RFCs define the contract.
- Spec notes describe the implementation-facing expectations.
- Benchmarks define measurable targets.
- The README and roadmap provide entry points.

## Toolchain Shape

- One binary.
- One manifest model.
- One diagnostic model.
- One formatting standard.
- One package and build workflow.

## Future Implementation Shape

When implementation begins, the repository should support a clean split
between:

- Frontend parsing and syntax analysis.
- Semantic checking and diagnostics.
- Build and package orchestration.
- Language server and editor integration.
- Testing, reproducibility, and release hardening.
