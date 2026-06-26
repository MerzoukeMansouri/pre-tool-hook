# fast-bash — Install Guide

Claude Code PreToolUse hook that rewrites `grep` → `rg`, `find` → `fd`, and blocks dangerous shell commands.

## Step 1 — Install dependencies

```bash
brew install ripgrep fd
```

## Step 2 — Install fast-bash

**Option A: Homebrew (recommended)**

```bash
brew tap MerzoukeMansouri/homebrew
brew install MerzoukeMansouri/homebrew/fast-bash
```

**Option B: Cargo**

```bash
cargo install fast-bash
```

**Option C: Build from source**

```bash
git clone https://github.com/MerzoukeMansouri/pre-tool-hook.git
cd pre-tool-hook
cargo build --release
# binary at: target/release/fast-bash
```

## Step 3 — Wire up Claude Code

Add to `~/.claude/settings.json`:

**Homebrew / Cargo (binary in PATH):**

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

**From source (full path):**

```json
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "Bash",
        "hooks": [{ "type": "command", "command": "/path/to/pre-tool-hook/target/release/fast-bash" }]
      }
    ]
  }
}
```

## Step 4 — Verify

Restart Claude Code. Run any Bash tool with `grep -rn` — it will silently rewrite to `rg`.
