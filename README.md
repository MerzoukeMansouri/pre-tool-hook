# fast-bash

Claude Code `PreToolUse` hook that rewrites slow shell commands to faster modern alternatives and blocks dangerous operations.

## What it does

| Before | After |
|--------|-------|
| `grep -rn "foo" .` | `rg -n "foo" .` + auto-exclude `node_modules`, `dist`, `target`, … |
| `find . -name "*.ts"` | `fd -e ts .` + auto-exclude noise dirs |
| `grep -rn --include="*.rs"` | `rg -g "*.rs"` |
| `rm -rf /` | **blocked** |
| `git push --force` | **blocked** (use `--force-with-lease`) |

## Setup

| | Build Manually | Homebrew |
|---|---|---|
| **Install** | `cargo install fast-bash` | `brew tap MerzoukeMansouri/homebrew`<br>`brew install MerzoukeMansouri/homebrew/fast-bash` |
| **Dependencies** | `brew install ripgrep fd` | included |
| **`~/.claude/settings.json`** | `"command": "/path/to/target/release/fast-bash"` | `"command": "fast-bash"` |

Full `settings.json` snippet:

```json
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "Bash",
        "hooks": [
          {
            "type": "command",
            "command": "fast-bash"
          }
        ]
      }
    ]
  }
}
```

## License

MIT
