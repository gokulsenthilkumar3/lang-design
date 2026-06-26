# Language Development Roadmap

---

## Phase 0 — Language Specification

**Duration:** 6 months | **Status:** Active

### Goal

Produce a complete, unambiguous, formally reviewed language specification
covering syntax, type system, memory model, and effect system. Every subsequent
phase depends on this document being stable.

### Deliverables

- RFC-0001 through RFC-0010 (core language decisions, all Accepted)
- Formal grammar (EBNF) for the full language surface syntax
- Type system specification with decision procedure
- Memory model: affine ownership + region inference rules
- Effect system: capability declarations and algebraic effect handlers
- Benchmark targets document (compile time, binary size, runtime baselines)

### Success Criteria (Gate 1)

- All RFC-0001–0010 approved by at least 3 reviewers
- Grammar is unambiguous (verified by automated parser generator)
- No open questions in RFC-0001 (vision) remain unresolved

---

## Phase 1 — Lexer & Parser

**Duration:** 4 months | **Status:** Not started

**Depends on:** Phase 0 complete (Grammar RFC accepted)

### Goal

Implement a hand-written recursive-descent parser producing a complete,
spanned AST. Error recovery must be good enough that the parser continues
after a syntax error and reports all errors in one pass.

### Deliverables

- Hand-written recursive-descent parser (no parser generator)
- Fully spanned AST (every node carries source location)
- Error recovery: parser continues after syntax errors
- Round-trip property: `parse(print(ast)) == ast`
- Fuzzing harness: parser must not panic on any input
- 100% of RFC-0002 syntax principles covered by parser tests

---

## Phase 2 — Type System

**Duration:** 6 months | **Status:** Not started

**Depends on:** Phase 1 complete

### Goal

Implement Hindley-Milner type inference extended with: affine types,
refinement types (opt-in), algebraic effects, and first-class tensor shape
types. The type checker must produce actionable error messages, not type
variable dumps.

### Deliverables

- Hindley-Milner type inference core (Algorithm W + extensions)
- Affine type checker (ownership, move, borrow)
- Region inference (no explicit lifetime annotations in common cases)
- Refinement type checker (opt-in, `#[verify]` attribute)
- Effect type checker (capability declarations, handler typing)
- Tensor shape types (`Tensor[Float32, (M, N)]`, shape inference)
- Error message quality benchmark: 90% of type errors produce actionable messages

---

## Phase 3 — IR & Optimiser

**Duration:** 5 months | **Status:** Not started

**Depends on:** Phase 2 complete

### Goal

Design and implement a Static Single Assignment IR. Implement a core set of
optimisation passes. The IR must be serialisable (for debugging and
cross-compilation caching).

### Deliverables

- SSA-form IR specification
- AST → IR lowering pass
- Optimisation passes: dead code elimination, constant folding,
  inlining (up to configurable depth), escape analysis
- IR serialisation format (binary + human-readable text)
- Optimisation benchmark: measure compile-time vs runtime tradeoff at `-O0`,
  `-O1`, `-O2`

---

## Phase 4 — Multi-Architecture Codegen

**Duration:** 6 months | **Status:** Not started

**Depends on:** Phase 3 complete

### Deliverables

- x86-64 native target (Linux, macOS, Windows)
- ARM64 native target (Linux, macOS/Apple Silicon)
- RISC-V target (Linux)
- WebAssembly target (WASI + browser)
- LLVM backend (IR → LLVM IR → native, as validation and fallback)
- Cross-compilation support (host builds target binary)
- Benchmark: binary size, startup time, runtime performance vs C baseline

---

## Phase 5 — Runtime & Standard Library

**Duration:** 8 months | **Status:** Not started

**Depends on:** Phase 4 complete

### Deliverables

- Memory allocator (affine ownership region allocator + fallback general
  allocator)
- Async runtime (algebraic-effect-based, no implicit executor)
- Standard collections: List, Map, Set, Queue, Deque
- String handling: UTF-8 by default, validated at boundary
- IO primitives: File, Network, Process (all capability-gated)
- Error handling: Result type, `?` operator, structured error context
- Stdlib test coverage: 95% line coverage

---

## Phase 6 — Developer Tooling

**Duration:** 6 months | **Status:** Not started

**Depends on:** Phase 5 (stdlib complete enough to write tools in the language)

### Deliverables

- Language Server Protocol (LSP) implementation
  - Completions, hover, go-to-definition, find-references
  - Inline type display, inline error display
- Formatter (opinionated, zero-config, idempotent)
- Linter (configurable, RFC-defined rules)
- Debugger (DWARF debug info, GDB/LLDB compatible)
- REPL (expression evaluation, type display, effect sandbox)
- Build system (integrated in the main binary, no separate config format)
- Package manager (integrated, reproducible builds, lockfile)

---

## Phase 7 — Package Ecosystem

**Duration:** 6 months | **Status:** Not started

**Depends on:** Phase 6 (package manager complete)

### Deliverables

- Package registry (public, self-hostable)
- Package signing (all published packages must be signed)
- SBOM generation (every package build produces a software bill of materials)
- Dependency resolver (SAT-based, reproducible)
- Security advisory database integration
- `500` seed packages covering: HTTP, JSON, SQL, crypto, testing, CLI

---

## Phase 8 — Testing & Verification

**Duration:** 4 months | **Status:** Not started

**Depends on:** Phase 6 (test runner), Phase 2 (refinement types)

### Deliverables

- Built-in test runner (no external framework required)
- Property-based testing (`forall` in stdlib)
- Compiler fuzzer (differential testing against reference interpreter)
- Kernel fuzzer (if OS integration is in scope)
- Formal verification integration: export proof obligations to Lean 4
- Chaos / fault injection test harness
- Benchmark runner (statistical, reports p50/p95/p99)

---

## Phase 9 — Documentation & Community

**Duration:** 4 months | **Status:** Not started

**Depends on:** Phase 7 (ecosystem exists to document)

### Deliverables

- Full language reference (every construct, formally defined)
- Tutorial: "The Language in 30 Minutes" (runnable in browser WASM playground)
- "The Language Book" (comprehensive, Rust Book-style)
- API documentation generator (integrated in toolchain)
- Community forum or GitHub Discussions structure
- Governance: RFC process published, TSC charter, security response policy
- Three university course pilot programmes

---

## Phase 10 — Hardening & Audit

**Duration:** 6 months | **Status:** Not started

**Depends on:** Phase 9 (language is stable enough for external audit)

### Deliverables

- Compiler fuzzing campaign (minimum 10 billion inputs)
- External security audit of compiler and stdlib
- ABI stability guarantee published (first stable ABI)
- Reproducible build verification (independent rebuild produces identical binary)
- CVE response process exercised (at least one dry-run disclosure)
- Performance regression suite (blocks release if any benchmark regresses >2%)

---

## Phase 11 — GPU & AI Standard Library

**Duration:** 6 months | **Status:** Not started

**Depends on:** Phase 5 (stdlib), Phase 4 (codegen)

### Deliverables

- Native tensor type in stdlib
- GPU codegen backend (CUDA, ROCm, Metal)
- Automatic differentiation in stdlib (`grad`, `jvp`, `vjp`)
- Neural network primitives (linear, conv, attention, norm layers)
- ONNX import/export
- Inference server stdlib module (HTTP + gRPC)
- AI compiler optimisations: kernel fusion, tiling, quantisation

---

## Phase 12 — Industry Adoption

**Duration:** Ongoing | **Status:** Not started

**Depends on:** Phase 10 (stable, audited release)

### Deliverables

- v1.0 public release
- Cloud provider native support (at least one major provider)
- Foundation established (nonprofit, holds trademarks and domain)
- Enterprise support programme (SLA, security patch guarantee)
- Certification programme (language proficiency, for hiring)
- OEM / hardware vendor support programme
- 100,000 GitHub repositories using the language

---

## Dependency Graph

| Phase | Depends On  |
| ----- | ----------- |
| 0     | (none)      |
| 1     | 0           |
| 2     | 1           |
| 3     | 2           |
| 4     | 3           |
| 5     | 4           |
| 6     | 5           |
| 7     | 6           |
| 8     | 6, 2        |
| 9     | 7           |
| 10    | 9           |
| 11    | 5, 4        |
| 12    | 10          |
