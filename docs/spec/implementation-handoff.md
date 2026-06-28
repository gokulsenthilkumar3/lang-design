# Implementation Handoff

**Status:** Draft\
**Purpose:** Defines the criteria and process for handing the accepted design
off to a separate implementation repository

---

## Scope

This document defines when Phase 0 is complete, what the handoff package
must contain, and how the design repository and the future implementation
repository relate after the handoff.

## Handoff Criteria (Gate 1)

Phase 0 is complete and the handoff is authorised when all of the following
are true:

- RFC-0001 through RFC-0010 have been accepted (merged) with no unresolved
  open questions.
- The formal EBNF grammar (from RFC-0003) has been validated as unambiguous
  by an automated parser generator or grammar analyser.
- All spec documents listed in `docs/implementation-plan.md` under "Current
  entry points" exist and are in a non-placeholder state.
- The benchmark targets document in `docs/benchmarks/` exists and specifies
  measurable targets for each phase.
- The governance charter is accepted and the initial reviewer set is recorded.

## Handoff Package

The implementation handoff package consists of:

1. **Accepted RFC set** — RFC-0001 through RFC-0010, all merged with status
   `Accepted`.
2. **Formal grammar** — the EBNF grammar file verified as unambiguous.
3. **Spec documents** — all `docs/spec/` files in their final reviewed state.
4. **Benchmark targets** — the `docs/benchmarks/` document with numeric
   targets per phase.
5. **This document** — included in the handoff as the scope boundary record.

## Post-Handoff Relationship

After the handoff, the two repositories operate as follows:

| Concern | Design repo (this) | Implementation repo |

                param($m)
                $inner = $m.Value.TrimStart('|').Trim()
                $hasLeadingColon = $inner.StartsWith(':')
                $hasTrailingColon = $inner.EndsWith(':')
                $dashes = if ($hasLeadingColon -and $hasTrailingColon) { ':---:' }
                           elseif ($hasLeadingColon) { ':---' }
                           elseif ($hasTrailingColon) { '---:' }
                           else { '---' }
                "| $dashes "
            
                param($m)
                $inner = $m.Value.TrimStart('|').Trim()
                $hasLeadingColon = $inner.StartsWith(':')
                $hasTrailingColon = $inner.EndsWith(':')
                $dashes = if ($hasLeadingColon -and $hasTrailingColon) { ':---:' }
                           elseif ($hasLeadingColon) { ':---' }
                           elseif ($hasTrailingColon) { '---:' }
                           else { '---' }
                "| $dashes "
            
                param($m)
                $inner = $m.Value.TrimStart('|').Trim()
                $hasLeadingColon = $inner.StartsWith(':')
                $hasTrailingColon = $inner.EndsWith(':')
                $dashes = if ($hasLeadingColon -and $hasTrailingColon) { ':---:' }
                           elseif ($hasLeadingColon) { ':---' }
                           elseif ($hasTrailingColon) { '---:' }
                           else { '---' }
                "| $dashes "
            |
| RFC decisions | Authoritative | Read-only reference |
| Bug fixes to accepted RFCs | Require a new RFC | May open a design repo issue |
| Implementation bugs | Out of scope | Handled in implementation repo |
| New features | RFC required here first | Cannot merge without an accepted RFC |
| Spec clarifications | Merged with 2-reviewer approval | May reference the spec commit |

## Process

1. The maintainer opens a handoff PR against this repository containing only
   a status update to this document (marking it `Accepted`).
2. The implementation repository is created with a reference to the handoff
   commit SHA.
3. The implementation repository's README links back to this design repository
   as the authoritative spec source.
4. A GitHub Release is tagged in this repository marking the handoff point.

## Implementation Notes

- Do not add implementation code to this repository before the handoff PR
  is merged. The rule from README.md holds: every line of compiler code must
  have a corresponding accepted RFC.
- If a design gap is discovered during implementation, a new RFC must be
  opened in this repository. The implementation is blocked on that RFC until
  it is accepted.
