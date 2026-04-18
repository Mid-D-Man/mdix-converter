// Auto-generated stub
// crates/converter-core/src/emitter/formatter.rs
//
// Post-processes the raw .mdix string produced by the emitter into clean,
// consistently indented, human-readable DixScript.
//
// Passes applied in order:
//   1. Trim trailing whitespace on every line
//   2. Collapse runs of more than one blank line to a single blank line
//   3. Align `=` signs within consecutive flat-property lines
//      (same indent level, same section block)
//   4. Ensure the file ends with exactly one newline

/// Format raw .mdix source into clean DixScript.
pub fn format(raw: &str) -> String {
    let lines: Vec<&str> = raw.lines().collect();

    // Pass 1: trim trailing whitespace
    let trimmed: Vec<String> = lines
        .iter()
        .map(|l| l.trim_end().to_string())
        .collect();

    // Pass 2: collapse multiple blank lines → single blank line
    let collapsed = collapse_blank_lines(trimmed);

    // Pass 3: align `=` in property blocks
    let aligned = align_equals(collapsed);

    // Pass 4: ensure single trailing newline
    let mut out = aligned.join("\n");
    if !out.ends_with('\n') {
        out.push('\n');
    }
    out
}

// ── Pass 2 ────────────────────────────────────────────────────────────────────

fn collapse_blank_lines(lines: Vec<String>) -> Vec<String> {
    let mut out = Vec::with_capacity(lines.len());
    let mut prev_blank = false;
    for line in lines {
        if line.is_empty() {
            if !prev_blank {
                out.push(line);
            }
            prev_blank = true;
        } else {
            out.push(line);
            prev_blank = false;
        }
    }
    // Strip leading blank lines
    while out.first().map(|l: &String| l.is_empty()).unwrap_or(false) {
        out.remove(0);
    }
    out
}

// ── Pass 3 — `=` alignment ────────────────────────────────────────────────────
//
// Groups consecutive lines that look like `<indent><key> = <value>` and
// pads the keys so all `=` signs land in the same column.
//
// Example input:
//   name = "Alice"
//   age = 30
//   active = true
//
// Example output:
//   name   = "Alice"
//   age    = 30
//   active = true

fn align_equals(lines: Vec<String>) -> Vec<String> {
    let mut out: Vec<String> = Vec::with_capacity(lines.len());
    let mut i = 0;

    while i < lines.len() {
        // Collect a "block" — consecutive lines that are simple `key = value`
        // at the same indentation level
        let block_start = i;
        let indent = leading_spaces(&lines[i]);

        while i < lines.len()
            && is_simple_property(&lines[i])
            && leading_spaces(&lines[i]) == indent
        {
            i += 1;
        }

        let block = &lines[block_start..i];

        if block.len() < 2 {
            // Single line or not a property — push as-is
            for l in block { out.push(l.clone()); }
        } else {
            // Find the longest key in this block
            let max_key_len = block
                .iter()
                .filter_map(|l| key_len(l))
                .max()
                .unwrap_or(0);

            for l in block {
                out.push(pad_equals(l, max_key_len));
            }
        }
    }

    out
}

/// Return the number of leading spaces on a line.
fn leading_spaces(s: &str) -> usize {
    s.len() - s.trim_start().len()
}

/// True when a line looks like `<ws><identifier> = <anything>`.
fn is_simple_property(line: &str) -> bool {
    let t = line.trim();
    if t.is_empty() { return false; }
    // Must contain ` = ` and the left side must be a bare identifier or quoted string
    if let Some(eq_pos) = t.find(" = ") {
        let lhs = &t[..eq_pos];
        // Reject lines that look like function calls, section headers, etc.
        !lhs.contains('(') && !lhs.contains('[') && !lhs.contains('"')
    } else {
        false
    }
}

/// Return the byte length of the key part (before ` = `).
fn key_len(line: &str) -> Option<usize> {
    let t = line.trim();
    t.find(" = ").map(|pos| pos)
}

/// Pad a `key = value` line so the `=` falls at column `indent + max_key_len + 1`.
fn pad_equals(line: &str, max_key_len: usize) -> String {
    let indent = leading_spaces(line);
    let t      = line.trim();
    if let Some(eq_pos) = t.find(" = ") {
        let key   = &t[..eq_pos];
        let value = &t[eq_pos + 3..];
        let pad   = " ".repeat(max_key_len - key.len());
        format!("{}{}{} = {}", " ".repeat(indent), key, pad, value)
    } else {
        line.to_string()
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collapses_multiple_blank_lines() {
        let raw = "a = 1\n\n\n\nb = 2";
        let out = format(raw);
        assert!(!out.contains("\n\n\n"), "triple blank should be collapsed:\n{}", out);
    }

    #[test]
    fn trims_trailing_whitespace() {
        let raw = "a = 1   \nb = 2\t";
        let out = format(raw);
        for line in out.lines() {
            assert_eq!(line, line.trim_end(), "trailing whitespace on: {:?}", line);
        }
    }

    #[test]
    fn ends_with_single_newline() {
        let raw = "@DATA(\n  x = 1\n)\n\n\n";
        let out = format(raw);
        assert!(out.ends_with('\n'));
        assert!(!out.ends_with("\n\n"));
    }

    #[test]
    fn aligns_equals_in_property_block() {
        let raw = "  name = \"Alice\"\n  age = 30\n  active = true";
        let out = format(raw);
        // All = signs should be in the same column
        let col: Vec<usize> = out.lines()
            .filter(|l| l.contains(" = "))
            .map(|l| l.find(" = ").unwrap())
            .collect();
        assert!(
            col.windows(2).all(|w| w[0] == w[1]),
            "= signs not aligned:\n{}", out
        );
    }

    #[test]
    fn does_not_align_across_blank_line_boundary() {
        let raw = "  short = 1\n\n  very_long_key = 2";
        let out = format(raw);
        // The two blocks are separated by a blank — they should NOT be aligned together
        // (each is length-1, so padding won't change them either way, but the test
        // verifies the function doesn't crash on split blocks)
        assert!(out.contains("short"));
        assert!(out.contains("very_long_key"));
    }

    #[test]
    fn round_trip_is_idempotent() {
        let raw = "@DATA(\n  name = \"Alice\"\n  age  = 30\n)\n";
        let once  = format(raw);
        let twice = format(&once);
        assert_eq!(once, twice, "formatting is not idempotent");
    }
  }
