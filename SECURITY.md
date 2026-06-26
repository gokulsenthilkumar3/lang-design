# Security Policy

## Scope

This repository contains language design documents, RFCs, and syntax sketches.
It does not contain executable compiler code at this stage.

Once compiler implementation begins, this policy will be updated with:

- Supported compiler versions
- Known vulnerability classes (memory safety, supply chain, compiler bugs)
- Coordinated disclosure timelines

## Reporting a Vulnerability

If you find a security-relevant issue in this repository (for example, a
design decision that would create a structural security weakness in the
language), please:

1. **Do not open a public issue.** Open a private discussion or contact the
   project maintainer directly.
2. Describe the issue clearly: what the design decision is, why it creates a
   security risk, and what a safer alternative would be.
3. Allow 14 days for a response before any public disclosure.

## Security Design Principles

The language is being designed with the following security properties as
first-class goals:

- Memory safety without unsafe escape hatches in the safe subset
- Capability-based effect system (IO, Network, FS must be declared and granted)
- Reproducible builds (compiler output is deterministic given the same input
  and toolchain version)
- Supply chain security: package signatures and SBOM generation in the
  standard package manager
- Secure-by-default standard library (no implicit network access, no ambient
  authority)
