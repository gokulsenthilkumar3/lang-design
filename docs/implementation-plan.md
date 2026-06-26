# Implementation Plan

**Scope:** Design-phase execution plan for the language platform  
**Status:** Draft  
**Last Updated:** 2026-06-27

---

## Objective

Turn the current vision into a complete, reviewable design package that can
support future implementation without ambiguity.

## Workstreams

### 1. Vision and Governance

- Finalise RFC-0001 and the non-goals.
- Keep governance and contribution documents aligned with the RFC process.
- Preserve a single decision trail for major language choices.

### 2. Core Language Design

- Write RFCs for grammar, type system, memory model, and effect system.
- Keep syntax examples aligned with the syntax philosophy RFC.
- Add benchmark and validation documents where a decision depends on evidence.

### 3. Toolchain Design

- Define the integrated compiler and toolchain boundary.
- Specify the build, package, formatter, and LSP responsibilities.
- Document diagnostics and developer experience requirements early.

### 4. Research and Prior Art

- Expand the prior-art notes as new design tensions appear.
- Capture benchmark targets and compare against known language tradeoffs.
- Keep research notes separate from normative RFC decisions.

### 5. Communication Assets

- Maintain a roadmap that shows phases, dependencies, and gates.
- Maintain a visual roadmap image for quick orientation.
- Keep the README as the entry point for new contributors.

## Delivery Order

### Stage A: Foundation

1. Keep RFC-0001 and RFC-0002 current.
2. Add the PRD and implementation plan.
3. Add a simple roadmap image for the repository overview.
4. Link all entry-point docs from the README.

### Stage B: Core Spec Pack

1. RFC-0003 grammar.
2. RFC-0004 type system.
3. RFC-0005 memory model.
4. RFC-0006 effect system.
5. Benchmark targets document and acceptance criteria.

### Stage C: Toolchain Spec Pack

1. Package and build system RFCs.
2. Formatter and language server requirements.
3. Diagnostic design and error-message principles.
4. WASM and native target expectations.

### Stage D: Implementation Handoff

1. Freeze the accepted spec set.
2. Move implementation planning into the future compiler repository.
3. Keep this repository as the authoritative design source.

## File Inventory

### Current entry points

- `README.md`
- `ROADMAP.md`
- `docs/prd.md`
- `docs/implementation-plan.md`
- `docs/images/roadmap.svg`
- `docs/spec/grammar.md`
- `docs/spec/type-system.md`
- `docs/spec/memory-model.md`
- `docs/spec/effects.md`
- `docs/spec/toolchain.md`
- `docs/spec/parser.md`
- `docs/spec/formatter.md`
- `docs/spec/diagnostics.md`
- `docs/spec/error-recovery.md`
- `docs/spec/cli.md`
- `docs/spec/package-management.md`
- `docs/spec/build-system.md`
- `docs/spec/lsp.md`
- `docs/spec/abi.md`
- `docs/spec/reproducibility.md`
- `docs/spec/testing.md`
- `docs/spec/roadmap-summary.md`
- `docs/spec/contributing-notes.md`
- `docs/spec/architecture-overview.md`

### Next files to create

- `docs/spec/release-notes.md`
- `docs/spec/governance-charter.md`
- `docs/spec/implementation-handoff.md`

## Acceptance Gates

- The PRD states the product problem, scope, users, and success metrics.
- The implementation plan names the next files in the sequence.
- The README points new contributors to the right entry docs.
- The roadmap image gives a one-glance view of the design journey.

## Decision Rules

- If a proposal is not needed for the vision, it stays out of the core spec.
- If a topic affects semantics, it needs an RFC before implementation.
- If a doc is likely to drift, it should be linked from the README.
- If the project cannot explain a choice in plain language, the choice needs
  more design work.
