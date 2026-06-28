# RFC-0002: Syntax Philosophy

**Status:** Draft\
**Author:** Gokul Senthilkumar\
**Created:** 2026-06-26\
**Updated:** 2026-06-26

---

## Summary

This RFC establishes the syntax design principles that all grammar decisions
must follow. It does not define the full grammar (that is RFC-0003), but
defines the constraints the grammar must satisfy.

---

## Motivation

Syntax is the first thing a new developer encounters. It is also the hardest
thing to change after the language is released. Getting the philosophy right
before writing a single grammar rule prevents years of accumulated
inconsistency.

---

## Proposed Principles

### SP1 — No Ceremony for Common Cases

The most frequent operations — declaring a variable, defining a function,
calling a function, returning a value — must require the fewest possible
characters. Ceremony is reserved for operations that carry semantic weight
(capability declaration, proof obligation, unsafe block).

### SP2 — Explicit at Boundaries

Public API types, capability requirements, and proof obligations are declared
explicitly. Internal implementation types are inferred. The rule: what crosses
a boundary must be declared; what stays inside can be inferred.

### SP3 — No Implicit Conversions

Every type coercion is visible in the source code. There are no silent integer
promotions, no implicit string conversions, and no implicit numeric widening.
The programmer must explicitly request a conversion.

### SP4 — Whitespace-Insensitive

Indentation is style, not syntax. The formatter enforces a canonical style;
the parser accepts any whitespace. This eliminates an entire class of syntax
debates and makes programmatic code generation simpler.

### SP5 — One Way to Do Common Things

For any common operation, the language should have exactly one idiomatic way
to express it. Multiple equivalent syntaxes for the same operation are not
added without a compelling reason.

### SP6 — Error Messages Are Part of the Syntax Design

Every syntactic construct must have a designed error message. If a natural
error message cannot be written for a construct, the construct's design is
probably wrong.

---

## Syntax Sketch (Illustrative, Not Final)

```lang
// Variable declaration — inferred type, immutable by default
let x = 42

// Mutable variable — mutation is explicit
let mut y = 0

// Function
fn add(a: Int, b: Int) -> Int {
    a + b
}

// Function with capability constraint
fn write_file(path: Path, content: Str) -> Result[(), IOError]
    requires IO
{
    // ...
}

// Struct
struct Point {
    x: Float64
    y: Float64
}

// Enum
enum Shape {
    Circle { radius: Float64 }
    Rectangle { width: Float64, height: Float64 }
}

// Pattern matching
match shape {
    Circle { radius } => area_circle(radius)
    Rectangle { width, height } => width * height
}

// Proof obligation (opt-in)
#[verify]
fn divide(a: Int, b: Int) -> Int
    requires b != 0   // refinement type constraint
{
    a / b
}
```

---

## Alternatives Considered

**Python-style significant whitespace.** Rejected per SP4. Whitespace-sensitive
parsers are harder to tool, harder to generate, and create unnecessary friction
in editors.

**C-style syntax (semicolons, braces required).** Considered. Rejected because
the ceremony cost (semicolons, explicit braces for one-liners) violates SP1.

**Lisp-style uniform syntax.** Rejected. The readability cost for non-Lisp
programmers is too high for a language targeting production engineering teams.

---

## Decisions

The following open questions from the original draft have been resolved:

### D1 — `requires` Keyword

**Decision:** The `requires` keyword is shared for both capability constraints and refinement type preconditions.
**Rationale:** Both represent the same fundamental concept (a precondition the caller must satisfy to invoke the function). Unifying them reduces keyword soup and aligns with SP1 (No Ceremony).

### D2 — Struct Field Defaults

**Decision:** Struct field declarations use `:` for types and `=` for default values (e.g., `x: Float64 = 0.0`).
**Rationale:** Overloading `:` for defaults makes parsing ambiguous with type inference. Using `=` is unambiguous, standard across modern languages, and aligns with SP2 (Explicit at Boundaries).

### D3 — Function Definition Keyword

**Decision:** The language will use the `fn` keyword for function definitions.
**Rationale:** It is the shortest option (SP1) and highly familiar to the target audience (Rust users).

---

## Open Questions

All open questions have been resolved in the Decisions section above.
