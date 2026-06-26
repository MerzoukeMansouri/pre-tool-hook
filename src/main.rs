use std::io::{self, Read};

const DEFAULT_EXCLUDES: &[&str] = &[
    "node_modules",
    "dist",
    ".next",
    "build",
    "out",
    "coverage",
    "target",
    ".turbo",
    "__pycache__",
    "vendor",
];

// grep flags whose regex syntax rg doesn't support — pass through untouched
const INCOMPATIBLE_GREP_FLAGS: &[char] = &[
    'G', // basic regex (BRE)
    'P', // Perl-compatible regex (PCRE)
];

// ── shared ────────────────────────────────────────────────────────────────

fn tokenize(s: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut chars = s.chars().peekable();
    loop {
        while chars.peek().map(|c: &char| c.is_whitespace()) == Some(true) {
            chars.next();
        }
        match chars.peek() {
            None => break,
            Some(&q @ ('"' | '\'')) => {
                chars.next();
                let mut tok = String::new();
                for c in &mut chars {
                    if c == q {
                        break;
                    }
                    tok.push(c);
                }
                tokens.push(tok);
            }
            _ => {
                let mut tok = String::new();
                for c in &mut chars {
                    if c.is_whitespace() {
                        break;
                    }
                    tok.push(c);
                }
                if !tok.is_empty() {
                    tokens.push(tok);
                }
            }
        }
    }
    tokens
}

fn exclude_flags_rg(original_cmd: &str) -> String {
    DEFAULT_EXCLUDES
        .iter()
        .filter(|&&dir| !original_cmd.contains(dir))
        .map(|dir| format!("-g '!{}' ", dir))
        .collect()
}

// ── grep → rg ─────────────────────────────────────────────────────────────

// Returns Some(rewritten) or None if the command should pass through unchanged.
fn rewrite_grep(cmd: &str) -> Option<String> {
    let mut out = String::with_capacity(cmd.len());
    let mut rest = cmd;
    let mut rewrote = false;

    while !rest.is_empty() {
        let Some(pos) = rest.find("grep ") else {
            out.push_str(rest);
            break;
        };
        let before = &rest[..pos];
        let at_boundary =
            before.is_empty() || before.ends_with(|c: char| !c.is_alphanumeric() && c != '_');
        if !at_boundary {
            out.push_str(&rest[..pos + 1]);
            rest = &rest[pos + 1..];
            continue;
        }

        out.push_str(before);
        rest = &rest[pos + "grep ".len()..];

        let trimmed = rest.trim_start_matches(' ');
        let spaces = rest.len() - trimmed.len();
        rest = trimmed;

        if !rest.starts_with('-') {
            out.push_str("grep ");
            out.push_str(&" ".repeat(spaces));
            continue;
        }

        rest = &rest[1..];
        let flag_end = rest
            .find(|c: char| !c.is_alphabetic())
            .unwrap_or(rest.len());
        let flags = &rest[..flag_end];
        rest = &rest[flag_end..];

        let has_r = flags.chars().any(|f| f == 'r' || f == 'R');
        let has_incompatible = flags.chars().any(|f| INCOMPATIBLE_GREP_FLAGS.contains(&f));

        if !has_r || has_incompatible {
            out.push_str("grep -");
            out.push_str(flags);
            if !rest.is_empty() {
                out.push(' ');
                rest = rest.trim_start_matches(' ');
            }
            continue;
        }

        let extra: String = flags.chars().filter(|f| !"rRnN".contains(*f)).collect();
        if extra.is_empty() {
            out.push_str("rg ");
        } else {
            out.push_str("rg -");
            out.push_str(&extra);
            out.push(' ');
        }
        rest = rest.trim_start_matches(' ');
        rewrote = true;
    }

    // --include="*.ext" → -g "*.ext"
    let out = if out.contains("--include=") {
        let mut result = String::with_capacity(out.len());
        let mut s = out.as_str();
        while !s.is_empty() {
            if let Some(pos) = s.find("--include=") {
                result.push_str(&s[..pos]);
                result.push_str("-g ");
                s = &s[pos + "--include=".len()..];
                if s.starts_with(['\'', '"']) {
                    let q = s.chars().next().unwrap();
                    s = &s[1..];
                    let end = s.find(q).unwrap_or(s.len());
                    result.push('"');
                    result.push_str(&s[..end]);
                    result.push('"');
                    s = if end < s.len() { &s[end + 1..] } else { "" };
                } else {
                    let end = s.find(char::is_whitespace).unwrap_or(s.len());
                    result.push_str(&s[..end]);
                    s = &s[end..];
                }
            } else {
                result.push_str(s);
                break;
            }
        }
        result
    } else {
        out
    };

    if rewrote {
        let excl = exclude_flags_rg(cmd);
        let rewritten = if let Some(pos) = out.find("rg ") {
            format!("{}{}{}", &out[..pos + 3], excl, &out[pos + 3..])
        } else {
            out
        };
        Some(rewritten)
    } else {
        None
    }
}

// ── safety checks ─────────────────────────────────────────────────────────

fn safety_block(cmd: &str) -> Option<&'static str> {
    if is_catastrophic_rm(cmd) {
        return Some("Blocked: rm -rf targeting root/home/cwd. Specify an explicit safe path.");
    }
    if is_force_push(cmd) {
        return Some(
            "Blocked: git push --force/-f. Use --force-with-lease to avoid overwriting upstream commits.",
        );
    }
    None
}

fn is_catastrophic_rm(cmd: &str) -> bool {
    if !cmd.contains("rm") {
        return false;
    }
    let tokens = tokenize(cmd);
    let mut i = 0;
    while i < tokens.len() {
        if tokens[i] == "rm" {
            i += 1;
            let (mut has_r, mut has_f) = (false, false);
            while i < tokens.len() && tokens[i].starts_with('-') && tokens[i] != "--" {
                let f = &tokens[i][1..];
                if f.contains('r') || f.contains('R') {
                    has_r = true;
                }
                if f.contains('f') {
                    has_f = true;
                }
                i += 1;
            }
            if has_r && has_f && i < tokens.len() {
                let t = tokens[i].as_str();
                let norm = if t == "/" {
                    "/"
                } else {
                    t.trim_end_matches('/')
                };
                // ponytail: only exact dangerous roots, not subdirs like /home/user/project
                return matches!(
                    norm,
                    "/" | "~" | "." | "/usr" | "/etc" | "/var" | "/bin" | "/sbin" | "/root"
                );
            }
        }
        i += 1;
    }
    false
}

fn is_force_push(cmd: &str) -> bool {
    if !cmd.contains("git") || !cmd.contains("push") {
        return false;
    }
    let tokens = tokenize(cmd);
    let mut i = 0;
    while i < tokens.len() {
        if tokens[i] == "git" {
            i += 1;
            while i < tokens.len() && tokens[i].starts_with('-') {
                i += 1;
            }
            if i < tokens.len() && tokens[i] == "push" {
                i += 1;
                while i < tokens.len() {
                    match tokens[i].as_str() {
                        "--force-with-lease" | "--force-if-includes" => return false,
                        "--force" | "-f" => return true,
                        _ => {}
                    }
                    i += 1;
                }
            }
        }
        i += 1;
    }
    false
}

// ── main ──────────────────────────────────────────────────────────────────

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let Ok(v) = serde_json::from_str::<serde_json::Value>(&input) else {
        return;
    };
    if v["tool_name"] != "Bash" {
        return;
    }

    let cmd = match v["tool_input"]["command"].as_str() {
        Some(s) => s.to_owned(),
        None => return,
    };

    // safety first
    if let Some(msg) = safety_block(&cmd) {
        println!("{}", serde_json::json!({"type": "block", "message": msg}));
        return;
    }

    // find hint (no rewrite — fd syntax differs too much)
    if cmd.trim().starts_with("find ") {
        eprintln!("[fast-bash] hint: consider using fd instead of find");
    }

    // grep → rg rewrite
    if let Some(new_cmd) = rewrite_grep(&cmd) {
        eprintln!("[fast-bash] rewritten: grep → rg");
        println!(
            "{}",
            serde_json::json!({
                "type": "tool_input",
                "tool_input": { "command": new_cmd }
            })
        );
    }
}

// ── tests ─────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grep_basic() {
        let all_excl: String = DEFAULT_EXCLUDES
            .iter()
            .map(|d| format!("-g '!{}' ", d))
            .collect();
        let r = rewrite_grep(r#"grep -rn "foo" ."#).unwrap();
        assert!(r.starts_with("rg "));
        assert!(r.contains(&all_excl));
        assert!(r.contains(r#""foo" ."#));
    }

    #[test]
    fn grep_no_r_untouched() {
        assert!(rewrite_grep("grep -n foo .").is_none());
    }

    #[test]
    fn grep_explicit_dir_not_excluded() {
        let r = rewrite_grep("grep -rn foo node_modules/").unwrap();
        assert!(r.contains("rg"));
        assert!(!r.contains("-g '!node_modules'"));
        assert!(r.contains("-g '!dist'"));
    }

    #[test]
    fn grep_include_rewrite() {
        let r = rewrite_grep(r#"grep -rn --include="*.rs" foo ."#).unwrap();
        assert!(r.contains(r#"-g "*.rs""#));
        assert!(r.contains("rg"));
    }

    #[test]
    fn grep_incompatible_flags_pass_through() {
        assert!(rewrite_grep("grep -rP 'foo(?=bar)' .").is_none());
        assert!(rewrite_grep("grep -rG 'foo' .").is_none());
    }

    #[test]
    fn rm_root_blocked() {
        assert!(safety_block("rm -rf /").is_some());
        assert!(safety_block("rm -rf /etc").is_some());
        assert!(safety_block("rm -rf ./some/path").is_none());
        assert!(safety_block("rm -rf /home/user/project").is_none());
    }

    #[test]
    fn force_push_blocked() {
        assert!(safety_block("git push --force").is_some());
        assert!(safety_block("git push -f origin main").is_some());
        assert!(safety_block("git push --force-with-lease").is_none());
    }
}
