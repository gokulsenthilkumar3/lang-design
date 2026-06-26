# Prior Art: Language Design Research

This document summarises relevant prior art in programming language design. Each entry notes what the language does well, what its limitations are, and what this project can learn from it.

---

## Memory Safety Models

### Rust — Affine Types with Explicit Borrow Syntax
**What it does well:** Rust proves that memory safety without a garbage collector is practical and deployable at scale. The borrow checker eliminates the class of memory safety bugs that has plagued C and C++ for decades.  
**Limitation:** The syntax burden of explicit lifetime annotations is a genuine barrier to adoption. Surveys consistently show borrow checker friction as the primary reason engineers avoid Rust for prototyping.  
**Lesson:** The ownership model is correct. The syntax for expressing it can be improved. Inference-first ownership is the hypothesis to test.

### Swift — Automatic Reference Counting
**What it does well:** ARC provides memory safety with low cognitive overhead in the common case. Swift developers rarely think about memory.  
**Limitation:** ARC has non-deterministic performance characteristics at cycle boundaries. It does not work for systems programming targets where latency must be deterministic.  
**Lesson:** ARC is viable as an opt-in mode (e.g., for interop or REPL) but not as the primary model.

---

## Type System Approaches

### Hindley-Milner (ML, Haskell, OCaml)
**What it does well:** Complete type inference in the common case. Programs can be written without any type annotations and the compiler infers everything.  
**Limitation:** HM struggles with features like type classes, GADTs, and higher-kinded types, which require extensions that can break decidability.  
**Lesson:** Use HM as the core, with explicit extensions for the features that require them. Keep the core decidable.

### Dependent Types (Agda, Idris, Lean 4)
**What it does well:** Types can express arbitrary program properties. Formally verified software is possible.  
**Limitation:** Full dependent types require the programmer to write proofs. This is the right tool for critical systems but too much overhead for everyday programming.  
**Lesson:** Refinement types (a restricted form of dependent types) offer most of the practical benefit with far lower overhead. Gradual formalism = refinement types, opt-in.

---

## Effect Systems

### Koka — Algebraic Effects
**What it does well:** Koka demonstrates that algebraic effects can replace monads, async/await, and checked exceptions with a single uniform mechanism. The effect system is sound and the type inference works.  
**Limitation:** Koka is a research language; its ecosystem and tooling are minimal.  
**Lesson:** The effect system design is the right approach. The implementation and tooling work is what remains.

### Haskell — Monadic IO
**What it does well:** IO effects are tracked in the type system. Pure functions are provably side-effect free.  
**Limitation:** Monadic syntax (`do` notation, `<-`, `>>=`) is a significant ergonomic cost for programmers not already familiar with category theory.  
**Lesson:** Effects must be tracked. The syntax for expressing effects must be accessible to a programmer who has never heard of monads.

---

## Toolchain Design

### Cargo (Rust)
**What it does well:** Cargo unified the Rust build system and package manager. It is widely credited as a major driver of Rust adoption.  
**Limitation:** Cargo is a separate binary from `rustc`. The build system and compiler are integrated by convention, not by design.  
**Lesson:** Go further: compiler, build, package, formatter, and LSP in one binary.

### Deno (JavaScript/TypeScript)
**What it does well:** Deno ships the runtime, formatter, linter, and test runner in one binary. The experience is noticeably better than the npm/node/eslint/prettier stack.  
**Limitation:** Deno is constrained by JavaScript semantics.  
**Lesson:** The unified toolchain approach works. Apply it to a language designed from scratch.

---

## AI-Native Languages

### Mojo (Modular)
**What it does well:** Mojo demonstrates demand for a Python-compatible language with systems-level performance for AI workloads. Strong early adoption signal.  
**Limitation:** Mojo is constrained by Python compatibility. It inherits Python's dynamic semantics at the interop boundary.  
**Lesson:** AI-native primitives should be in a language designed from scratch, not retrofitted onto Python. Tensor types in the core type system, not in a library.

### Julia
**What it does well:** Multiple dispatch and JIT compilation deliver excellent numerical performance. First-class matrix and array types.  
**Limitation:** JIT compilation means startup latency is high. Julia is not suitable for latency-sensitive systems.  
**Lesson:** AOT compilation is required for systems targets. LLVM-based AOT with AI-native types is the approach.
