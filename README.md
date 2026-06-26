# fast-bash

Claude Code `PreToolUse` hook with three behaviors:

1. **`grep -r` → `rg`** with auto-excludes (`node_modules`, `dist`, `target`, …) — 10–100× faster on real repos. Incompatible flags (`-P`, `-G`) pass through untouched.
2. **`find` hint** — logs `[fast-bash] hint: consider using fd` to stderr. No rewrite, no breakage.
3. **Safety blocks** — hard-blocks `rm -rf /`, `rm -rf .`, `rm -rf /etc`, … and `git push --force` (allows `--force-with-lease`).

## Examples

| Command | Result |
|---------|--------|
| `grep -rn "foo" .` | → `rg -n "foo" .` + auto-excludes |
| `grep -rn --include="*.rs" foo .` | → `rg -g "*.rs" foo .` |
| `grep -rP 'foo(?=bar)' .` | pass-through (PCRE flag) |
| `find . -name "*.ts"` | hint to stderr, runs as-is |
| `rm -rf /` | **blocked** |
| `git push --force` | **blocked** (use `--force-with-lease`) |

## Setup

<table>
<tr>
<th></th>
<th>Homebrew</th>
<th>Build Manually</th>
</tr>
<tr>
<td><b>Install</b></td>
<td>

```bash
brew tap MerzoukeMansouri/homebrew
brew install MerzoukeMansouri/homebrew/fast-bash
```

</td>
<td>

```bash
cargo install fast-bash
```

</td>
</tr>
<tr>
<td><b>Update</b></td>
<td>

```bash
brew upgrade MerzoukeMansouri/homebrew/fast-bash
```

</td>
<td>

```bash
cargo install fast-bash
```

</td>
</tr>
</table>

### `~/.claude/settings.json`

<table>
<tr>
<th>Homebrew</th>
<th>Build Manually</th>
</tr>
<tr>
<td>

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

</td>
<td>

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

</td>
</tr>
</table>

## Install via Claude Code

Paste this prompt into Claude Code:

```
@https://raw.githubusercontent.com/MerzoukeMansouri/pre-tool-hook/main/doc/install.md install fast-bash hook following the guide
```

## License

MIT
