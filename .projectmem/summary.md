# projectmem - lomi_win

_Last updated: 2026-08-29_

## Project purpose
Replace this placeholder with a concise description of what this project does, who it serves, and the main technologies or runtime assumptions.

## Recent issues
- [DONE] #0001 Implementing IoTBridge subcommand naming, Swarm join fallback, --dashboard-port option, warning cleanup, and cli_test updates -> Added iotbridge name and aliases, Swarm join fallback port, configurable dashboard_port option, warning cleanups, and test suite updates in main.rs and cli_test.rs (fixed)
  - Partial attempt: Implementing IoTBridge subcommand naming, Swarm join fallback, --dashboard-port option, warning cleanup, and cli_test updates

## Decisions
- Single-file Rust binary architecture (src/main.rs) implementing a complete local AI Gateway & Tuner with CLI, TUI, GUI, HTTP Proxy, and Windows OS API integrations

## Notes
- High churn detected: README.md (4 edits in 10 min) [README.md]
- Key components: Universal HTTP API Proxy (:8080) & Named Pipe (\\\\.\\pipe\\LomiGateway), Token Squeezer minifier, Waterfall model router (Local, Groq, OpenAI), Hyper-V/Job Object process sandboxing, ETW/Event Viewer crash diagnostics, Vector DB indexer, WSL2 cross-VM network bridge, and TUI/GUI dashboards.
- Build & Run: Cargo project with Windows-specific crate dependencies (windows 0.52, winreg 0.52, tray-item 0.10). Supports `cargo check` and `cargo build --release`. Optional CUDA support enabled via `--features cuda`.
- Integration test suite tests/cli_test.rs executed via `cargo test --test cli_test`. All 16 feature tests passed successfully (16 passed, 0 failed).

## Key files
- `README.md`
- `src/main.rs`
- `0.52`
- `0.10`
- `tests/cli_test.rs`
- `main.rs`
- `cli_test.rs`

## Open questions
- None logged yet.
