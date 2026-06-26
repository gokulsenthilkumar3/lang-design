# Grammar Spec Notes

**Status:** Draft  
**Purpose:** Companion specification for RFC-0003

---

## Scope

This document translates the grammar RFC into a practical checklist for future
parser and formatter work. It is intentionally narrower than the RFC:

- The RFC defines what the language accepts.
- This note tracks how we expect the parser and formatter to behave.
- Implementation details stay here only when they affect developer experience.

## Core Rules

- Parser accepts whitespace-insensitive source.
- Formatter produces one canonical style.
- Top-level declarations remain easy to scan.
- Blocks and expressions should be recoverable after a syntax error.

## Canonical Forms

- File starts with an optional module declaration.
- Imports appear before definitions where possible.
- Public APIs use explicit signatures.
- Block expressions always have a stable delimiter shape.
- Pattern matching should favor one readable form per case.

## Implementation Notes

- Parser should be recursive descent.
- Error recovery should skip to the next declaration boundary.
- Formatter should not require semantic analysis to stabilise layout.
- Round-trip tests should cover examples from RFC-0002 and RFC-0003.

