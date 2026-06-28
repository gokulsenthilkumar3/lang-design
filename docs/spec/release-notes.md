# Release Notes Spec

**Status:** Draft\
**Purpose:** Companion specification for the release process and versioning
model of the language platform

---

## Scope

This document defines the structure, authoring policy, and tooling
expectations for release notes across the language and its toolchain. It
does not define the versioning scheme (see RFC-0010) but defines how changes
are communicated to users at each release.

## Core Rules

- Release notes are produced for every tagged release, including patch
  releases.
- Each entry must link to the RFC or issue that motivated the change.
- Breaking changes must appear at the top of the release notes under a
  dedicated heading.
- Deprecations must list the replacement mechanism and a timeline for removal.
- Security fixes must include a CVE or advisory identifier where applicable.

## Categories

Each release notes file must organise entries under these headings, omitting
empty ones:

1. **Breaking Changes** — requires a user action to migrate.
2. **New Features** — additive changes with no breaking effect.
3. **Bug Fixes** — corrections to existing behaviour.
4. **Deprecations** — features scheduled for removal in a future release.
5. **Performance** — measurable improvements against the benchmark targets.
6. **Security** — safety or correctness fixes with security implications.
7. **Toolchain** — changes to compiler, formatter, LSP, REPL, or package
   manager behaviour visible to end users.

## Format

```markdown
## v0.x.y — YYYY-MM-DD

### Breaking Changes

- **[RFC-XXXX]** Description of the breaking change and migration path.

### New Features

- **[RFC-XXXX]** Description of the new feature.

### Bug Fixes

- **[#issue]** Description of the fix.
```

## Authoring Notes

- Notes are written by the RFC author or maintainer, not auto-generated.
- Each entry must be comprehensible to a developer who has not read the RFC.
- Avoid internal implementation terminology unless it is user-visible.

## Implementation Notes

- The toolchain changelog generator should validate that every new RFC
  accepted since the last release appears in the release notes.
- Release notes live at `CHANGELOG.md` in the root of the implementation
  repository (not this design repository).
- This spec defines the schema; the implementation repository enforces it
  via CI.
