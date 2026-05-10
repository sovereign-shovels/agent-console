---
repo: agent-console
rank: 8
score: 0.66
sprint: 4
substrate_anchor: Cross-cut: Cline + Aider + Claude Code + Goose + OpenHands users
build_estimate: "3–4 weeks for v0.1"
status: planned
---

# PRD v1.0 — agent-console

> **One-liner:** Unified log timeline for your agent sessions. Replay, search, compare across Cline, Aider, Claude Code, Goose.
>
> **Substrate:** Anyone running multiple agentic coding tools and losing track of what each did
> **Launch channels:** r/cursor, r/ClineProject, r/aider, AI Twitter, HN
> **Build estimate (v0.1):** 3–4 weeks for v0.1

---

## What problem does this solve

If you run multiple agentic coding tools — Claude Code on one project, Aider on another, Cline on a third — you have no unified view of what each agent did, when, and why. Each tool stores logs differently. agent-console reads them all and gives you one timeline you can replay, search, and diff.

## Why this is a shovel and not a product

Cross-tool unification is a structural moat — no single tool will solve it. Real demand from the polyglot agent crowd. Buildable. Scope-evolves into cost analytics and a community-standard log format.

---

## v0.1 — what ships

Reads logs from each tool's known log location. Unified timeline view (Tauri desktop). Session replay (step through the agent's actions). Search across sessions.

### Acceptance criteria for v0.1

A v0.1 release is publishable to GitHub when ALL of these are true:

- [ ] Core functionality described above works on the primary developer machine.
- [ ] At least one local-only configuration is documented and tested (no cloud required).
- [ ] BYO endpoint / BYO key configuration is documented.
- [ ] README explains: what it is, who it's for, how to install, how to configure, what it doesn't do.
- [ ] LICENSE present (Apache 2.0 unless overridden).
- [ ] No hardcoded keys or vendor URLs anywhere.
- [ ] No telemetry / phone-home.
- [ ] At least one passing test for the main code path.
- [ ] CI green.
- [ ] AGENTS.md compliance reviewed.

## v0.5 — first major evolution

Diff between sessions. Annotation. Export to shareable HTML. Unified Log Format proposal as a community standard.

## v1.0 — fuller scope

Live tail mode. Multi-machine aggregation (read logs over Tailscale). Cost/token analytics.

---

## Architecture sketch

### Stack

Tauri shell. SQLite for indexed log storage. Per-tool log adapters. Open the door for the Unified Log Format proposal early.

### Provider abstraction

The shovel MUST expose a provider abstraction even if v0.1 only uses one
provider. Suggested shape:

```
interface Provider {
  name: string;
  endpoint: URL;
  apiKeyEnvVar: string;
  call(input: ProviderInput): Promise<ProviderOutput>;
}
```

The default config in v0.1 must point to a free, local provider where
applicable, and document how to swap in any other.

### Configuration

Configuration order of precedence (highest to lowest):

1. Command-line flags
2. Environment variables (prefix: `AGENT_CONSOLE_*`)
3. User config file (`~/.config/agent-console/config.toml` on Linux/Mac, equivalent on Windows)
4. Default config (shipped, but never with secrets)

---

## Anti-scope (do NOT build)

Not a tool launcher. Not a project manager. Not a billing dashboard. Just observability.

---

## Tombstone risk and mitigation

**Risk:** Any one tool building proper internal observability. Medium — Aider has logs, Cline could. But cross-tool unification is the moat.

**Mitigation:** Ship fast (v0.1 in 3–4 weeks for v0.1). Build community early
(launch on r/cursor, r/ClineProject, r/aider, AI Twitter, HN). Even if upstream absorbs the feature, accumulated
stars and the community are the audience-build payoff.

**Kill signal:** All major agent tools standardizing on the same log format independently. Then this becomes a thin viewer.

If the kill signal triggers, the maintainer must announce within one week and
either (a) refocus on a remaining gap, (b) merge gracefully into upstream if
they're receptive, or (c) mark the repo as archived with a clear pointer to the
replacement.

---

## Launch plan

### Pre-launch checklist

- [ ] Repo on GitHub at `github.com/sovereign-shovels/agent-console`
- [ ] README polished (see template in `_templates/`)
- [ ] At least 3 issues / discussions seeded (real ones, not placeholder)
- [ ] LICENSE, CODE_OF_CONDUCT, CONTRIBUTING present
- [ ] Demo asset (gif, screenshot, or short video — depending on category)
- [ ] First-launch post drafted for primary launch channel

### Day-1 launch

Post to: r/cursor, r/ClineProject, r/aider, AI Twitter, HN

Subject template (adjust per channel):
- Show HN: `Show HN: agent-console – Unified log timeline for your agent sessions. Replay, search, compare across Cline, Aider, Claude Code, Goose.`
- Reddit: `[OSS] Unified log timeline for your agent sessions. Replay, search, compare across Cline, Aider, Claude Code, Goose.` with full post explaining the gap and the build
- Twitter/X: thread leading with the demo gif

### Week-1 follow-up

- Respond to every issue and comment within 24h.
- Ship at least one bugfix release based on launch feedback.
- Cross-post to secondary channels.

### Month-1 review

- Assess star velocity and community formation.
- If kill signal triggered, follow tombstone protocol above.
- If trajectory is healthy, plan v0.5.

---

## Cross-references

- Constitution: [[AGENTS]]
- Public README: [[README]]
- Progress frontmatter: [[progress]]
- Internal knowledge graph: [[knowledge-graph]]
