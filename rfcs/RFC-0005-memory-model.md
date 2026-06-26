# RFC-0005: Memory Model

**Status:** Draft\
**Author:** Gokul Senthilkumar\
**Created:** 2026-06-27\
**Updated:** 2026-06-27

---

## Summary

This RFC defines the memory model for the language: affine ownership by
default, region inference for common code, explicit sharing for safe aliasing,
and capability-gated access to mutation and external resources. The goal is to
achieve memory safety without a garbage collector while keeping the common case
lightweight.

---

## Motivation

The type system RFC establishes that ownership exists, but the memory model
must spell out how values move, how borrowing works, how regions are inferred,
and what the compiler guarantees about aliasing. Without a dedicated memory
model RFC, safety rules would be scattered across implementation notes and the
compiler would be hard to reason about.

---

## Detailed Design

### Core Rules

- Values are affine by default.
- Moves consume the source binding.
- Shared access is explicit and constrained.
- Region inference covers common temporary lifetimes.
- No garbage collector is part of the primary model.

### Syntax

```lang
fn first(items: List[Int]) -> Int {
    let head = items.pop_front()
    head
}

fn read_twice(path: Path) -> Result[(Str, Str), IOError]
    requires IO
{
    let content = read_file(path)
    (content.clone(), content)
}
```

### Semantics

#### Moves

- Assigning an affine value transfers ownership.
- Using a moved value is a compile-time error.
- Copyable primitives may be duplicated implicitly, but only when the type is
  marked copyable.

#### Borrowing

- Borrowing is available for shared or temporary access.
- Borrow rules prevent data races and use-after-free patterns.
- Borrow lifetimes should be inferred where possible.

#### Regions

- Regions are inferred from lexical structure and dataflow.
- Short-lived values should usually be allocated in the smallest safe region.
- Region information is an implementation detail unless surfaced in diagnostics.

#### Mutation

- Mutation is explicit in source.
- Mutable state remains subject to the same ownership and borrow rules.
- Shared mutable access must be constrained through explicit mechanisms.

#### External Resources

- File, network, process, and similar resources are capability-gated.
- Resource handles follow the same ownership rules as ordinary values.

### Type System Impact

- The checker must enforce move semantics, borrow constraints, and region
  validity.
- Ownership and region reasoning must integrate with the type inference layer.
- Diagnostics should explain ownership violations in terms of moves, borrows,
  and regions rather than implementation internals.

### Toolchain Impact

- The language server should surface move/borrow hints inline.
- The formatter should make ownership-heavy code visually easy to scan.
- The compiler needs dedicated diagnostics for use-after-move and invalid
  borrow paths.

---

## Benchmark Targets

| Metric | Target | Rationale |
| --- | --- | --- |
| Allocation throughput | > 500M alloc/sec | Safety cannot make ordinary allocation unusable |
| Deallocation throughput | > 500M dealloc/sec | Ownership should stay cheap to drop |
| GC pause (safe subset) | 0ms | No garbage collector in the primary model |
| Region allocator overhead vs malloc | < 5% | Inference should not impose large runtime cost |
| Functions needing lifetime annotations | < 10% | Borrowing should feel inference-first |

---

## Innovation Backlog

| Option | Description | Tradeoff |
| --- | --- | --- |
| Explicit lifetimes everywhere | Require lifetime annotations for all borrows | Simple to specify, heavy to use |
| Garbage collection | Add GC to simplify memory management | Easier mental model, worse latency and determinism |
| **Chosen** | Affine ownership with region inference | Best fits systems goals and annotation-light UX |

---

## Drawbacks

- Borrow checking and region inference are difficult to implement.
- Diagnostics must be excellent or the model will feel opaque.
- Some patterns that are easy in GC languages will require explicit structure.

---

## Alternatives Considered

**Garbage-collected ownership model.** Rejected because deterministic systems
software is a core target.

**Manual memory management.** Rejected because the design goal is memory safety
by default.

**Rust-style explicit lifetime syntax as the norm.** Rejected because the
project aims to reduce annotation burden in common code.

---

## Open Questions

- How much region inference is needed in the first implementation?
- Should shared mutable state require a distinct capability marker?
- Which resource types should be built into the core runtime, if any?
- How should the compiler explain aliasing violations in nested data
  structures?

---

## Success Metric

This RFC is successful if memory-safe code remains ergonomic, unsafe aliasing
is rejected at compile time, and common programs do not require explicit
lifetime annotations everywhere.

