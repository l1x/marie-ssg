# PRD: Watch Mode

## 1. Overview

The "edit-build-check" loop is tedious if the user has to manually run the build command after every save. Watch mode automates this by monitoring the file system for changes and triggering a rebuild, enabling a live preview workflow.

## 2. Goals

1.  **Productivity**: Eliminate manual build steps during writing/development.
2.  **Responsiveness**: Detect changes and rebuild near-instantly.
3.  **Convenience**: Watch all relevant files (content, templates, config, static assets).

## 3. User Stories

- As a writer, I want the site to rebuild automatically when I save a Markdown file so I can see my changes immediately.
- As a theme developer, I want the site to rebuild when I edit a Jinja template.
- As a user, I want the watch command to be robust against rapid saves (debouncing).

## 4. Functional Requirements

### FR-1: Watch Command

- Add a `watch` subcommand to the CLI.
- Accept the same configuration arguments as `build`.
- **Acceptance**: `marie-ssg watch` starts a long-running process.

### FR-2: File Monitoring

- Monitor `content_dir`, `template_dir`, `static_dir`, and the configuration file itself.
- Use OS-native notification mechanisms (fsevent on macOS, potentially others later).
- **Acceptance**: Modifying a file in any of these locations triggers a log message "Changes detected".

### FR-3: Automatic Rebuild

- Trigger a full site build when a change is detected.
- **Acceptance**: The `output_dir` is updated with fresh files after a change.

### FR-4: Debouncing

- Implement a debounce mechanism (e.g., 500ms) to prevent multiple rebuilds from a single "save all" action or rapid file system events.
- **Acceptance**: Rapidly saving a file 3 times results in only one rebuild.

### FR-5: Fresh Environment

- Ensure templates are reloaded from disk on every rebuild (don't cache compiled templates across rebuilds in watch mode).
- **Acceptance**: Changing a template reflects in the output without restarting the watch process.

## 5. Non-functional Requirements

- **NFR-1**: Low CPU usage when idle.
- **NFR-2**: Graceful shutdown on Ctrl+C.

## 6. Non-Goals

- Built-in HTTP server (users can use `python -m http.server` or `caddy`).
- Hot Module Replacement (HMR) / Browser auto-reload (out of scope for now).

## 7. Success Metrics

- Latency between file save and build start < 1s.
- Reliable detection of changes on supported platforms (macOS initially).
