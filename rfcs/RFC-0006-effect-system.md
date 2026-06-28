# RFC-0006: Effect System

**Status:** Draft\
**Author:** Gokul Senthilkumar\
**Created:** 2026-06-27\
**Updated:** 2026-06-28\
**Depends on:** RFC-0001, RFC-0004

---

## Summary

This RFC specifies the algebraic effect system: a unified mechanism for IO,
async/await, error handling, logging, and any user-defined side effect.
Effects are tracked in the type system. Handlers are first-class and
composable. The system replaces monads, checked exceptions, and async
keywords with one consistent mechanism.

---

## Motivation

Most languages have multiple ad-hoc mechanisms for side effects:

- Exceptions for errors
- `async/await` for concurrency
- Monads (Haskell) or do-notation for tracked effects
- Logging via globals or dependency injection

Algebraic effects unify all of these. A function declares which effects it
needs. The caller decides how to handle them. This makes effects:

- Testable (swap real IO for a mock handler)
- Composable (multiple effects in one function)
- Explicit (no hidden global state)

---

## Detailed Design

### Defining an Effect

```lang
effect Logger {
    fn log(level: LogLevel, msg: Str) -> ()
}

effect IO {
    fn read(path: Path) -> Result[Bytes, IOError]
    fn write(path: Path, data: Bytes) -> Result[(), IOError]
}

effect Random {
    fn next_u64() -> UInt64
}
```

### Using an Effect

A function that uses an effect declares it in its signature. No annotation
means pure.

```lang
// Pure function — no effects
fn add(a: Int, b: Int) -> Int { a + b }

// Effectful function
fn process(data: Bytes) -> Result[Output, Error]
    requires IO, Logger
{
    Logger.log(Info, "starting")
    let config = IO.read(Path.from("config.toml"))??
    // ...
    Logger.log(Info, "done")
    Ok(output)
}
```

### Handling an Effect

Effects are handled at the call site using `with` blocks:

```lang
// Production: real IO and stdout logger
with IO.system(), Logger.stdout() {
    process(data)
}

// Test: in-memory IO and captured log
let logs = Vec.new()
with IO.memory(files), Logger.capture(&mut logs) {
    process(data)
}
assert(logs.contains("starting"))
```

### Built-In Effects

| Effect | Description |
| --- | --- |
| `IO` | File, network, process IO |
| `Async` | Cooperative concurrency (replaces async/await) |
| `Panic` | Unrecoverable errors (replaces unwrap/panic) |
| `Alloc` | Heap allocation (for no-alloc embedded targets) |
| `Random` | Random number generation |
| `Time` | Clock access |
| `Env` | Environment variables, CLI args |

### Async Without Keywords

Async is just the `Async` effect. No `async fn`, no `.await`. The scheduler
is the effect handler.

```lang
// In other languages: async fn fetch(url: Url) -> Result<Json, Error>
fn fetch(url: Url) -> Result[Json, Error]
    requires IO, Async
{
    let resp = Http.get(url)??
    Json.parse(resp.body())
}

// Scheduling handled by the handler, not the language:
with Async.tokio_runtime() {
    let result = fetch(url)
}
```

### Effect Polymorphism

Functions can be polymorphic over effects:

```lang
// Works with any handler for E
fn retry[E](f: fn() -> Result[T, Error] requires E, times: Int)
    -> Result[T, Error]
    requires E
{
    for _ in 0..times {
        match f() {
            Ok(v) => return Ok(v)
            Err(_) => continue
        }
    }
    Err(Error.max_retries())
}
```

---

## Benchmark Targets

| Metric | Target | Rationale |
| --- | --- | --- |
| Effect handler dispatch overhead | < 5 ns | Must not replace zero-cost abstractions |
| Async task switch latency | < 1 us | Competitive with Tokio/Go runtime |
| Zero-effect pure function | 0 ns overhead | Pure functions must have no effect machinery cost |

---

## Innovation Backlog

| Option | Description | Tradeoff |
| --- | --- | --- |
| Checked exceptions (Java) | Familiar, limited | Viral, not composable |
| Monads (Haskell) | Sound, composable | Ergonomic burden |
| async/await (Rust/JS) | Ergonomic | Only handles concurrency |
| Capabilities (Pony/Wren) | Secure | Less expressive |
| **Chosen: Algebraic Effects (Koka-style)** | Unified, composable, testable | Dispatch overhead must be benchmarked carefully |

---

## Drawbacks

- Effect types make function signatures longer.
- Effect polymorphism adds type system complexity.
- Programmers unfamiliar with algebraic effects face a learning curve.
- Zero-cost for pure functions must be verified by benchmarking, not assumed.

---

## Alternatives Considered

**Monads (Haskell).** Rejected. Do-notation is an ergonomic barrier. The
underlying model is correct; the syntax is not.

**async/await only (Rust/JS/Python).** Rejected. Only handles concurrency.
Does not unify IO tracking, logging, or error handling.

**Global state / dependency injection.** Rejected. Hidden side effects are
untestable. DI frameworks are boilerplate.

---

## Decisions

The following open questions from the original draft have been resolved:

### D1 — Effect Extensibility

**Decision:** Effects are a fixed set of built-in capabilities for v1.0, rather than extensible row types. (Shared with RFC-0004).
**Rationale:** Row polymorphism severely complicates type inference and error messages. A fixed set covers the vast majority of practical use cases while keeping the type checker fast and predictable.

### D2 — `Panic` vs `?` Operator

**Decision:** The `Panic` effect and the `?` operator are completely orthogonal. `?` does not catch panics.
**Rationale:** The `?` operator acts on `Result` types for expected, recoverable errors. The `Panic` effect models unrecoverable errors (contract violations, OOM). A specific `catch_panic` handler must be invoked to intercept panics.

### D3 — Handler Scoping

**Decision:** Handlers are exclusive (one handler per effect per scope) rather than stackable.
**Rationale:** Stackable handlers make it difficult to determine which handler intercepts a call, leading to unexpected behavior and brittle code. The innermost `with` block for an effect completely shadows any outer handlers for that same effect.

### D4 — Zero-Cost Pure Functions

**Decision:** Effect-polymorphic functions are monomorphized into separate pure and effectful copies.
**Rationale:** When the pure copy of a function (e.g., `map`) is instantiated, all effect-tracking machinery and handler dispatch overhead is completely optimized away, guaranteeing 0 ns overhead for pure invocations.

---

## Open Questions

All open questions have been resolved in the Decisions section above.

---

## Success Metric

This RFC is successful if programs can clearly express IO, mutation, and
concurrency requirements while pure code remains simple and effect-free,
and if the system is measurably testable through effect handler substitution.
