# RFC-0003: Type System Design

**Status:** Draft\
**Author:** Gokul Senthilkumar\
**Created:** 2026-06-27\
**Updated:** 2026-06-27\
**Depends on:** RFC-0001, RFC-0002

---

## Summary

This RFC specifies the type system: a Hindley-Milner core extended with affine
types, opt-in refinement types, algebraic effects, first-class tensor shapes,
and a capability system. The type system must be decidable in the common case
and produce actionable error messages.

---

## Motivation

The type system is the central correctness mechanism of the language. Every
goal in RFC-0001 either depends on or is expressed through the type system:

- G1 (memory safety) → affine ownership types
- G2 (gradual formalism) → opt-in refinement types
- G3 (intent-aware compiler) → semantic type annotations
- G5 (AI-native) → tensor shape types
- G6 (WASM) → type-directed compilation targets

The design must be sound (no well-typed program has undefined behaviour in
the safe subset), decidable (type checking terminates), and ergonomic
(inference eliminates annotations in the common case).

---

## Detailed Design

### Core: Hindley-Milner with Extensions

The base type system is Algorithm W (Hindley-Milner). This gives:

- Complete type inference in the common case
- Parametric polymorphism (generics)
- Decidable type checking

Extensions are layered on top, each with a defined interaction with the
HM core.

### Primitive Types

```lang
// Integers
Int8, Int16, Int32, Int64, Int128
UInt8, UInt16, UInt32, UInt64, UInt128
Int    // platform-native signed integer (64-bit on 64-bit targets)
UInt   // platform-native unsigned integer

// Floats (IEEE 754)
Float16, Float32, Float64

// Other primitives
Bool
Char   // Unicode scalar value (32-bit)
Str    // UTF-8 string slice (fat pointer: ptr + len)
Bytes  // byte slice
()
// unit type (zero-size, used as return type of effectful functions)
```

### Composite Types

```lang
// Tuple
(Int, Str, Bool)

// Array (fixed size, stack-allocated)
[Int; 8]    // array of 8 Ints

// Slice (fat pointer: ptr + len)
[Int]

// Struct
struct Point { x: Float64, y: Float64 }

// Enum (sum type, tagged union)
enum Option[T] { Some(T), None }
enum Result[T, E] { Ok(T), Err(E) }

// Function type
(Int, Int) -> Int

// Generic type
struct List[T] { ... }
```

### Affine Types (Ownership)

Every value has exactly one owner. Ownership can be transferred (moved) but
not copied unless the type implements `Clone`.

```lang
// T — owned value (affine: used exactly once unless cloned)
// &T — shared reference (many readers, no writers)
// &mut T — exclusive reference (one writer, no readers)
```

The compiler infers borrows in most cases. Explicit `&` and `&mut` are
required only at public API boundaries (SP2 from RFC-0002).

### Refinement Types (Opt-In)

Refinement types attach logical predicates to values. They are opt-in:
a function without a `#[verify]` attribute has no refinement constraints.

```lang
// Refined type alias
type NonZeroInt = Int where self != 0
type Probability = Float64 where self >= 0.0 && self <= 1.0
type SortedList[T: Ord] = List[T] where self.is_sorted()

// Precondition and postcondition
#[verify]
fn divide(a: Int, b: NonZeroInt) -> Int
    ensures result * b == a
{
    a / b
}
```

The verifier is an SMT solver (Z3 or CVC5) invoked at compile time for
`#[verify]`-annotated functions only.

### Tensor Shape Types

Tensor dimensions are part of the type. Shape mismatches are caught at
compile time.

```lang
// Tensor[dtype, shape]
let w: Tensor[Float32, (768, 768)] = Tensor.zeros()
let x: Tensor[Float32, (1, 768)]   = embed(token)

// matmul checks: (1,768) x (768,768) -> (1,768)
let y = matmul(x, w)  // type: Tensor[Float32, (1, 768)]

// Shape error caught at compile time:
let bad = matmul(w, x)  // error: shape mismatch (768,768) x (1,768)
```

Dimension variables allow polymorphic shapes:

```lang
fn matmul[M, K, N](
    a: Tensor[Float32, (M, K)],
    b: Tensor[Float32, (K, N)]
) -> Tensor[Float32, (M, N)] { ... }
```

### Effect Types (Capabilities)

Effects are part of the function type. A function that performs IO must
declare it:

```lang
fn read_file(path: Path) -> Result[Str, IOError]
    requires IO
{ ... }

// Composing effects
fn fetch_and_log(url: Url) -> Result[Json, Error]
    requires IO, Logger
{ ... }
```

A function with no `requires` clause is pure. Purity is verified by the
compiler: calling an effectful function from a pure context is a type error.

---

## Benchmark Targets

| Metric | Target | Rationale |
|--------|--------|-----------|
| Type check: 100k LOC codebase | < 5 seconds | Developer-loop speed |
| Type error: actionable message rate | > 90% | Measured by user study |
| Refinement check: simple predicate | < 100ms | Must not break the loop |
| Tensor shape inference: per matmul | < 1ms | Used in hot paths |

---

## Innovation Backlog

| Option | Description | Tradeoff |
|--------|-------------|----------|
| Pure HM | Hindley-Milner only, no extensions | Fast, limited expressiveness |
| Dependent types | Full dependent types (Agda/Idris) | Too complex for daily use |
| Gradual typing | Mix typed and untyped | Unsound boundaries |
| **Chosen: HM + Affine + Opt-in Refinement** | HM core, affine ownership, refinement opt-in | Decidable, ergonomic, sound |

---

## Drawbacks

- Two-phase compilation (HM + SMT for verified functions) adds complexity.
- Tensor shape types require the compiler to do symbolic dimension arithmetic.
- Effect types make function signatures longer at public API boundaries.

---

## Alternatives Considered

**Full dependent types (Agda/Idris style).** Rejected. Proof burden is too
high for everyday programming. Gradual formalism is the correct tradeoff.

**Gradual typing (TypeScript style).** Rejected. Unsound type boundaries
undermine the memory safety guarantee. The safe subset must be sound.

**Monad-based effects (Haskell style).** Rejected. Monadic syntax is a
barrier. Algebraic effects give the same tracking with better ergonomics.

---

## Open Questions

- Should dimension variables in tensor types be resolved at compile time
  only, or also at runtime for dynamic shapes?
- Should effects be rows (extensible, like Koka) or a fixed set of
  built-in capabilities?
- How do refinement types interact with generic type parameters?
- Should `Clone` be a compiler-known trait or a library trait?
