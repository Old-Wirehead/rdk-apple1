//! Sources for the xa (xa65) cross-assembler, such as Dave Schmenk's
//! Apple1-Slideshow (2014): `#define`, `#include`, `*=`, `.BYT`, `.ASC`,
//! `/* */` comments and labels in column 0 without a colon. They are put
//! into the ca65 syntax of the built-in assembler, line for line (the
//! errors keep their line numbers).

/// xa source: a `#include` / `#define` line, or a column-0 label without a
/// colon followed by a mnemonic.
pub fn is_xa(text: &str) -> bool {
    text.lines().any(|l| {
        let t = l.trim_start();
        t.starts_with("#include") || t.starts_with("#define")
    })
}

/// The line-for-line ca65 version of an xa text.
pub fn to_ca65(text: &str) -> String {
    let mut out = String::with_capacity(text.len() + 256);
    let mut in_comment = false;
    for (i, line) in text.lines().enumerate() {
        if i > 0 {
            out.push('\n');
        }
        // /* ... */ comments (several lines)
        let mut l = String::new();
        let mut rest = line;
        loop {
            if in_comment {
                match rest.find("*/") {
                    Some(e) => {
                        rest = &rest[e + 2..];
                        in_comment = false;
                    }
                    None => break,
                }
            } else {
                match find_outside_string(rest, "/*") {
                    Some(s) => {
                        l.push_str(&rest[..s]);
                        rest = &rest[s + 2..];
                        in_comment = true;
                    }
                    None => {
                        l.push_str(rest);
                        break;
                    }
                }
            }
        }
        out.push_str(&line_to_ca65(&l));
    }
    out.push('\n');
    out
}

fn find_outside_string(s: &str, pat: &str) -> Option<usize> {
    let mut q = false;
    let b = s.as_bytes();
    for i in 0..b.len() {
        if b[i] == b'"' {
            q = !q;
        } else if !q && s[i..].starts_with(pat) {
            return Some(i);
        } else if !q && b[i] == b';' {
            return None;
        }
    }
    None
}

fn directive(word: &str) -> Option<&'static str> {
    Some(match word.to_ascii_uppercase().as_str() {
        ".BYT" | ".BYTE" | ".ASC" | ".ASCII" | ".AASC" => ".byte",
        ".WORD" | ".WOR" => ".word",
        ".DSB" => ".res",
        _ => return None,
    })
}

fn line_to_ca65(l: &str) -> String {
    let t = l.trim();
    if let Some(r) = t.strip_prefix("#define") {
        let mut it = r.trim().splitn(2, char::is_whitespace);
        let (n, v) = (it.next().unwrap_or(""), it.next().unwrap_or("").trim());
        return if v.is_empty() { format!("{n} = 1") } else { format!("{n} = {v}") };
    }
    if let Some(r) = t.strip_prefix("#include") {
        return format!(".include {}", r.trim());
    }
    if t.starts_with('#') {
        // other preprocessor lines (#if...): kept as comments
        return format!("; {t}");
    }
    if let Some(r) = t.strip_prefix('*') {
        if let Some(v) = r.trim_start().strip_prefix('=') {
            return format!(".org {}", v.trim());
        }
    }
    // column 0: a label (no colon in xa)
    let (label, body) = if !l.starts_with(char::is_whitespace) && !t.is_empty() && !t.starts_with(';') && !t.starts_with('.') {
        let end = t.find(|c: char| c.is_whitespace() || c == ';').unwrap_or(t.len());
        let name = t[..end].trim_end_matches(':');
        (Some(name.to_string()), t[end..].trim_start())
    } else {
        (None, t)
    };
    let body = {
        let end = body.find(char::is_whitespace).unwrap_or(body.len());
        match directive(&body[..end]) {
            Some(d) => format!("{d} {}", body[end..].trim()),
            None => body.to_string(),
        }
    };
    match label {
        Some(n) if body.is_empty() => format!("{n}:"),
        Some(n) => format!("{n}: {body}"),
        None if body.is_empty() => String::new(),
        None => format!("        {body}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn translates_the_slideshow_constructs() {
        let src = "/*\n * x\n */\n\n#define\tPTR\t$06\n\t*=$0280\nENTRY\tLDA\t#>PTR\nGREY\n\t.BYT\t$A0, $AE\n\t/*.ASC\t\"x\" */\n#include \"woz.rle\"\n\t.ASC\t\"WOZ \"\nL2\tRTS\t; fin\n";
        assert!(is_xa(src));
        let t = to_ca65(src);
        let l: Vec<&str> = t.lines().collect();
        assert_eq!(l.len(), src.lines().count());
        assert_eq!(&l[..4], ["", "", "", ""]);
        assert_eq!(l[4], "PTR = $06");
        assert_eq!(l[5], ".org $0280");
        assert_eq!(l[6], "ENTRY: LDA\t#>PTR");
        assert_eq!(l[7], "GREY:");
        assert_eq!(l[8], "        .byte $A0, $AE");
        assert_eq!(l[9], "");
        assert_eq!(l[10], ".include \"woz.rle\"");
        assert_eq!(l[11], "        .byte \"WOZ \"");
        assert_eq!(l[12], "L2: RTS\t; fin");
    }
}
