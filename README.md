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
<td><b>Dependencies</b></td>
<td>included</td>
<td>

```bash
brew install ripgrep fd
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
