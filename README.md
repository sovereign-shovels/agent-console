# agent-console

> Unified log timeline for your agent sessions. Replay, search, compare across Cline, Aider, Claude Code, Goose.

**Status:** v0.1 — ready to use.

**Sovereignty:** sovereign-by-construction. Local SQLite. No cloud, no login, no telemetry.

---

## Architecture

```
┌─────────────┐     ┌──────────────┐     ┌─────────────────┐
│  Aider log  │────▶│              │     │   SQLite DB     │
├─────────────┤     │   agent-     │────▶│   (sessions +   │
│ Cline log   │────▶│   console    │     │    entries)     │
│ (planned)   │     │  (adapters)  │     ├─────────────────┤
├─────────────┤     │              │     │   FTS5 search   │
│ Claude Code │────▶│              │     └─────────────────┘
│ (planned)   │     └──────────────┘              │
├─────────────┤                                   ▼
│  Generic    │────▶                      ┌──────────────┐
│  JSON lines │                          │  CLI query   │
└─────────────┘                          │ timeline/    │
                                         │ search/list  │
                                         └──────────────┘
```

## What this is

If you run multiple agentic coding tools — Claude Code on one project, Aider on another, Cline on a third — you have no unified view of what each agent did, when, and why. Each tool stores logs differently.

agent-console reads them all and gives you one timeline you can replay, search, and diff.

## What this isn't

- Not a tool launcher
- Not a project manager
- Not a billing dashboard
- Just observability

See [PRD-v1.md](./PRD-v1.md) for the full anti-scope definition.

---

## Install

### From source

**Prerequisites:**
- [Rust](https://rustup.rs/) 1.75+

```bash
git clone https://github.com/sovereign-shovels/agent-console.git
cd agent-console

# Build
cargo build --release

# The binary is at target/release/agent-console
```

---

## Usage

### Import logs

```bash
# Import Aider log
agent-console import --tool aider ~/.aider/input-history

# Import generic JSON lines
agent-console import --tool generic ./my-logs.jsonl
```

Generic JSON lines format:
```json
{"timestamp":"2024-01-01T00:00:00Z","tool":"custom","session_id":"sess-1","action":"write","content":"file contents"}
```

### View timeline

```bash
agent-console timeline
agent-console timeline --tool aider --limit 20
```

### Search across all sessions

```bash
agent-console search "auth middleware"
```

### List sessions

```bash
agent-console sessions
```

**Demo output:**
```
$ agent-console import test-aider-log.txt --tool aider
Imported 4 entries from aider.

$ agent-console timeline

=== Session: aider-1778434991 ===
[17:43:11] [aider] output | ---
[17:43:11] [aider] output | user: Can you fix this bug?
[17:43:11] [aider] output | assistant: I'll help you with that.
[17:43:11] [aider] output | ---

$ agent-console search "refactor"
No results for 'refactor'.
```

---

## Supported tools

| Tool | Status |
|---|---|
| Aider | ✅ Basic log import |
| Generic JSON | ✅ Full support |
| Cline | 🚧 Planned v0.5 |
| Claude Code | 🚧 Planned v0.5 |
| Goose | 🚧 Planned v0.5 |

---

## Why this exists

Cross-tool unification is a structural moat — no single tool will solve it. Real demand from the polyglot agent crowd.

See [PRD-v1.md](./PRD-v1.md) for the full problem statement and rationale.

## What's next

- **v0.5:** Diff between sessions, annotation, export to shareable HTML, Cline/Claude Code/Goose adapters
- **v1.0:** Live tail mode, multi-machine aggregation, cost/token analytics

See [PRD-v1.md](./PRD-v1.md) for the full roadmap.

---

## License

Apache 2.0. See [LICENSE](./LICENSE).

## Part of sovereign-shovels

This repo is part of the [sovereign-shovels](https://github.com/sovereign-shovels) portfolio of small, focused, sovereign-by-construction AI utilities.

Other shovels: claude-vault, bulbul-studio, saaras-tray, claude-prompts, ollama-cron, mcp-forge, sarvam-pdf, agent-console, sarvam-meet, obsidian-llm, llm-diff, claude-bridge, claude-radio, sarvam-cast.
