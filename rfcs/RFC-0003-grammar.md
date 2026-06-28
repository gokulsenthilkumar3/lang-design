# RFC-0003: Grammar

**Status:** Draft\
**Author:** Gokul Senthilkumar\
**Created:** 2026-06-27\
**Updated:** 2026-06-27

---

## Summary

This RFC defines the core surface grammar for the language: declarations,
expressions, patterns, types, modules, effects, attributes, and the canonical
statement structure that the parser must accept. It does not settle every
semantic question; it establishes the syntactic contract that later RFCs and
the parser implementation must follow.

---

## Motivation

Syntax is the bridge between the language vision and the implementation
toolchain. The repository already defines a syntax philosophy, but the project
still needs a concrete grammar so later RFCs can refer to named constructs
instead of informal sketches. A grammar RFC also makes the future parser
scope explicit and prevents semantic decisions from leaking into syntax
debates.

---

## Detailed Design

### Grammar Principles

The grammar is designed around the following constraints:

- Whitespace is not syntactic.
- Common constructs are short and visually stable.
- Type annotations are explicit at boundaries.
- Capability and proof markers remain visible in source.
- Every construct must have a parseable error recovery story.

### Syntax

```lang
module math.algebra

import std.io.{File, print}

pub let pi = 3.1415926535

pub fn add(a: Int, b: Int) -> Int {
    a + b
}

pub fn map_list[T, U](items: List[T], f: fn(T) -> U) -> List[U] {
    let mut out = List[U]()
    for item in items {
        out.push(f(item))
    }
    out
}

enum Result[T, E] {
    Ok(value: T)
    Err(error: E)
}

struct Point {
    x: Float64
    y: Float64
}

trait Display {
    fn format(self) -> Str
}

impl Display for Point {
    fn format(self) -> Str {
        "Point"
    }
}

#[verify]
fn safe_div(a: Int, b: Int) -> Int
    requires b != 0
{
    a / b
}

match value {
    Some(x) => x
    None => 0
}
```

### Top-Level Forms

The parser must accept these top-level forms:

- `module`
- `import`
- `pub` and `let` declarations
- `fn` definitions
- `struct` definitions
- `enum` definitions
- `trait` definitions
- `impl` blocks
- `test` blocks
- attributes such as `#[verify]`

### Expressions

Expressions must support:

- Literals
- Identifiers
- Unary and binary operators
- Function calls
- Field access
- Indexing
- Tuple and record literals
- Blocks
- `if` expressions
- `match` expressions
- `for` loops
- `while` loops
- `return`, `break`, and `continue`
- Anonymous functions

### Patterns

Patterns must support:

- Wildcards
- Variable bindings
- Tuple patterns
- Record patterns
- Enum variant patterns
- Literal patterns
- Nested patterns

### Types

Types must support:

- Primitive types
- Generic application, such as `List[Int]`
- Function types, such as `fn(Int) -> Int`
- Tuple and record types
- Enum and struct references
- Capability markers
- Refinement annotations for verified code

### Modules and Imports

The grammar must support:

- A declared module path at file scope.
- Import lists with named members.
- Relative and absolute module references.
- Visibility modifiers on exported items.

### Error Recovery

The parser must recover after:

- Missing closing delimiters.
- Missing separators in parameter lists and records.
- Unexpected tokens in expressions and pattern matches.
- Incomplete blocks at EOF.

Recovery should prefer continuing at a block boundary or the next top-level
declaration boundary so users get multiple actionable errors in one pass.

---

## Benchmark Targets

| Metric | Target | Rationale |
| --- | --- | --- |
| Grammar ambiguity count | 0 | An unambiguous grammar is required for parser correctness |
| Parse success on valid corpus | 100% | The parser must accept all approved syntax examples |
| Recovery on malformed corpus | 100% non-panicking | Error recovery is a core UX requirement |
| Round-trip stability | `parse(print(ast)) == ast` | Canonical formatting should preserve structure |
| Grammar documentation coverage | 100% of top-level forms | Every accepted construct needs a written definition |

---

## Innovation Backlog

| Option | Description | Tradeoff |
| --- | --- | --- |
| Explicit semicolons | Statement terminators on every line | Simpler parsing, worse readability and higher ceremony |
| Significant whitespace | Indentation drives block structure | Easier visual nesting, harder tooling and generation |
| Canonical block grammar | Delimiters define structure; whitespace is ignored | Cleaner tooling and formatter alignment, slightly more parser work |
| **Chosen** | Canonical block grammar with whitespace-insensitive parsing | Best matches the syntax philosophy and formatter-first design |

---

## Drawbacks

- The parser must support more recovery logic than a minimal grammar would
  require.
- Several syntax decisions remain intentionally open until later RFCs.
- The grammar can become large if every edge case is captured too early.

---

## Alternatives Considered

**Delay the grammar until the type system is accepted.** Rejected. Syntax and
type design inform each other; the grammar needs to exist early so later RFCs
can refer to named syntax forms.

**Adopt a parser-generator-first design.** Rejected. The roadmap explicitly
calls for a hand-written recursive-descent parser with robust recovery.

**Lock all syntax now.** Rejected. That would freeze the design before the
project has enough evidence from the remaining RFCs.

---

## Decisions

The following open questions from the original draft have been resolved:

### D1 — Module Declarations

**Decision:** Module structure is inferred from the file path. Mandatory file-scope module declarations are not required.
**Rationale:** Requiring a `module a.b.c` declaration at the top of `a/b/c.lang` is redundant ceremony (violates SP1). The file system path serves as the single source of truth for the module hierarchy.

### D2 — `impl` Blocks and Traits

**Decision:** `impl` blocks support exactly one trait per block (e.g., `impl TraitA for Type`).
**Rationale:** Allowing multiple traits per block makes it difficult to visually determine which trait a specific method belongs to, especially if traits share method names. One trait per block is unambiguous and aligns with SP5 (One Way to Do Common Things).

### D3 — Pattern Guards

**Decision:** Pattern guards (e.g., `match x { Some(v) if v > 0 => ... }`) are deferred to a post-v1.0 RFC.
**Rationale:** The MVP `match` expression handles structural destructuring. Pattern guards introduce parsing complexity and overlap semantically with `if` expressions, and can be added later without breaking backwards compatibility.

### D4 — Anonymous Function Syntax

**Decision:** Anonymous functions use a shorter arrow form (e.g., `|args| body`), dropping the `fn` keyword.
**Rationale:** Forcing the `fn` keyword creates visual noise for frequently used iterator chains (e.g., `map`, `filter`), violating SP1 (No Ceremony). The pipe-syntax closure is minimal and well-understood by the target audience.

---

## Open Questions

All open questions have been resolved in the Decisions section above.

---

## Success Metric

This RFC is successful if parser implementation can proceed without syntax
debates blocking core language work, and every later RFC can cite a specific
grammar form rather than inventing one ad hoc.
