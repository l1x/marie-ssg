# TOML Parser Migration: basic-toml → toml

This document compares the `basic-toml` and `toml` crates and outlines the migration path.

## Why Migrate?

| Aspect      | basic-toml             | toml              |
| ----------- | ---------------------- | ----------------- |
| Performance | ~20µs (typical config) | ~10µs (2x faster) |
| Maintained  | **No** (archived)      | Yes               |
| TOML Spec   | 1.0                    | 1.1               |
| Parser      | Hand-written (old)     | winnow (modern)   |

## Benchmark Results

```
| Config Size | basic-toml | toml     | Speedup |
|-------------|------------|----------|---------|
| minimal     | 5.52 µs    | 2.85 µs  | 1.94x   |
| typical     | 20.22 µs   | 10.06 µs | 2.01x   |
| large       | 45.04 µs   | 22.94 µs | 1.96x   |
```

## Current Code (basic-toml)

### Cargo.toml

```toml
[dependencies]
basic-toml = { version = "0" }
```

### src/config.rs

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum ConfigError {
    #[error("IO error reading config file: {0}")]
    Io(#[from] std::io::Error),
    #[error("TOML parsing error in config file: {0}")]
    TomlParse(#[from] basic_toml::Error),
}

impl Config {
    pub(crate) fn from_str(content: &str) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(basic_toml::from_str(content)?)
    }
}
```

### src/content.rs

```rust
basic_toml::from_str(&meta_content).map_err(|e| ContentError::TomlParse {
    path: meta_path,
    source: e,
})
```

## After Migration (toml)

### Cargo.toml

```toml
[dependencies]
toml = { version = "0" }
```

### src/config.rs

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum ConfigError {
    #[error("IO error reading config file: {0}")]
    Io(#[from] std::io::Error),
    #[error("TOML parsing error in config file: {0}")]
    TomlParse(#[from] toml::de::Error),
}

impl Config {
    pub(crate) fn from_str(content: &str) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(toml::from_str(content)?)
    }
}
```

### src/content.rs

```rust
toml::from_str(&meta_content).map_err(|e| ContentError::TomlParse {
    path: meta_path,
    source: e,
})
```

## Migration Steps

1. Update `Cargo.toml`:
   - Remove `basic-toml` from `[dependencies]`
   - Add `toml` to `[dependencies]`
   - Keep `basic-toml` in `[dev-dependencies]` for benchmarks

2. Update `src/config.rs`:
   - Change `basic_toml::Error` to `toml::de::Error` in `ConfigError`
   - Change `basic_toml::from_str` to `toml::from_str`

3. Update `src/content.rs`:
   - Change `basic_toml::from_str` to `toml::from_str`
   - Update `ContentError::TomlParse` source type

4. Run tests: `mise run verify`

## API Compatibility

Both crates use serde, so the deserialization API is identical:

```rust
// basic-toml
let config: Config = basic_toml::from_str(content)?;

// toml
let config: Config = toml::from_str(content)?;
```

The only difference is the error type:

- `basic_toml::Error`
- `toml::de::Error`

## Files to Modify

| File             | Changes                                              |
| ---------------- | ---------------------------------------------------- |
| `Cargo.toml`     | Swap dependency                                      |
| `src/config.rs`  | Update error type, change `basic_toml::` to `toml::` |
| `src/content.rs` | Change `basic_toml::` to `toml::`                    |
