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

## Install

```bash
cargo install fast-bash
```

Requires [ripgrep](https://github.com/BurntSushi/ripgrep) (`rg`) and [fd](https://github.com/sharkdp/fd) to be in `$PATH`.

## Wire up

Add to `~/.claude/settings.json`:

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
