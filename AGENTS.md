# AGENTS.md

## Project Overview

**Termino** — A terminal-based timer and stopwatch CLI tool with countdown, lap timing, pomodoro mode, desktop notifications, and session logging. Written in Rust.

## Architecture

```
src/
├── main.rs          # Entry point — calls into termino::cli
├── lib.rs           # Library root — re-exports all modules
├── cli.rs           # CLI argument parsing with clap derive (Parser + Subcommand)
├── timer.rs         # Stopwatch, Countdown, Lap, TimerState
├── pomodoro.rs      # PomodoroTimer with work/break cycling
├── display.rs       # Terminal display with crossterm (raw mode, key polling, progress bars)
├── notifications.rs # Desktop notifications (notify-rust on Linux, terminal bell fallback)
└── storage.rs       # Session logging to ~/.termino/sessions.json (serde JSON)
tests/
├── test_timer.rs    # 22 tests for Stopwatch & Countdown
├── test_pomodoro.rs # 13 tests for PomodoroTimer
├── test_storage.rs  # 9 tests for storage layer
└── test_cli.rs      # 9 tests for CLI (subprocess)
```

## Building

```bash
cargo build
cargo build --release
```

## Testing

```bash
cargo test
```

## Key Design Decisions

1. **clap for CLI** — Derive-based argument parsing with subcommands
2. **crossterm for terminal** — Raw mode, key polling, cursor management, ANSI styling
3. **notify-rust for notifications** — Native desktop notifications on Linux, fallback to terminal bell
4. **serde_json for storage** — Simple, portable JSON session logging under `~/.termino/`
5. **chrono for timestamps** — RFC 3339 timestamps in session data
6. **anyhow for errors** — Simple error handling with context propagation
7. **No threading** — Single-threaded async event loop with non-blocking key polling
8. **Atomic file writes** — Write to temp file, rename to avoid corruption

## Dependencies

- `clap` 4.x — CLI argument parsing
- `crossterm` 0.28 — Terminal manipulation
- `notify-rust` 4.x — Desktop notifications
- `serde` / `serde_json` — JSON serialization
- `chrono` — Timestamp handling
- `dirs` — Platform-specific directories
- `anyhow` — Error handling

## State Machine

### Stopwatch
```
Idle → Running ⇄ Paused → Stopped
```

### Countdown
```
Idle → Running ⇄ Paused → Stopped | Finished
```

### Pomodoro
```
Work → Break → Work → ... → Long Break → Work → ... → Complete
```

## Data Storage

- Path: `~/.termino/sessions.json`
- Format: JSON array of session objects
- Structure: type, started, ended, duration_seconds, status, laps[], cycles[]
- Writing: atomic (write to .tmp, rename)
- Reading: tolerant of corrupted files (returns empty array)

## Testing Conventions

- Integration tests only (tests/ directory)
- CLI tests run the binary as a subprocess
- Storage tests use TERMINO_HOME env var for isolation
- Timer tests use real time with small sleeps
- Pomodoro tests are purely logical (no sleep)