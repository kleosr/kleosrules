pub fn strip_strings_for_scan(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let bytes = text.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let c = bytes[i] as char;
        if c == '"' || c == '\'' {
            let quote = c;
            out.push(' ');
            i += 1;
            while i < bytes.len() {
                let ch = bytes[i] as char;
                if ch == '\\' {
                    out.push(' ');
                    i += 2;
                    continue;
                }
                if ch == quote {
                    out.push(' ');
                    i += 1;
                    break;
                }
                out.push(' ');
                i += 1;
            }
            continue;
        }
        if c == '`' {
            out.push(' ');
            i += 1;
            while i < bytes.len() && bytes[i] != b'`' {
                if bytes[i] == b'\\' {
                    out.push(' ');
                    i += 2;
                    continue;
                }
                out.push(' ');
                i += 1;
            }
            if i < bytes.len() {
                out.push(' ');
                i += 1;
            }
            continue;
        }
        out.push(c);
        i += 1;
    }
    out
}

fn is_js_directive(rest: &str) -> bool {
    let r = rest.trim_start();
    r.starts_with("eslint")
        || r.starts_with("@ts-")
        || r.starts_with("ts-")
        || r.starts_with("prettier")
        || r.starts_with("istanbul")
        || r.starts_with("biome")
        || r.starts_with("v8 ignore")
        || r.starts_with("pragma")
}

fn is_hash_directive(rest: &str) -> bool {
    let r = rest.trim_start();
    r.starts_with("type: ignore")
        || r.starts_with("noqa")
        || r.starts_with("pragma:")
        || r.starts_with("pylint:")
        || r.starts_with("mypy:")
        || r.starts_with("fmt:")
        || r.starts_with("ruff:")
        || r.starts_with("!")
}

pub fn has_prose(text: &str) -> bool {
    if text.is_empty() {
        return false;
    }
    let scrubbed = strip_strings_for_scan(text);
    let bytes = scrubbed.as_bytes();
    let mut i = 0;
    let mut line_start = true;
    let mut hash_line_idx = 0usize;
    while i < bytes.len() {
        let c = bytes[i];
        if c == b'\n' {
            line_start = true;
            hash_line_idx += 1;
            i += 1;
            continue;
        }
        if c == b'/' && i + 1 < bytes.len() && bytes[i + 1] == b'/' {
            let rest = &scrubbed[i + 2..];
            let end = rest.find(['\n', '\r']).unwrap_or(rest.len());
            let body = rest[..end].trim();
            if !body.is_empty() && !is_js_directive(body) {
                return true;
            }
            i += 2 + end;
            continue;
        }
        if c == b'/' && i + 1 < bytes.len() && bytes[i + 1] == b'*' {
            return true;
        }
        if c == b'#' && line_start {
            let rest = &scrubbed[i + 1..];
            let end = rest.find(['\n', '\r']).unwrap_or(rest.len());
            let body = rest[..end].trim();
            if hash_line_idx == 0 && body.starts_with('!') {
                i += 1 + end;
                line_start = false;
                continue;
            }
            if !body.is_empty() && !is_hash_directive(body) {
                return true;
            }
            i += 1 + end;
            continue;
        }
        if c != b' ' && c != b'\t' {
            line_start = false;
        }
        i += 1;
    }
    false
}
