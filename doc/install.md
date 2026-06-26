# fast-bash — Install Guide

Claude Code PreToolUse hook that rewrites `grep` → `rg`, `find` → `fd`, and blocks dangerous shell commands.

## Step 1 — Install fast-bash

```bash
brew tap MerzoukeMansouri/homebrew
brew install MerzoukeMansouri/homebrew/fast-bash
```

## Step 2 — Wire up Claude Code

Add to `~/.claude/settings.json`:

```json
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "Bash",
        "hooks": [{ "type": "command", "command": "fast-bash" }]
      }
    ]
  }
}
```

## Step 3 — Verify

Restart Claude Code. Run any Bash tool with `grep -rn` — it will silently rewrite to `rg`.
