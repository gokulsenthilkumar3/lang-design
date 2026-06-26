# Contributing

Thank you for your interest in contributing to this language design project. This document explains how to participate constructively in the research and specification phase.

---

## What We Accept

During the current **Phase 0 (Research & Architecture)**, the most valuable contributions are:

- **RFC proposals** — new design ideas in the RFC format (see below)
- **Literature review summaries** — papers, prior art, and relevant language design decisions from other languages
- **Syntax sketches** — concrete examples of how language features would look and feel
- **Prototype experiments** — small proof-of-concept implementations exploring a design question
- **Benchmark baselines** — measurements of existing languages on workloads relevant to this language’s goals
- **Feedback on open RFCs** — constructive critique, alternative approaches, and edge cases

We do not accept implementation pull requests during Phase 0. The specification must be stable before implementation begins.

---

## RFC Process

All language changes, additions, or removals require an RFC (Request for Comments).

### When an RFC Is Required

- Any new syntax or keyword
- Any change to the type system
- Any change to the memory or ownership model
- Any change to the effect system or concurrency model
- Any addition to the standard library surface area
- Any change to the build system, package format, or registry protocol

### RFC Lifecycle

1. **Draft** — Author opens a pull request adding `rfcs/RFC-XXXX-title.md`
2. **Comment period** — Minimum 14 days open for community feedback
3. **Revision** — Author addresses feedback; RFC is updated
4. **TSC review** — Technical Steering Committee votes to accept, reject, or request further revision
5. **Accepted / Rejected** — RFC is merged (accepted) or closed with explanation (rejected)

### RFC Template

Create `rfcs/RFC-XXXX-short-title.md` using this structure:

```markdown
# RFC-XXXX: Title

**Status:** Draft  
**Author:** Your Name  
**Created:** YYYY-MM-DD  
**Updated:** YYYY-MM-DD  

## Summary

One paragraph. What is being proposed and why.

## Motivation

What problem does this solve? What is the current pain? Why is this the right time?

## Existing Solutions

How do other languages or systems solve this problem? What are their limitations?

## Proposed Design

Detailed specification. Include syntax examples, type rules, and edge cases.

## Alternatives Considered

What other approaches were considered and why were they rejected?

## Drawbacks

What are the costs and risks of this proposal?

## Success Metric

How will we know this RFC’s design is working correctly in practice?

## Open Questions

What is still unresolved? What needs further research?
```

---

## Architecture Decision Records (ADRs)

For decisions that are not language-facing but affect the implementation architecture (e.g., IR format, backend choice, registry protocol), we use Architecture Decision Records.

Create `docs/decisions/ADR-XXXX-title.md` using this structure:

```markdown
# ADR-XXXX: Title

**Status:** Proposed | Accepted | Superseded by ADR-XXXX  
**Date:** YYYY-MM-DD  

## Context

What is the situation that requires a decision?

## Options Considered

| Option | Description | Pros | Cons |
|--------|-------------|------|------|
| A | ... | ... | ... |

## Decision

Which option was chosen and why.

## Consequences

What becomes easier or harder as a result of this decision?
```

---

## Code of Conduct

All contributors are expected to follow the [Code of Conduct](./CODE_OF_CONDUCT.md).

---

## Commit Style

Use conventional commits:

```
type(scope): short description

Longer explanation if needed.
```

Types: `feat`, `fix`, `docs`, `spec`, `rfc`, `bench`, `chore`

Examples:
- `spec(type-system): add refinement type layer specification`
- `rfc(RFC-0003): revise IPC syntax after feedback`
- `bench(baseline): add Rust vs Go latency comparison`

---

## Questions

Open a GitHub Discussion or comment on the relevant RFC. Do not open issues for design questions — use Discussions so the conversation is preserved for future contributors.
