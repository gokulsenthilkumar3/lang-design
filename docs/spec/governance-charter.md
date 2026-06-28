# Governance Charter

**Status:** Draft\
**Purpose:** Defines the decision-making structure, roles, and processes for
the language project

---

## Scope

This document defines how governance works during the design phase and how
it will evolve as the project moves into implementation and community phases.
It is the authoritative reference for who makes decisions and how.

## Roles

### Maintainer

- Merges or closes RFCs after the discussion period.
- Ensures the RFC process is followed.
- Casts a deciding vote if the reviewer set is split.
- During Phase 0: **Gokul Senthilkumar** holds this role.

### Reviewer

- Reviews RFCs and implementation contributions.
- Provides technical feedback on design trade-offs.
- Has one vote per RFC on the accept/reject decision.
- Phase 0 gate requires at least **3 reviewers** to approve an RFC.

### Contributor

- Submits RFC proposals, research notes, or documentation improvements.
- Does not have a formal vote but is encouraged to participate in review
  discussions.

## Decision Process

### For RFCs

1. Author opens an issue using the RFC idea template.
2. Author submits a pull request with the RFC document.
3. Discussion period: minimum **14 days** from PR opening.
4. If 3 or more reviewers approve and the maintainer agrees, the RFC is
   merged as **Accepted**.
5. If consensus cannot be reached, the maintainer makes a final decision
   with a written rationale.
6. A rejected RFC is closed with the rejection reason recorded in the PR.

### For Spec Documents

- Spec documents (in `docs/spec/`) do not require full RFC review.
- Two-reviewer approval is sufficient for spec updates.
- Spec changes that alter the normative language semantics require a full RFC.

### For Emergency Fixes

- Security disclosures or critical correctness fixes to accepted RFCs may be
  merged by the maintainer alone, followed by a post-hoc review notice.

## Conflicts of Interest

- A reviewer must not vote on their own RFC.
- A reviewer with a personal or commercial interest in a specific design
  decision must disclose it before voting.

## Evolution

- This charter is itself versioned in the repository.
- Changes to the charter require an RFC in Phase 3 and beyond.
- During Phase 0, the maintainer may amend this document with a written
  rationale in the commit message.

## Implementation Notes

- GitHub pull requests are the canonical venue for RFC review.
- Votes are recorded as GitHub review approvals.
- The maintainer decision is recorded as a merge commit message or a
  closing comment.
