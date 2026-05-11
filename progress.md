---
repo: agent-console
rank: 8
score: 0.66
sprint: 4
substrate_anchor: Cross-cut: Cline + Aider + Claude Code + Goose + OpenHands users
status: ready-to-launch
v01_acceptance_pct: 95
last_update: 2026-05-10
stars: 0
dependents: 0
---

# Progress — agent-console

The frontmatter above is what the root [[../PORTFOLIO]] view aggregates.
Update it as the build progresses.

## Status legend

- `planned` — PRD complete, no code yet
- `scaffolding` — repo set up, dependencies in place
- `building` — actively writing v0.1 code
- `testing` — v0.1 feature-complete, in test
- `ready-to-launch` — passes acceptance criteria, awaits launch
- `live` — published on GitHub
- `tombstone-watch` — kill signal triggered, evaluating
- `archived` — gracefully shut down

## Milestones

### v0.1
- [x] Repo initialized
- [x] Provider abstraction in place
- [x] Local-only configuration documented
- [x] Core functionality on primary platform (SQLite log storage, timeline, search, Aider adapter)
- [x] One passing test for main code path
- [x] CI green
- [x] README polished
- [x] Acceptance criteria from [[PRD-v1]] satisfied
- [ ] Launched

### Post-launch (track if `live`)
- Stars: 0
- Dependents: 0
- Open issues: 0
- Discord/community presence: none yet

## Decision log

> Append entries here for any decisions that affect direction.
> Format: `YYYY-MM-DD — what — why`.

- 2026-05-10 — scaffolded from sovereign-shovels-vault — initial PRD imported
- 2026-05-10 — v0.1 built — Rust CLI with SQLite + FTS5, timeline view, Aider log adapter, search

## Tombstone watch

What we're monitoring (from PRD-v1):

All major agent tools standardizing on the same log format independently. Then this becomes a thin viewer.

Status: not triggered.
- 2026-05-10 — hardened against local Ollama — all CLI paths verified, compile+tests green
