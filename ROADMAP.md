# Roadmap

This document tracks the phased development plan for the language. Each phase has a defined duration, key deliverable, and explicit success criteria. Phases do not begin until the preceding Go/No-Go gate is passed.

---

## Phase 0 — Language Specification
**Duration:** 6 months | **Status:** Active

### Goal
Produce a complete, unambiguous, formally reviewed language specification before any implementation begins. Architecture decisions made here are the hardest to reverse.

### Deliverables
- RFC-0001 through RFC-0010 (core language model)
- Formal grammar in EBNF and PEG
- Type system formal specification (paper-level detail)
- Memory model formal specification
- Effect system design document
- Benchmark baseline: measure Rust, Go, Python, C++ on target workloads
- Threat model for compiler supply chain
- Hardware and platform support matrix

### Success Criteria (Gate 1)
- All RFC-0001–0010 approved by TSC
- Grammar is provably unambiguous (tested with a reference parser)
- Type system has no known soundness holes
- At least 3 external reviewers have reviewed the type system spec

---

## Phase 1 — Lexer & Parser
**Duration:** 6 months | **Status:** Not started

### Goal
Implement a complete, correct parser for the Phase 0 grammar with good error recovery and a clean AST.

### Deliverables
- Hand-written recursive-descent parser (no parser generator)
- Full lexer with Unicode support
- AST design document and implementation
- Error recovery: parser continues after syntax errors and reports all errors in one pass
- Parser test suite covering 100% of grammar productions
- Syntax highlighting grammar for VS Code and Neovim

---

## Phase 2 — Type System
**Duration:** 12 months | **Status:** Not started

### Goal
Implement the full type checker including ownership inference, the gradual formalism layer, and the effect system.

### Deliverables
- Hindley-Milner type inference core
- Affine ownership checker (borrow inference without explicit lifetime annotations)
- Refinement type layer (opt-in proof obligations)
- Algebraic effect type checker
- AI/tensor shape types
- Type error messages: every error links to documentation and suggests a fix
- Type system test suite: all spec test cases pass

---

## Phase 3 — IR & Optimiser
**Duration:** 12 months | **Status:** Not started

### Goal
Design and implement the intermediate representation and initial optimisation passes.

### Deliverables
- SSA-form IR specification
- Lowering pass from typed AST to IR
- LLVM backend (initial)
- Cranelift backend (WASM target)
- Optimisation passes: constant folding, dead code elimination, inlining
- IR test suite
- Benchmark: compilation speed ≥ 500k loc/s

---

## Phase 4 — Multi-Architecture Codegen
**Duration:** 12 months | **Status:** Not started

### Deliverables
- x86-64 native target (Linux, macOS, Windows)
- ARM64 native target (Linux, macOS/Apple Silicon)
- WebAssembly target (WASI + browser)
- RISC-V target (research/embedded)
- Cross-compilation support
- Benchmark: runtime throughput ≥ 85% of C on target workloads

---

## Phase 5 — Runtime & Standard Library
**Duration:** 18 months | **Status:** Not started

### Deliverables
- Memory allocator (affine ownership + region model)
- Async runtime (algebraic effects-based)
- Green thread scheduler
- Core stdlib: collections, strings, IO, filesystem, network
- Error handling stdlib
- REPL (read-eval-print loop with incremental type checking)
- Benchmark: p99 latency ≤ 110% of Rust equivalent

---

## Phase 6 — Developer Tooling
**Duration:** 12 months | **Status:** Not started

### Deliverables
- Language Server Protocol (LSP) implementation
  - Autocomplete (p95 < 50 ms)
  - Inline error display
  - Go-to-definition, find-references
  - Refactoring: rename, extract function
- Zero-config formatter (enforced in CI)
- DWARF-based debugger
- Time-travel debugging (target, not guaranteed)
- Integrated profiler: flame graph + allocation profiler
- VS Code extension (shipped to marketplace)
- Neovim plugin

---

## Phase 7 — Package Ecosystem
**Duration:** 12 months | **Status:** Not started

### Deliverables
- Package registry (public, self-hosted)
- Dependency solver (SAT-based, reproducible)
- Lockfile format and specification
- Private registry support
- SBOM generation per build
- Package signing and verification
- Benchmark: cold install of 100-dependency project < 10 s
- 50+ seed packages published by core team

---

## Phase 8 — Testing & Verification
**Duration:** 12 months | **Status:** Not started

### Deliverables
- Built-in test runner (no external framework needed)
- Property-based testing library
- Compiler fuzzer (find parser and type-checker bugs)
- Formal verification bridge to Lean 4
- Proof coverage target: ≥ 80% of stdlib critical paths by v1.0
- Benchmark and regression suite integrated into CI

---

## Phase 9 — Documentation & Community
**Duration:** 6 months | **Status:** Not started

### Deliverables
- Full language reference (every feature documented)
- Getting started guide (from zero to first project in 30 minutes)
- Cookbook (idiomatic solutions to 50 common problems)
- Browser playground (WASM-compiled compiler)
- AI assistant trained on stdlib and idioms (integrated into LSP)
- RFC archive published and searchable
- Community forum or discussion space

---

## Phase 10 — Hardening & Audit
**Duration:** 12 months | **Status:** Not started

### Deliverables
- Compiler fuzzing campaign (minimum 1000 CPU-hours)
- External security audit
- ABI stability guarantee documented and enforced
- Deprecation policy enforced in CI
- Zero known critical CVEs at release
- Reproducible build verification by independent party

---

## Phase 11 — GPU & AI Standard Library
**Duration:** 18 months | **Status:** Not started

### Deliverables
- Native tensor type in stdlib
- GPU codegen backend (NVIDIA CUDA, AMD ROCm, Intel Arc)
- Kernel fusion optimiser
- AI inference stdlib (forward pass, batching, quantisation)
- Benchmark: tensor op throughput ≥ 120% of PyTorch (CPU)
- Interop layer with Python (call Python ML libraries from this language)

---

## Phase 12 — Industry Adoption
**Duration:** Ongoing | **Status:** Not started

### Deliverables
- v1.0 public release
- Nonprofit foundation established (holds trademark, registry, domain)
- Enterprise support track
- University partnership programme (3+ universities teaching the language)
- Certification programme
- Annual language summit

---

## Timeline Summary

| Year | Milestone |
|------|----------|
| 1 | Language spec locked (Gate 1 passed) |
| 2 | Self-hosting parser + type checker |
| 3 | First compilable programs; Gate 2 passed |
| 4 | Self-hosting compiler (Gate 3 passed) |
| 5 | Full runtime + stdlib + REPL |
| 6 | Complete developer tooling (LSP, debugger, formatter) |
| 7 | Package registry live; Gate 4 passed |
| 8 | v1.0 release |
| 10 | GPU/AI stdlib; 3+ university courses |
| 13 | Industry mainstream adoption target |
