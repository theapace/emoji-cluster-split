// Full Unicode grapheme clustering (UAX #29) covers every script and is a lot
// of machinery. This tool only needs to get emoji right, so it uses a smaller
// state machine built around the codepoints that actually change what an
// emoji "means": ZWJ joins, skin tone modifiers, variation selectors, keycap
// combining marks, regional indicator pairs (flags), and tag sequences
// (subdivision flags like England/Scotland).

const ZWJ: char = '\u{200D}';
const VARIATION_SELECTOR_TEXT: char = '\u{FE0E}';
const VARIATION_SELECTOR_EMOJI: char = '\u{FE0F}';
const COMBINING_ENCLOSING_KEYCAP: char = '\u{20E3}';

fn is_regional_indicator(c: char) -> bool {
    matches!(c as u32, 0x1F1E6..=0x1F1FF)
}

fn is_skin_tone_modifier(c: char) -> bool {
    matches!(c as u32, 0x1F3FB..=0x1F3FF)
}

fn is_variation_selector(c: char) -> bool {
    c == VARIATION_SELECTOR_TEXT || c == VARIATION_SELECTOR_EMOJI
}

fn is_keycap_mark(c: char) -> bool {
    c == COMBINING_ENCLOSING_KEYCAP
}

// Trailing marks that attach to whatever base or joined element came before
// them without starting a new cluster on their own.
fn is_trailing_modifier(c: char) -> bool {
    is_skin_tone_modifier(c) || is_variation_selector(c) || is_keycap_mark(c)
}

// The tag characters block, used to spell out region codes after a tag base
// (U+1F3F4 for subdivision flags) and terminated by the cancel tag U+E007F.
fn is_tag_char(c: char) -> bool {
    matches!(c as u32, 0xE0000..=0xE007F)
}

/// Splits `input` into the emoji sequences a reader would perceive as single
/// units, plus every other character as its own one-codepoint cluster.
pub fn split_clusters(input: &str) -> Vec<String> {
    let chars: Vec<char> = input.chars().collect();
    let mut clusters = Vec::new();
    let mut i = 0;

    while i < chars.len() {
        let mut cluster = String::new();
        let base = chars[i];
        cluster.push(base);
        i += 1;

        if is_regional_indicator(base) {
            // Flags are exactly two regional indicators. A lone trailing one
            // (truncated input, or just not a flag) stays on its own.
            if i < chars.len() && is_regional_indicator(chars[i]) {
                cluster.push(chars[i]);
                i += 1;
            }
        } else {
            while i < chars.len() && is_trailing_modifier(chars[i]) {
                cluster.push(chars[i]);
                i += 1;
            }
            while i < chars.len() && is_tag_char(chars[i]) {
                cluster.push(chars[i]);
                i += 1;
            }
        }

        // A ZWJ glues on another emoji element (which may itself carry a
        // skin tone or variation selector), and the result can chain: family
        // sequences join three or more base emoji this way.
        while i < chars.len() && chars[i] == ZWJ {
            cluster.push(chars[i]);
            i += 1;
            if i >= chars.len() {
                break;
            }
            cluster.push(chars[i]);
            i += 1;
            while i < chars.len() && is_trailing_modifier(chars[i]) {
                cluster.push(chars[i]);
                i += 1;
            }
        }

        clusters.push(cluster);
    }

    clusters
}

/// Formats every codepoint in `cluster` as `U+XXXX`, for display.
pub fn codepoints(cluster: &str) -> Vec<String> {
    cluster.chars().map(|c| format!("U+{:04X}", c as u32)).collect()
}

// Cluster text can itself contain quotes, backslashes, or control characters
// (a bare ZWJ or tag char would be invisible but still needs escaping to keep
// the output valid JSON), so this can't just be wrapped in quotes as-is.
fn escape_json(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out
}

/// Renders clusters as a JSON array of `{"text", "codepoints"}` objects, for
/// piping into scripts instead of parsing the human-readable table output.
pub fn to_json(clusters: &[String]) -> String {
    let mut out = String::from("[\n");
    for (i, cluster) in clusters.iter().enumerate() {
        let codes: Vec<String> = codepoints(cluster)
            .iter()
            .map(|c| format!("\"{}\"", c))
            .collect();
        out.push_str(&format!(
            "  {{\"text\": \"{}\", \"codepoints\": [{}]}}",
            escape_json(cluster),
            codes.join(", ")
        ));
        if i + 1 < clusters.len() {
            out.push(',');
        }
        out.push('\n');
    }
    out.push(']');
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escape_json_handles_quotes_backslashes_and_control_chars() {
        assert_eq!(escape_json("a\"b"), "a\\\"b");
        assert_eq!(escape_json("a\\b"), "a\\\\b");
        assert_eq!(escape_json("a\nb"), "a\\nb");
        assert_eq!(escape_json("\u{200D}"), "\\u200d");
    }

    #[test]
    fn escape_json_leaves_plain_emoji_untouched() {
        assert_eq!(escape_json("👋🏽"), "👋🏽");
    }
}
