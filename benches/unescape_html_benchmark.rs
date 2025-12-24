// benches/unescape_html_benchmark.rs

use criterion::{Criterion, black_box, criterion_group, criterion_main};
use std::time::Duration;

// Current implementation from src/syntax.rs
fn unescape_html_entities_current(s: &str) -> String {
    s.replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&amp;", "&")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&apos;", "'")
}

// ──────────────────────────────────────────────────────────────────────
// Fast, single‑pass HTML entity unescaper (no external crate)
// ──────────────────────────────────────────────────────────────────────
pub fn unescape_html_entities_single_pass(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let bytes = input.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] != b'&' {
            out.push(bytes[i] as char);
            i += 1;
            continue;
        }
        // Verify that we have at least ';'
        if i + 1 >= bytes.len() || bytes[i + 1] != b';' {
            out.push('&');
            i += 1;
            continue;
        }
        // Find endpoint of the entity
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
            s if s.starts_with("&#x") => {
                if let Ok(v) = u32::from_str_radix(&s[4..], 16) {
                    Some(char::from_u32(v).unwrap_or('?'))
                } else {
                    None
                }
            }
            s if s.starts_with("&#") => {
                if let Ok(v) = u32::from_str_radix(&s[2..], 10) {
                    Some(char::from_u32(v).unwrap_or('?'))
                } else {
                    None
                }
            }
            _ => None,
        };
        match decoded {
            Some(ch) => out.push(ch),
            None => {
                // Unknown entity → copy verbatim including '&' and ';'
                out.push_str(&input[i..=end]);
                i = end + 1;
                continue;
            }
        }
        i = end + 1;
    }
    out
}

// Manual implementation using a state machine (more efficient)
fn unescape_html_entities_manual(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '&' {
            // Check for known entities
            let mut entity = String::new();
            entity.push('&');
            while let Some(&next) = chars.peek() {
                entity.push(next);
                chars.next();
                if next == ';' {
                    break;
                }
                // Limit entity length to avoid infinite loop on malformed input
                if entity.len() > 10 {
                    break;
                }
            }

            match entity.as_str() {
                "&lt;" => result.push('<'),
                "&gt;" => result.push('>'),
                "&amp;" => result.push('&'),
                "&quot;" => result.push('"'),
                "&#39;" => result.push('\''),
                "&apos;" => result.push('\''),
                _ => result.push_str(&entity), // Unknown entity, leave as is
            }
        } else {
            result.push(c);
        }
    }

    result
}

// Using the html-escape library (if available)
// Note: This function will only be compiled if the html-escape crate is added as a dependency.
// We assume it is available for the benchmark.
fn unescape_html_entities_library(s: &str) -> String {
    html_escape::decode_html_entities(s).to_string()
}

// Test strings for benchmarking
const TEST_STRINGS: &[&str] = &[
    // No entities
    "Hello, world!",
    // Single entity
    "&lt;div&gt;",
    // Multiple entities
    "&lt;div&gt;&amp;&quot;&#39;&apos;&lt;/div&gt;",
    // Mixed content
    "Hello &lt;world&gt; &amp; welcome!",
    // Long string with many entities
    "&lt;html&gt;&lt;head&gt;&lt;title&gt;Test&lt;/title&gt;&lt;/head&gt;&lt;body&gt;&lt;p&gt;Hello &amp; welcome!&lt;/p&gt;&lt;/body&gt;&lt;/html&gt;",
    // Edge case: entity at the beginning
    "&lt;start",
    // Edge case: entity at the end
    "end&gt;",
    // Edge case: incomplete entity
    "&lt;div &gt;",
    // Edge case: unknown entity
    "&unknown;",
    // Real-world example from markdown code block
    "fn main() {\n    println!(\"Hello &amp; welcome!\");\n}",
];

// Helper function to run benchmarks for a given function
fn bench_unescape_function(c: &mut Criterion, name: &str, f: fn(&str) -> String) {
    let mut group = c.benchmark_group(name);
    group.measurement_time(Duration::from_secs(5));

    for (i, &input) in TEST_STRINGS.iter().enumerate() {
        group.bench_function(format!("test_string_{}", i), |b| {
            b.iter(|| f(black_box(input)))
        });
    }

    group.finish();
}

// Main benchmark function
fn benchmark_unescape(c: &mut Criterion) {
    // Benchmark current implementation
    bench_unescape_function(
        c,
        "unescape_html_entities_current",
        unescape_html_entities_current,
    );

    // Benchmark single-pass implementation
    bench_unescape_function(
        c,
        "unescape_html_entities_single_pass",
        unescape_html_entities_single_pass,
    );

    // Benchmark manual implementation
    bench_unescape_function(
        c,
        "unescape_html_entities_manual",
        unescape_html_entities_manual,
    );

    // Benchmark library implementation (if available)
    // Note: We conditionally compile this benchmark only if the html-escape crate is available.
    // Since we cannot conditionally compile in this file without Cargo.toml changes, we assume it's available.
    // If the library is not available, comment out this line.
    bench_unescape_function(
        c,
        "unescape_html_entities_library",
        unescape_html_entities_library,
    );
}

// Criterion setup
criterion_group!(benches, benchmark_unescape);
criterion_main!(benches);
