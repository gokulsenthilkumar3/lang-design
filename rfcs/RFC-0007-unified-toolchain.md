# RFC-0007: Unified Toolchain

**Status:** Draft\
**Author:** Gokul Senthilkumar\
**Created:** 2026-06-27\
**Updated:** 2026-06-27

---

## Summary

This RFC defines the integrated toolchain model for the language: compiler,
build system, package manager, formatter, REPL, and language server are all
shipped as one binary with one shared project model. The goal is to make the
day-to-day developer workflow coherent, reproducible, and easy to learn.

---

## Motivation

The language vision requires more than a good compiler. If build, package,
formatting, and language services live in separate tools with separate
configuration models, the user experience fragments quickly. A unified
toolchain keeps the project philosophy intact: one source of truth, one
invocation model, and one set of diagnostics.

---

## Detailed Design

### Toolchain Components

The single binary provides:

- Compilation
- Build orchestration
- Package resolution and lockfiles
- Formatting
- Language server features
- REPL and interactive evaluation
- Benchmark and test entry points

### Project Model

The toolchain uses one project manifest format for:

- Package metadata
- Dependency declarations
- Build targets
- Tool configuration that must be shared across compiler stages

The manifest must be readable by humans and stable enough for automated tools,
but should not become a second programming language.

### Syntax

```lang
project lang_design {
    name = "lang-design"
    version = "0.1.0"

    target main {
        source = "src/main.lang"
        kind = "binary"
    }

    dependency std {
        version = "^0.1"
    }

    format {
        line_width = 100
    }
}
```

### Semantics

#### Build System

- Builds are driven from the project manifest.
- Dependency resolution is reproducible and lockfile-backed.
- Build targets are explicit and can be compiled independently.
- Incremental rebuilds should reuse cached artefacts when inputs are unchanged.

#### Package Manager

- Package resolution must be deterministic.
- Package publishing must require explicit metadata and versioning.
- The lockfile must represent the exact dependency set used for a build.

#### Formatter

- Formatting is opinionated and canonical.
- The formatter must be idempotent.
- Formatting should not require semantic compilation unless necessary for
  disambiguation.

#### REPL

- The REPL evaluates expressions in the same language semantics as compiled
  code.
- Type information should be surfaced interactively where possible.

#### Language Server

- The LSP implementation must reuse compiler analysis where possible.
- Diagnostics, hover, go-to-definition, and completion should all come from the
  same semantic model as the compiler.

### Type System Impact

- Toolchain configuration must not weaken type guarantees around project data.
- Manifest parsing should be strongly typed so tool-level errors remain clear.

### Toolchain Impact

- This RFC is the direct source of truth for integrated developer tooling.
- Compiler, formatter, and LSP must not diverge in their understanding of the
  language surface.
- Tooling decisions that change user-visible semantics require an RFC update.

---

## Benchmark Targets

| Metric | Target | Rationale |
| --- | --- | --- |
| Package registry: publish time | < 30 seconds | Publishing should be fast enough for routine releases |
| Package registry: install (cached) | < 1 second | Dependency reuse must feel instant |
| LSP response time (completions) | < 100ms | Interactive editing needs low latency |
| Formatter: 100k LOC | < 1 second | Formatting should be cheap enough to run often |
| REPL startup | < 500ms | Interactive iteration should feel immediate |

---

## Innovation Backlog

| Option | Description | Tradeoff |
| --- | --- | --- |
| Separate binaries | Ship compiler, formatter, LSP, and package manager independently | Simpler to split internally, worse user experience |
| Config-file ecosystem | Allow multiple tool-specific configuration files | Flexible, but fragmented and hard to reason about |
| **Chosen** | One binary, one manifest, one project model | Best matches the repository vision and reduces context switching |

---

## Drawbacks

- A unified binary increases the surface area of the main toolchain.
- Shared project models can become brittle if they are not carefully versioned.
- Internal coupling between tooling components must be managed carefully.

---

## Alternatives Considered

**Separate tools with thin wrappers.** Rejected because it recreates the very
fragmentation this project is trying to remove.

**A generic config language for all tools.** Rejected because it would add a
second language surface without guaranteeing coherence.

**Compiler-only scope.** Rejected because the toolchain is part of the product
experience, not an optional extra.

---

## Open Questions

- What is the minimum manifest format needed for v1?
- Should the package manager be strictly registry-based or allow local sources?
- Which formatter rules can be inferred from the syntax RFC versus declared
  explicitly here?
- Should the REPL share all compiler features or run in a reduced capability
  mode?

---

## Success Metric

This RFC is successful if a developer can install one binary, create one
project manifest, and use the same toolchain for build, format, package, and
language-server workflows without switching mental models.

