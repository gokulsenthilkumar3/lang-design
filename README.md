# lang-design

Research, specification, and design documents for a new systems programming language built from first principles.

---

## Problem Statement

No existing language scores above 7 across all critical design dimensions simultaneously:

| Language | Memory Safety | AI-Native | Readability | Compile Speed | Formal Verify | Cross-Platform |
|----------|:---:|:---:|:---:|:---:|:---:|:---:|
| Python | 2 | 9 | 9 | 4 | 2 | 7 |
| Rust | 10 | 5 | 5 | 4 | 7 | 8 |
| Go | 6 | 4 | 8 | 9 | 3 | 7 |
| C++ | 4 | 5 | 4 | 8 | 5 | 9 |
| Swift | 7 | 5 | 8 | 7 | 5 | 5 |
| **This Language** | **9** | **9** | **9** | **8** | **8** | **9** |

*Scores 0–10. Target: ≥8 across all dimensions.*

Specific unsolved problems this language addresses:

| Current Pain | Why Existing Tools Don’t Solve It | This Language’s Approach |
|---|---|---|
| Rust memory safety requires steep borrow-checker syntax | Syntax is fundamental to Rust’s ownership model; cannot be changed without breaking the language | Ownership-by-default with inferred lifetimes; explicit annotation only at API boundaries |
| Python is too slow for AI inference workloads | Dynamic typing prevents LLVM-level optimisation | Statically typed core with Python-style ergonomics at the REPL layer |
| No language has AI-native compiler tooling | AI tools are wrappers around existing compilers, not integrated | Type system encodes intent; compiler uses it to suggest refactors, detect logic errors, generate tests |
| Formal verification requires full proof-assistant expertise | Agda/Idris/Lean require all-or-nothing commitment | Gradual formalism: write informally, opt into proof obligations per function |
| Every language has a fragmented toolchain | Build systems, package managers, and runtimes evolved separately | Unified binary: compiler + build + package manager + REPL + formatter |

---

## Vision

A systems-capable programming language where:

- **Memory is safe by default** without borrow-checker syntax overhead. The compiler enforces ownership; the programmer declares intent.
- **Gradual formalism** lets you prototype informally and opt into formal verification for critical paths — in the same codebase, without rewriting.
- **The compiler understands intent**, not just types. It can detect logic errors, suggest architectural refactors, and generate test cases from specs.
- **One binary ships everything**: compiler, build system, package manager, REPL, formatter, and language server.
- **AI workloads are a first-class target**: the type system has native tensor and graph types; the scheduler is aware of GPU memory pressure.
- **WebAssembly is a primary compilation target** alongside native binaries — not an afterthought.

---

## Design Pillars

| Pillar | Description | Inspiration | Gap Filled |
|--------|-------------|-------------|------------|
| Ownership without annotation burden | Memory safety enforced by the compiler, not declared by the programmer at every call site | Rust | Rust’s learning curve is a political blocker in most engineering orgs |
| Gradual formalism | Informal code is valid. Proof obligations are opt-in per function. | Lean 4, Idris 2 | No language offers this spectrum without a full rewrite |
| Intent-aware type system | Types encode not just shape but purpose and invariants | Dependent types, refinement types | Enables AI-compiler integration and better error messages |
| Unified toolchain | One binary: compiler, build, package, REPL, fmt, LSP | Cargo + rustc (separate), Deno (close) | Eliminates the largest category of onboarding friction |
| AI-native primitives | Tensor, graph, and stream types in the stdlib | Mojo, Julia | Python is the glue; this language is the foundation |
| WASM-first portability | WASM is a primary target, not a cross-compilation afterthought | Grain, AssemblyScript | Serverless and edge computing require WASM-native languages |

---

## Roadmap

### Phase Overview

| Phase | Name | Duration | Key Deliverable |
|-------|------|----------|-----------------|
| 0 | Language Specification | 6 months | RFC-0001 through RFC-0020; formal grammar; type system spec |
| 1 | Lexer & Parser | 6 months | Full parser for spec grammar; error recovery; AST |
| 2 | Type System | 12 months | Type checker, borrow inference, gradual formalism layer |
| 3 | IR & Optimiser | 12 months | SSA-form IR, LLVM backend, basic optimisation passes |
| 4 | Multi-Architecture Codegen | 12 months | x86-64, ARM64, WASM targets; cross-compilation |
| 5 | Runtime & Standard Library | 18 months | Allocator, scheduler, async runtime, core stdlib |
| 6 | Developer Tooling | 12 months | LSP, debugger, formatter, profiler, REPL |
| 7 | Package Ecosystem | 12 months | Package registry, dependency solver, build reproducibility |
| 8 | Testing & Verification | 12 months | Test framework, fuzzer, formal verification bridge (Lean 4) |
| 9 | Documentation & Community | 6 months | Full language reference, tutorials, playground |
| 10 | Hardening & Audit | 12 months | Compiler fuzzing, security audit, ABI stability guarantee |
| 11 | GPU & AI Stdlib | 18 months | Native tensor types, GPU codegen, CUDA/ROCm backends |
| 12 | Industry Adoption | Ongoing | University partnerships, enterprise support, foundation |

### Go / No-Go Gates

| Gate | Trigger | Pass Criteria | Fail Action |
|------|---------|---------------|-------------|
| Gate 1 (Month 6) | End of spec phase | RFC-0001–0020 approved; grammar is unambiguous; type system formally specified | Extend spec phase; do not begin parser |
| Gate 2 (Year 2) | Type system functional | All spec test cases type-check correctly; borrow inference has no known unsound cases | Architecture review of type system design |
| Gate 3 (Year 4) | Self-hosting compiler | Language compiles its own compiler; LLVM backend produces correct binaries | Scope reduction; defer gradual formalism to Phase 8 |
| Gate 4 (Year 7) | Ecosystem functional | Package registry has 500+ packages; LSP works in VS Code and Neovim; 3 production users | Delay v1.0; extend ecosystem phase |

---

## Language Design Decisions

### Syntax Philosophy

- **No ceremony for common cases.** Variable declarations, function signatures, and struct definitions should read like prose.
- **Explicit at boundaries.** Public API types, capability constraints, and proof obligations are declared explicitly. Internal implementation is inferred.
- **No implicit conversions.** Every type coercion is visible in the source. No silent integer promotion, no implicit string conversion.
- **Whitespace-insensitive.** Indentation is style, not syntax. Consistent formatting is enforced by the formatter, not the parser.

### Type System

| Feature | Approach | Rationale |
|---------|----------|-----------|
| Base type system | Hindley-Milner with extensions | Well-understood, decidable, excellent inference |
| Ownership model | Affine types (use-at-most-once) | Enables memory safety without GC and without borrow syntax |
| Gradual formalism | Refinement type layer, opt-in | Proof obligations attach to functions, not to the whole program |
| Effect system | Algebraic effects | Explicit IO, mutation, and exception effects without monadic syntax overhead |
| AI/tensor types | First-class tensor shape types | Shape errors caught at compile time, not runtime |
| WASM types | Linear memory and table types | WASM is a first-class target, not a cross-compilation hack |

### Memory Model

| Option | Approach | Decision |
|--------|----------|----------|
| A — Garbage collection | Automatic memory management | Rejected: unpredictable latency; incompatible with systems targets |
| B — Reference counting | Automatic with cycle detection | Considered for REPL layer only |
| C — Borrow checker (Rust-style) | Compiler-enforced lifetimes, explicit syntax | Rejected as primary model: syntax burden blocks adoption |
| D — Affine ownership (selected) | Compiler enforces use-once; regions handle bulk allocation | Selected: safety of Rust, readability of Go |
| E — Region-based | Arena allocation per task/scope | Used as an optimisation within affine ownership model |

**Decision:** Affine ownership (D) as the primary model. RC (B) as opt-in for interop and REPL. Regions (E) as a compiler-managed optimisation.

### Concurrency Model

| Option | Approach | Decision |
|--------|----------|----------|
| A — OS threads | POSIX threads, shared memory | Too low-level; data races possible |
| B — Green threads (Go-style) | M:N scheduler, goroutines | Good ergonomics; weak isolation |
| C — Async/await (Rust-style) | Explicit future composition | Too much syntax noise for common cases |
| D — Algebraic effects (selected) | Effect handlers for async, concurrency, and IO | Unifies async, concurrency, and error handling in one model |
| E — Actor model | Isolated state, message passing | Considered for distributed targets; deferred to stdlib |

**Decision:** Algebraic effects (D) as the core model. Actor model (E) as a stdlib abstraction on top.

---

## Benchmark Targets

| Dimension | Metric | Target |
|-----------|--------|--------|
| Compilation speed | Lines per second (release build) | ≥ 500 000 loc/s |
| Compilation speed | Hello-world incremental rebuild | < 100 ms |
| Runtime performance | Peak throughput vs C | ≥ 85% |
| Runtime performance | Latency p99 vs Rust | ≤ 110% |
| Memory safety | Known memory bugs in compiler test suite | 0 |
| Formal verification | Proof coverage on stdlib critical paths | ≥ 80% by v1.0 |
| Package install | Cold install of 100-dependency project | < 10 s |
| LSP response | Autocomplete latency (p95) | < 50 ms |
| WASM binary size | Hello-world WASM output | < 50 KB |
| AI inference | Tensor op throughput vs PyTorch (CPU) | ≥ 120% |

---

## Security Model

- **Supply chain integrity**: all dependencies pinned by cryptographic hash; SBOM generated per build
- **Reproducible builds**: bit-for-bit identical output from identical inputs, verified by independent rebuild
- **Secure compiler**: compiler itself is subject to formal verification and reproducible build requirements
- **Memory safety by construction**: the type system makes the class of memory safety bugs structurally impossible
- **Capability-aware stdlib**: IO, network, and filesystem access requires explicit capability tokens (aligned with the companion OS project)
- **CVE response**: 90-day coordinated disclosure policy; security@lang-design contact from day one

---

## Developer Experience Plan

| Tool | Delivery Phase | Description |
|------|---------------|-------------|
| Compiler (`lc`) | Phase 0–3 | Single binary; incremental, parallel compilation |
| Build system | Phase 0–3 | Integrated; no separate build file format |
| Package manager | Phase 7 | Reproducible installs; lockfile; private registry support |
| Language Server (LSP) | Phase 6 | Autocomplete, inline errors, refactoring; VS Code + Neovim day one |
| Formatter | Phase 6 | Zero-config; opinionated; enforced in CI |
| Debugger | Phase 6 | DWARF-based; time-travel debugging target |
| Profiler | Phase 6 | Integrated flame graph; allocation profiler |
| REPL | Phase 5 | Full language REPL with incremental type checking |
| AI assistant | Phase 9 | Trained on stdlib and idioms; integrated into LSP |
| Playground | Phase 9 | Browser-based; WASM-compiled compiler |
| Crash reporter | Phase 10 | Structured crash telemetry with stack symbolication |

---

## Governance

| Structure | Detail |
|-----------|--------|
| RFC process | All language changes require a public RFC with minimum 14-day comment period |
| Technical Steering Committee | 5–7 members, 2-year term limits, written decision criteria |
| Security Response Team | Dedicated CVE handling, 90-day disclosure policy |
| Foundation | Nonprofit or fiscal sponsorship; holds trademark, domain, package registry |
| Versioning | Semantic versioning; ABI stability guaranteed from v1.0 |
| Deprecation policy | Minimum 2 major versions’ notice before removal; automated migration tooling required |
| Compatibility guarantee | No breaking changes to stable APIs without TSC approval and migration path |

---

## Research References

- [Rust Reference — The Rust Programming Language](https://doc.rust-lang.org/reference/)
- [Simple Types and Programming Languages — Pierce (2002)](https://www.cis.upenn.edu/~bcpierce/tapl/)
- [Algebraic Effects for the Rest of Us — Daan Leijen](https://www.microsoft.com/en-us/research/publication/algebraic-effects-for-the-rest-of-us/)
- [Lean 4 Theorem Prover](https://lean-lang.org/)
- [Mojo Programming Language — Modular](https://docs.modular.com/mojo/)
- [The Cranelift Code Generator](https://cranelift.dev/)
- [LLVM Language Reference](https://llvm.org/docs/LangRef.html)
- [WebAssembly Specification](https://webassembly.github.io/spec/core/)
- [Refinement Types for ML — Freeman & Pfenning (1991)](https://dl.acm.org/doi/10.1145/113445.113468)
- [An Introduction to Algebraic Effects and Handlers — Pretnar (2015)](https://www.eff-lang.org/handlers-tutorial.pdf)

---

## Repository Structure

```
lang-design/
├── spec/
│   ├── grammar/             # Formal grammar (EBNF + PEG)
│   ├── type-system/         # Type system formal specification
│   ├── memory-model/        # Ownership and region model
│   ├── concurrency/         # Effect system and concurrency model
│   └── stdlib/              # Standard library specification
├── rfcs/                    # Request for Comments — design proposals
├── docs/
│   ├── research/            # Literature reviews and prior art
│   ├── decisions/           # Architecture Decision Records (ADR-XXXX)
│   └── benchmarks/          # Performance targets and comparisons
├── prototypes/              # Proof-of-concept experiments (parser, type checker)
├── examples/                # Syntax examples and idiom sketches
├── ROADMAP.md
├── CONTRIBUTING.md
├── CODE_OF_CONDUCT.md
└── LICENSE
```

---

## Phase 0 Checklist

- [ ] RFC-0001: Language vision and non-goals
- [ ] RFC-0002: Syntax philosophy and grammar approach
- [ ] RFC-0003: Type system foundations (HM + extensions)
- [ ] RFC-0004: Ownership and memory model
- [ ] RFC-0005: Effect system and concurrency model
- [ ] RFC-0006: Gradual formalism layer
- [ ] RFC-0007: Module and package system
- [ ] RFC-0008: Error handling model
- [ ] RFC-0009: AI and tensor type primitives
- [ ] RFC-0010: WASM compilation strategy
- [ ] Literature review: Rust, Lean 4, Mojo, Grain, Koka
- [ ] Formal grammar draft (EBNF)
- [ ] Type system formal specification draft
- [ ] Memory model formal specification draft
- [ ] Benchmark baseline measurements (vs Rust, Go, Python, C++)

---

## Related

This language is being co-designed alongside a new operating system. The language will eventually be the primary implementation language for OS components above the verified kernel layer.

- OS research repository: [os-research](https://github.com/gokulsenthilkumar3/os-research)

---

## Contributing

Contributions are welcome in the form of RFC proposals, literature review summaries, syntax sketches, and prototype experiments. See [CONTRIBUTING.md](./CONTRIBUTING.md) for guidelines.

---

## License

Apache License 2.0 — see [LICENSE](./LICENSE).
