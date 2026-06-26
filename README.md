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

### Homebrew (recommended)

```bash
brew tap MerzoukeMansouri/homebrew
brew install MerzoukeMansouri/homebrew/fast-bash
```

### From crates.io

```bash
cargo install fast-bash
```

### Dependencies

Requires [ripgrep](https://github.com/BurntSushi/ripgrep) and [fd](https://github.com/sharkdp/fd):

```bash
brew install ripgrep fd
```

### Update

```bash
brew update
brew upgrade MerzoukeMansouri/homebrew/fast-bash
```

## Wire up

Add to `~/.claude/settings.json`:

**Homebrew / crates.io** (binary in `$PATH`):

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

**Manual build** (full path to binary):

```json
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "Bash",
        "hooks": [
          {
            "type": "command",
            "command": "/path/to/pre-tool-hook/target/release/fast-bash"
          }
        ]
      }
    ]
  }
}
```

## License

MIT
