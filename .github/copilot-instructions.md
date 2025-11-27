# GitHub Copilot Instructions

You are an expert Rust developer building a high-performance database proxy for PII anonymization.

## Tech Stack
- **Core**: Rust (2021 edition)
- **Async Runtime**: `tokio`
- **Web Server**: `axum`
- **CLI**: `clap`
- **Logging**: `tracing`
- **Protocol Handling**: `tokio-util` (Codecs), `bytes`
- **Frontend**: Next.js (React), Tailwind CSS, Shadcn UI

## Coding Principles
1.  **Safety & Performance**: Prioritize memory safety. Use `Arc` and `Mutex`/`RwLock` judiciously. Prefer cloning only when necessary; aim for zero-copy parsing where possible.
2.  **Error Handling**: Use `thiserror` for library/core errors and `anyhow` for application/CLI errors. Never use `unwrap()` in production code; always handle `Result` and `Option`.
3.  **Async/Await**: Ensure all I/O is non-blocking. Use `tokio::select!` for concurrent task management.
4.  **Functional Style**: Prefer functional programming patterns. Use iterators (`map`, `filter`, `fold`) over explicit loops. Leverage `Option` and `Result` combinators (`and_then`, `map_err`) instead of nested `match` statements. Keep data immutable by default.
5.  **Testing**: Unit testing is critical. Write comprehensive unit tests for all parsing and anonymization logic. Ensure that every transformation function is covered by tests.
6.  **Comments**: Document complex protocol parsing logic. Explain *why* a specific byte manipulation is happening.

## Project Context
- This is a **Database Proxy**. It sits between a client and a real Postgres/MySQL database.
- **Goal**: Intercept `DataRow` packets and anonymize PII (Personally Identifiable Information) on the fly.
- **Critical**: The proxy must maintain the integrity of the wire protocol. Packet lengths must be recalculated if data size changes.
- **Roadmap**: Refer to `ROADMAP.md` in the root directory to track the project status and current phase.

## Frontend Guidelines
- Use Functional Components with Hooks.
- Use Strong Typing with TypeScript.
- State management should be handled via React Query (TanStack Query) for server state.
