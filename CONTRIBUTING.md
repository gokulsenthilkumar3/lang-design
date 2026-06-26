# Contributing to lang-design

Thank you for your interest in contributing. This repository is in the early design phase — the most valuable contributions right now are:

1. **Research findings** — papers, benchmarks, or prior-art discoveries relevant to the design
2. **RFC feedback** — critique of existing RFCs via PR comments or GitHub Issues
3. **New RFC proposals** — fully formed proposals for language features or design decisions

---

## Before You Contribute

- Read [RFC-0001](rfcs/RFC-0001-language-vision-and-non-goals.md) to understand the language vision and non-goals.
- Browse existing RFCs in `rfcs/` to understand the design direction and avoid duplicating discussions.
- Check open Issues and PRs for discussions already in progress.

---

## How to Propose a New RFC

1. **Open an Issue** using the [New RFC Idea](.github/ISSUE_TEMPLATE/new_rfc_idea.md) template. This validates interest before you spend time writing a full RFC.

2. **Copy the template**: Copy `rfcs/0000-template.md` to `rfcs/RFC-XXXX-short-descriptive-title.md`. Use the next available RFC number.

3. **Fill in every section**. The CI workflow checks that `Summary`, `Motivation`, `Open Questions`, and `Alternatives Considered` are all present. A PR with empty sections will not be merged.

4. **Open a Pull Request**. Use the PR template. Set the RFC status to `Draft`.

5. **Respond to feedback**. Update the RFC in response to review comments. Significant changes should be summarised in a comment on the PR so reviewers can track what changed.

6. **Acceptance**. An RFC is merged as `Accepted` when there is clear consensus and no unresolved objections. It is closed as `Rejected` with a written reason.

---

## How to Submit a Research Finding

1. Open an Issue using the [Research Finding](.github/ISSUE_TEMPLATE/research_finding.md) template.
2. If the finding warrants an update to `docs/research/prior-art.md`, open a PR with the update.
3. If the finding changes an existing RFC, open a PR against that RFC.

---

## How to Give RFC Feedback

- For general discussion on an open RFC PR: comment on the PR.
- For a specific concern on an already-merged RFC: open an Issue using the [RFC Discussion](.github/ISSUE_TEMPLATE/rfc_discussion.md) template.

---

## Writing Style

- Write in clear, direct English. Avoid jargon where plain language works.
- RFCs are design documents, not academic papers. Precision matters more than formality.
- Use concrete examples. A syntax sketch is worth ten paragraphs of prose.
- State tradeoffs honestly. The **Alternatives Considered** and **Drawbacks** sections exist because honest evaluation of tradeoffs is more valuable than advocacy.

---

## Code of Conduct

This project follows the [Contributor Covenant v2.1](CODE_OF_CONDUCT.md). All participants are expected to uphold it.

---

## Questions?

Open an Issue with the `question` label or start a GitHub Discussion.
