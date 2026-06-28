# CLI Spec Notes

**Status:** Draft  
**Purpose:** Companion specification for RFC-0007 and RFC-0008

---

## Scope

This note captures implementation-facing expectations for the command-line
interface.

## Core Expectations

- The CLI should expose build, format, test, run, and LSP entry points.
- Default output should stay concise.
- Verbose output should be available for debugging and CI.
- CLI commands should share one project discovery flow.

## Canonical Examples

```text
lang build
lang fmt
lang test
lang run
lang lsp
```

## Implementation Notes

- Subcommands should share argument parsing and help output conventions.
- Diagnostic rendering should be consistent across commands.
- Command output should be stable enough for scripts and editors.
- Help text should explain the unified toolchain model clearly.
