# Toolchain Spec Notes

**Status:** Draft  
**Purpose:** Companion specification for RFC-0007

---

## Scope

This note tracks the implementation-facing expectations for the unified
toolchain.

## Core Expectations

- One binary should expose all major developer workflows.
- One manifest should describe the project.
- One lockfile should pin dependencies.
- Formatter and compiler should agree on source shape.
- LSP should reuse compiler analysis.

## Canonical Examples

```lang
project sample {
    name = "sample"
    version = "0.1.0"
}
```

## Implementation Notes

- Tool subcommands should share project discovery logic.
- Manifest parsing should happen before tool-specific execution.
- Formatting, diagnostics, and package resolution should use shared data
  structures where possible.
- Integration tests should cover build, format, and LSP workflows together.
