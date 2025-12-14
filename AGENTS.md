# Marie SSG - Agent Guidelines

## Build Commands
- `mise run lint` - Run cargo lint (code formatting and style checks)
- `mise run tests` - Run all tests with cargo test -- --nocapture
- `cargo test <test_name>` - Run single test
- `mise run build-dev` - Development build with cargo build
- `mise run build-prod` - Production optimized build with cargo build --release

## Code Style Guidelines

### Imports & Organization
- Group imports: std crates first, then external crates, then internal modules
- Use `use crate::*` for internal module imports
- Keep imports at file top, organized alphabetically within groups

### Formatting & Types
- Use `rustfmt` for formatting (enforced via cargo lint)
- Prefer explicit types in function signatures and struct fields
- Use `pub(crate)` for internal visibility, `pub` for public API
- Struct fields use `pub` when data needs to be accessed externally

### Naming Conventions
- Functions: snake_case with descriptive names
- Structs/Enums: PascalCase with clear purpose
- Variables: snake_case, prefer descriptive over abbreviated
- Constants: SCREAMING_SNAKE_CASE
- File names: snake_case matching module names

### Error Handling
- Use `thiserror` for custom error types with proper error messages
- Implement `From` traits for error conversions
- Use `Result<T, ErrorType>` for fallible operations
- Include context in error messages (file paths, operation details)
- Use `#[source]` attribute for error chaining

### Documentation
- Add doc comments (`///`) for all public structs, functions, and types
- Include examples in doc comments where helpful
- Use `#[instrument(skip_all)]` tracing for key functions
- Add inline comments for complex logic only

### Architecture Patterns
- Separate concerns into distinct modules (config, content, template, etc.)
- Use `rayon` for parallel processing of independent operations
- Implement proper error propagation with `?` operator
- Use `tracing` for structured logging with appropriate levels
- Prefer composition over inheritance

### Testing
- Write unit tests for core logic in separate `#[cfg(test)]` modules
- Use `tempfile` for temporary file/directory creation in tests
- Include integration tests for end-to-end workflows
- Use `criterion` for performance benchmarks when relevant