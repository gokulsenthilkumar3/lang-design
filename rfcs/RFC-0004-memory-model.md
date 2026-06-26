# RFC-0004: Memory Model

**Status:** Draft\
**Author:** Gokul Senthilkumar\
**Created:** 2026-06-27\
**Updated:** 2026-06-27\
**Depends on:** RFC-0001, RFC-0003

---

## Summary

This RFC defines the memory model: affine ownership with region inference,
no garbage collector, no implicit reference counting, and a defined unsafe
escape hatch for FFI and kernel-level code. The model must eliminate memory
safety bugs in the safe subset without requiring explicit lifetime annotations
in common code.

---

## Motivation

Memory management is the hardest problem in systems programming. The design
space has three main options:

- **GC:** Safe but non-deterministic latency, large runtime.
- **Manual (C/C++):** Fast but memory-unsafe.
- **Ownership (Rust):** Safe + fast, but explicit lifetimes are a major
  ergonomic cost.

Goal G1 (RFC-0001) requires safety without GC and without the annotation
burden of Rust. Region inference is the mechanism: the compiler infers the
lifetime of most values without programmer annotation.

---

## Detailed Design

### Ownership Rules

Every value has exactly one owner at any point in time.

1. **Move:** Passing a value to a function or assigning it to a new binding
   transfers ownership. The old binding is invalidated.
2. **Borrow:** A function can receive a temporary reference (`&T` or `&mut T`)
   without taking ownership. The borrow must not outlive the owner.
3. **Clone:** A value can be explicitly duplicated if its type implements
   `Clone`. Copying is never implicit.

### Region Inference

Lifetimes are inferred by the compiler in the common case. Explicit lifetime
annotations are required only when:

- A struct stores a reference (the struct's lifetime is tied to the
  reference's lifetime).
- A function returns a reference and the compiler cannot determine which
  input the reference was derived from.

```lang
// Common case: no annotation needed
// Compiler infers that the returned reference lives as long as `list`
fn first(list: &List[Int]) -> &Int {
    &list[0]
}

// Explicit annotation required: ambiguous which input the return comes from
fn longer<'a>(a: &'a Str, b: &'a Str) -> &'a Str {
    if a.len() > b.len() { a } else { b }
}
```

The goal is that 90%+ of functions need no lifetime annotations.

### Stack vs Heap Allocation

- Values are stack-allocated by default.
- Heap allocation is explicit via `Box[T]` (owned heap allocation) or
  `Arc[T]` (atomically reference-counted, for shared ownership).
- The allocator is pluggable: the default is a region allocator tuned for
  affine ownership patterns.

```lang
let stack_val: Point = Point { x: 1.0, y: 2.0 }  // stack
let heap_val: Box[Point] = Box.new(Point { x: 1.0, y: 2.0 })  // heap
let shared: Arc[Config] = Arc.new(Config.default())  // shared heap
```

### Unsafe Escape Hatch

The safe subset has no undefined behaviour. For FFI and kernel-level code,
an `unsafe` block permits:

- Raw pointer creation and dereference (`*T`, `*mut T`)
- Calling foreign functions
- Disabling borrow checker checks for a lexical scope

```lang
unsafe {
    let raw: *mut Int = allocate(size_of[Int]())
    *raw = 42
    let val = *raw
    deallocate(raw)
}
```

Unsafe blocks are audited by the compiler and flagged in documentation.
A codebase's `unsafe` surface is visible via `build --unsafe-report`.

### Concurrency Memory Model

- Data races are a compile-time error in safe code.
- Shared mutable state requires `Arc[Mutex[T]]` or similar.
- The memory model is sequentially consistent within a thread.
- Cross-thread ordering follows the C++20 memory model (acquire/release).
- Atomic operations are in stdlib: `Atomic[Int64]`, etc.

---

## Benchmark Targets

| Metric | Target | Rationale |
|--------|--------|-----------|
| Allocation throughput | > 500M alloc/sec | General workloads |
| Deallocation throughput | > 500M dealloc/sec | No GC pauses |
| Max GC pause | 0ms | No GC in safe subset |
| Region allocator overhead vs malloc | < 5% | Competitive with C |
| % functions needing lifetime annotations | < 10% | Ergonomics target |

---

## Innovation Backlog

| Option | Description | Tradeoff |
|--------|-------------|----------|
| Garbage collector | Safe, ergonomic | Non-deterministic latency |
| ARC everywhere | Safe, deterministic | Cycle leaks, overhead |
| Manual (C-style) | Fast | Unsafe |
| Rust-style explicit lifetimes | Safe, fast | High annotation burden |
| **Chosen: Affine + Region Inference** | Safe, fast, low annotation burden | Inference limits: ambiguous cases still need annotations |

---

## Drawbacks

- Region inference is a research-grade problem. Getting it right for a
  production language is hard and time-consuming.
- The 10% annotation target may be too aggressive; some domains
  (e.g., self-referential data structures) may require more.
- Unsafe blocks require ongoing auditing infrastructure.

---

## Alternatives Considered

**GC (Go/Java style).** Rejected. Incompatible with G1 (systems-capable,
deterministic latency). GC is available as an opt-in mode for higher-level
code but not as the default.

**ARC everywhere (Swift).** Rejected. ARC has non-deterministic behaviour at
cycle boundaries and is unsuitable for latency-critical code paths.

**Rust-style explicit lifetimes.** Partially adopted. The ownership rules are
identical to Rust. Only the annotation syntax is different: inference-first
rather than explicit-first.

---

## Open Questions

- What is the exact inference algorithm for region inference? (Polyhedral
  analysis? Constraint-based? A bespoke algorithm?)
- How does the region allocator interact with `Box[T]` and `Arc[T]`?
- Should `unsafe` be a block or a function attribute?
- How are raw pointers represented in the type system to prevent
  use-after-free even within unsafe blocks?
