# PRD: Parallel Execution

## 1. Overview

As the content volume grows, sequential processing of Markdown files becomes a bottleneck. This feature aims to parallelize the content loading and conversion steps to significantly reduce build times for larger sites, leveraging modern multi-core processors.

## 2. Goals

1.  **Speed**: Drastically reduce the "Loading all content" phase duration.
2.  **Scalability**: Ensure the SSG remains fast even with thousands of pages.
3.  **Safety**: Maintain deterministic output despite parallel execution.

## 3. User Stories

- As a user with a large blog archive, I want the build to finish as fast as possible so I can deploy updates quickly.
- As a developer, I want the parallel execution to be transparent (no config changes needed) so I get free performance wins.

## 4. Functional Requirements

### FR-1: Parallel Content Loading

- Use `rayon` to parallelize the iteration over found Markdown files.
- Process file reading, frontmatter parsing, and Markdown-to-HTML conversion in parallel.
- **Acceptance**: The log output for "Loading all content" shows a decrease in time for large datasets compared to sequential execution.

### FR-2: Thread Safety

- Ensure that any shared resources (like the syntax highlighting theme set) are accessed safely or cloned per thread.
- **Acceptance**: No data races or crashes during build.

### FR-3: Error Handling

- If a file fails to process in a parallel thread, the build should fail fast or collect errors and report them.
- **Acceptance**: A syntax error in one markdown file stops the build or reports the error correctly, not swallowed by a thread.

## 5. Non-functional Requirements

- **NFR-1**: Parallelism should not exhaust system file handles (rayon handles this mostly, but good to note).
- **NFR-2**: CPU usage will spike during build (expected behavior).

## 6. Non-Goals

- Parallel writing of output files (disk I/O is often the bottleneck there, simple iteration is fine for now).
- Distributed build across machines.

## 7. Success Metrics

- > 50% reduction in build time for sites with >1000 pages on a quad-core machine.
