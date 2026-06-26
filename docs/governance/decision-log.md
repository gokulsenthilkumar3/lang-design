# Decision Log

This log records significant architectural decisions that were made outside
the formal RFC process (pre-RFC discussions, early direction choices) or
that span multiple RFCs. Every entry explains the decision, the alternatives
considered, and the rationale.

Formal RFC decisions are recorded in the RFC documents themselves.
This log is for context and continuity.

---

## DL-001: Start with Design, Not Implementation

**Date:** 2026-06-26\
**Decision:** All compiler implementation is blocked until the corresponding
RFC is accepted. No code is written speculatively.\
**Rationale:** The single most common cause of language project failure is
early implementation decisions that calcify into design constraints. By
requiring accepted RFCs before implementation, the design can be debated
freely without sunk-cost pressure.\
**Alternatives considered:** Prototype-first (write code, extract spec).
Rejected: prototypes create path dependence.

---

## DL-002: Apache-2.0 License

**Date:** 2026-06-26\
**Decision:** The language spec, compiler, and stdlib are licensed Apache-2.0.\
**Rationale:** Apache-2.0 is compatible with most corporate legal departments,
permits commercial use, includes a patent grant, and is compatible with
MIT-licensed dependencies. The Rust project uses a dual MIT/Apache-2.0
licence; Apache-2.0 alone is sufficient for this project's goals.\
**Alternatives considered:** MIT (simpler, no patent grant), GPL (copyleft,
would prevent commercial adoption), BSL (delayed open source, would prevent
community formation).

---

## DL-003: Algebraic Effects over Monads

**Date:** 2026-06-27\
**Decision:** The effect system uses algebraic effects (Koka-style) rather
than monads (Haskell-style) or async/await (Rust/JS-style). See RFC-0005.\
**Rationale:** Algebraic effects unify IO tracking, async, error handling,
and user-defined effects in one mechanism. Monads are theoretically equivalent
but syntactically heavier. async/await only handles concurrency.\
**Alternatives considered:** See RFC-0005 Innovation Backlog.

---

## DL-004: Rust-Compatible Ownership, Inference-First Syntax

**Date:** 2026-06-27\
**Decision:** The memory model is semantically identical to Rust's ownership
model. The syntax for expressing lifetimes is inference-first: explicit
annotations only at ambiguous boundaries. See RFC-0004.\
**Rationale:** Rust proves the ownership model is correct and practical.
The syntax for lifetimes is the primary ergonomic barrier. Keeping the
semantics and improving the syntax is lower risk than inventing a new model.\
**Alternatives considered:** GC (rejected: latency), ARC (rejected: cycles),
new ownership model (rejected: unproven at scale).

---

## DL-005: Unified Toolchain Binary

**Date:** 2026-06-27\
**Decision:** The compiler, build system, package manager, formatter, linter,
and REPL ship as one binary. There is no separate install step for any of
these tools.\
**Rationale:** Deno demonstrated this works and is valued by developers. The
fragmentation of the Node.js/npm/eslint/prettier toolchain is a genuine pain
point. A unified binary eliminates an entire class of "which tool version
am I running" bugs.\
**Alternatives considered:** Separate binaries (conventional but fragmented),
package-manager-installed tools (version drift problem).
