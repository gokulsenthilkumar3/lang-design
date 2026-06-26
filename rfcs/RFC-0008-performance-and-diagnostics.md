# RFC-0008: Performance and Diagnostics

**Status:** Draft\
**Author:** Gokul Senthilkumar\
**Created:** 2026-06-27\
**Updated:** 2026-06-27

---

## Summary

This RFC defines the user-facing performance and diagnostics contract for the
language toolchain. It covers compiler latency, error quality, parser
recovery, and the principles that should make diagnostics feel semantic rather
than mechanical. The intent is to keep the language fast enough for daily use
and clear enough that developers can act on compiler output without decoding
internal terminology.

---

## Motivation

A language can have strong semantics and still fail in practice if the compiler
is slow, the diagnostics are vague, or syntax errors cascade into unreadable
output. Since this project is intended for production use, performance and
diagnostic quality are product requirements, not afterthoughts.

This RFC exists to make those requirements explicit before implementation
begins.

---

## Detailed Design

### Performance Principles

- Common operations must remain interactive.
- Compile-time regressions must be tracked and gated.
- Parser recovery must avoid panic-driven failure.
- Slow paths should be intentional and measurable.

### Diagnostics Principles

- Errors should describe the user-visible problem first.
- Messages should prefer semantic language over solver terminology.
- Each diagnostic should include a primary span and, when useful, a fix hint.
- The compiler should keep reporting after recoverable errors.

### Syntax

```lang
fn divide(a: Int, b: Int) -> Int {
    a / b
}

fn use_file(path: Path) -> Str
    requires IO
{
    read_file(path)
}
```

### Semantics

#### Compiler Performance

- Parsing and type checking should remain within interactive budgets for large
  projects.
- Performance targets are tracked in `docs/benchmarks/targets.md`.
- Release gates should reject regressions that exceed the documented thresholds.

#### Diagnostics

- Type errors should identify the invalid assumption or missing requirement.
- Ownership errors should point to the move or borrow that invalidated the
  value.
- Effect errors should point to the missing capability.
- Refinement errors should name the predicate that failed.
- Parser errors should name the construct that could not be recovered.

#### Error Recovery

- The parser should recover at declaration boundaries and block boundaries.
- A malformed file should produce multiple useful diagnostics whenever possible.
- The compiler should avoid suppressing later errors unless recovery is unsafe.

### Type System Impact

- Diagnostics must distinguish inference failure, ownership failure, effect
  failure, and refinement failure.
- The type checker should preserve enough context for actionable suggestions.

### Toolchain Impact

- The language server should reuse the same diagnostic model as the compiler.
- The formatter should not introduce new parse ambiguities.
- CLI output should remain concise by default and expandable for deeper detail.

---

## Benchmark Targets

| Metric | Target | Rationale |
| --- | --- | --- |
| Type error: actionable message rate | > 90% | Users need errors they can fix quickly |
| Parse 100k LOC | < 2 seconds | Parser speed keeps the feedback loop tight |
| Recovery on malformed corpus | 100% non-panicking | Syntax errors should not crash the compiler |
| First diagnostic latency | < 200ms | The first error should appear quickly |
| Diagnostic suggestion acceptance | > 80% | Hints must be useful, not noisy |

---

## Innovation Backlog

| Option | Description | Tradeoff |
| --- | --- | --- |
| Machine-readable diagnostics only | Expose raw structured errors and let tools format them | Flexible, but poor default UX |
| Rich narrative diagnostics only | Prefer human-readable prose without structured metadata | Easy to read, harder to integrate with tools |
| **Chosen** | Structured diagnostics with human-readable messages | Best supports both CLI and IDE workflows |

---

## Drawbacks

- High-quality diagnostics take significant design and implementation effort.
- Performance budgets can constrain feature experimentation.
- Recovery logic can add complexity to the parser and checker.

---

## Alternatives Considered

**Performance as a separate engineering concern.** Rejected because users
experience performance as part of the language product.

**Diagnostics as an implementation detail.** Rejected because error quality is
part of the syntax and type-system experience.

**Minimal error reporting.** Rejected because the vision explicitly values
intent-aware feedback.

---

## Open Questions

- Should suggestions be generated automatically or only from curated rules?
- What is the minimal structured diagnostic schema for v1?
- How much context should the compiler include in CLI output by default?
- Which performance regressions should block merge versus warn only?

---

## Success Metric

This RFC is successful if developers can keep moving after compiler failures,
understand what went wrong without reading internal solver jargon, and trust
that ordinary edits still compile quickly enough for an interactive workflow.

