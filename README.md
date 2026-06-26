# lang-design

> Design repository for a new systems programming language — memory safe by
> default, AI-native, gradual formalism, unified toolchain.

[![CI](https://github.com/gokulsenthilkumar3/lang-design/actions/workflows/ci.yml/badge.svg)](https://github.com/gokulsenthilkumar3/lang-design/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)
[![RFCs](https://img.shields.io/badge/RFCs-2%20active-green.svg)](rfcs/)
[![Status](https://img.shields.io/badge/phase-design%20%26%20research-orange.svg)](#roadmap)

---

## What Is This?

This is the design and specification repository for a programming language that
targets the gap between Rust (correctness), Python (iteration speed), and Go
(deployment simplicity) — and aims to make context-switching between all three
unnecessary.

This repository contains:

- **RFCs** — every design decision is an RFC, discussed publicly
- **Research notes** — prior art, academic papers, benchmarks informing decisions
- **Syntax sketches** — illustrative (not final) code examples
- **Governance documents** — how decisions get made

No compiler code lives here yet. The rule: **every line of compiler code must
have a corresponding accepted RFC.**

---

## Vision

> A systems-capable language where memory is safe by default, verification is
> gradual and opt-in, the compiler understands intent, and the entire toolchain
> ships as one binary.

See [RFC-0001](rfcs/RFC-0001-language-vision-and-non-goals.md) for the full
vision and non-goals.

---

## Core Properties

| Property                  | Description                                            | Status         |
| ------------------------- | ------------------------------------------------------ | -------------- |
| **Memory safety**         | Affine ownership + region inference. No GC. No unsafe. | RFC-0001 Draft |
| **Gradual formalism**     | Refinement types, opt-in per function.                 | RFC-0001 Draft |
| **Intent-aware compiler** | Type system encodes purpose. Errors explain semantics. | RFC-0001 Draft |
| **Unified toolchain**     | Compiler + build + pkg + formatter + LSP = one binary. | RFC-0001 Draft |
| **AI-native types**       | Tensor shape, graph structure in stdlib.               | RFC-0001 Draft |
| **WASM primary target**   | WASM is a first-class compilation target.              | RFC-0001 Draft |

---

## Repository Structure

```text
lang-design/
├── rfcs/                    # All design decisions as RFCs
│   ├── 0000-template.md     # RFC template — copy this for new RFCs
│   ├── RFC-0001-language-vision-and-non-goals.md
│   └── RFC-0002-syntax-philosophy.md
├── docs/
│   └── research/
│       └── prior-art.md     # Prior art: languages, type systems, toolchains
├── examples/
│   └── syntax-sketch.md # Illustrative syntax sketches (not final)
├── .github/
│   ├── ISSUE_TEMPLATE/  # RFC idea, discussion, research templates
│   ├── PULL_REQUEST_TEMPLATE.md
│   └── workflows/ci.yml # Markdown lint + RFC structure validation
├── CONTRIBUTING.md
├── SECURITY.md
├── CODE_OF_CONDUCT.md
└── LICENSE              # Apache-2.0
```

---

## RFC Process

Every design decision goes through an RFC. The lifecycle:

```text
Idea (GitHub Issue)
    ↓
Draft RFC (Pull Request)
    ↓
Active Discussion (PR comments, linked issues)
    ↓
Accepted / Rejected (merged or closed PR)
    ↓
Implementation (future compiler repo)
```

1. **Open an issue** using the
   [New RFC Idea](.github/ISSUE_TEMPLATE/new_rfc_idea.md) template.
2. **Copy** `rfcs/0000-template.md` to `rfcs/RFC-XXXX-short-title.md`.
3. **Fill it in** completely. Empty sections block merge.
4. **Open a PR** with your RFC. Use the PR template.
5. Discussion happens on the PR. Author updates the RFC in response.
6. RFC is merged as **Accepted** or closed as **Rejected**.

See [CONTRIBUTING.md](CONTRIBUTING.md) for the full guide.

---

## Roadmap

| Phase                            | Focus                                           | Target       |
| -------------------------------- | ----------------------------------------------- | ------------ |
| **0 — Vision & Governance**      | Vision RFC, research, prior art, governance     | In progress  |
| **1 — Core Language Design**     | Type system, ownership model, effect system RFCs | Q3 2026      |
| **2 — Syntax Finalisation**      | Grammar RFC, parser spec, error message design  | Q4 2026      |
| **3 — Stdlib & Toolchain Design** | Package format, build system, LSP protocol RFCs | Q1 2027      |
| **4 — Compiler Bootstrap**       | Separate implementation repo created            | Q3 2027      |
| **5 — Ecosystem & Community**    | Package registry, docs site, first contributors | 2028         |

---

## Current RFC Status

| RFC                                                                          | Title                        | Status |
| ---------------------------------------------------------------------------- | ---------------------------- | ------ |
| [RFC-0001](rfcs/RFC-0001-language-vision-and-non-goals.md)                   | Language Vision and Non-Goals | Draft |
| [RFC-0002](rfcs/RFC-0002-syntax-philosophy.md)                               | Syntax Philosophy             | Draft |

---

## Contributing

All contributions are welcome — research findings, RFC feedback, syntax
critique, or entirely new RFC proposals. Read
[CONTRIBUTING.md](CONTRIBUTING.md) to get started.

---

## Author

**Gokul Senthilkumar** — Full-Stack Developer & SDET, Tamil Nadu, India.\
Building this as part of a longer-term platform vision covering both a language
and an OS.\
GitHub: [@gokulsenthilkumar3](https://github.com/gokulsenthilkumar3)

---

## License

Apache-2.0. See [LICENSE](LICENSE).
