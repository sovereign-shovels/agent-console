# agent-console

> Unified log timeline for your agent sessions. Replay, search, compare across Cline, Aider, Claude Code, Goose.

**Status:** v0.1 — planning. Not yet released.

**Sovereignty:** sovereign-by-construction. BYO endpoint, BYO key, BYO model.
A local-only configuration is documented and tested.

This is a community project, **not affiliated with Cross-cut: Cline + Aider + Claude Code + Goose + OpenHands users**.
Best-effort community shovel — no SLA, no roadmap commitments.

---

## What this is

Unified log timeline for your agent sessions. Replay, search, compare across Cline, Aider, Claude Code, Goose.

## What this isn't

Not a tool launcher. Not a project manager. Not a billing dashboard. Just observability.

## Install

> Coming with v0.1 release.

## Configure

You bring the model. By default `agent-console` tries to use a local provider:

- For LLM endpoints: Ollama at `http://localhost:11434`
- For voice endpoints: configurable, see [docs/configure.md]

To use any other provider (Claude, GPT, Hermes, OpenRouter, Sarvam, etc.):

```toml
# ~/.config/agent-console/config.toml
[provider]
endpoint = "https://api.your-provider.com/v1"
api_key_env = "YOUR_PROVIDER_KEY"
model = "your-model-name"
```

Anthropic, OpenAI, and Sarvam endpoints all work. Local Ollama, llama.cpp,
LM Studio, and vLLM all work via their OpenAI-compatible endpoints.

## Why this exists

If you run multiple agentic coding tools — Claude Code on one project, Aider on another, Cline on a third — you have no unified view of what each agent did, when, and why. Each tool stores logs differently. agent-console reads them all and gives you one timeline you can replay, search, and diff.

## What's next

See [PRD-v1.md](./PRD-v1.md) for the full v0.1 → v0.5 → v1.0 plan.

## License

Apache 2.0. See [LICENSE](./LICENSE).

## Part of sovereign-shovels

This repo is part of the [sovereign-shovels](https://github.com/sovereign-shovels)
portfolio of small, focused, sovereign-by-construction AI utilities.

Other shovels: claude-vault, bulbul-studio, saaras-tray, claude-prompts,
ollama-cron, mcp-forge, sarvam-pdf, agent-console, sarvam-meet, obsidian-llm,
llm-diff, claude-bridge, claude-radio, sarvam-cast.
