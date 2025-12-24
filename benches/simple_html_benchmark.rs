use criterion::{Criterion, black_box, criterion_group, criterion_main};

// Current implementation from src/syntax.rs
fn unescape_html_entities_current(s: &str) -> String {
    s.replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&amp;", "&")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&apos;", "'")
}

// Single-pass implementation
fn unescape_html_entities_single_pass(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let bytes = input.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] != b'&' {
            out.push(bytes[i] as char);
            i += 1;
            continue;
        }
        // Find endpoint of the entity
        if i + 1 >= bytes.len() {
            out.push('&');
            i += 1;
            continue;
        }
        let mut end = i + 1;
        while end < bytes.len() && bytes[end] != b';' {
            end += 1;
        }
        if end == bytes.len() {
            out.push('&');
            i += 1;
            continue;
        }
        let entity = &input[i + 1..end];
        let decoded = match entity {
            "lt" => Some('<'),
            "gt" => Some('>'),
            "amp" => Some('&'),
            "quot" => Some('"'),
            "apos" => Some('\''),
            _ => None,
        };
        match decoded {
            Some(ch) => out.push(ch),
            None => {
                // Unknown entity → copy verbatim including '&' and ';'
                out.push_str(&input[i..=end]);
            }
        }
        i = end + 1;
    }
    out
}

// Using the html-escape library
fn unescape_html_entities_library(s: &str) -> String {
    html_escape::decode_html_entities(s).to_string()
}

fn bench_current(c: &mut Criterion) {
    c.bench_function("current", |b| {
        b.iter(|| {
            unescape_html_entities_current(black_box("&lt;div&gt;Hello &amp; welcome!&lt;/div&gt;"))
        })
    });
}

fn bench_single_pass(c: &mut Criterion) {
    c.bench_function("single_pass", |b| {
        b.iter(|| {
            unescape_html_entities_single_pass(black_box(
                "&lt;div&gt;Hello &amp; welcome!&lt;/div&gt;",
            ))
        })
    });
}

fn bench_library(c: &mut Criterion) {
    c.bench_function("library", |b| {
        b.iter(|| {
            unescape_html_entities_library(black_box("&lt;div&gt;Hello &amp; welcome!&lt;/div&gt;"))
        })
    });
}

criterion_group!(benches, bench_current, bench_single_pass, bench_library);
criterion_main!(benches);
