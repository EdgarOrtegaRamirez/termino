# AGENTS.md

## Project Overview

**Termino** — A terminal-based timer and stopwatch CLI tool with countdown, lap timing, pomodoro mode, desktop notifications, and session logging.

## Architecture

- `src/termino/` — Main package directory
  - `__init__.py` — Package init, version info
  - `__main__.py` — Entry point for `python -m termino`
  - `cli.py` — Click-based CLI interface with all commands
  - `timer.py` — Core timer/stopwatch logic with key-based controls
  - `pomodoro.py` — Pomodoro timer with work/break cycling
  - `notifications.py` — Desktop notification support (notify-send, terminal bell)
  - `storage.py` — Session logging to JSON files
  - `display.py` — Rich-based terminal display formatting
- `tests/` — Test directory
  - `test_timer.py` — Tests for timer logic
  - `test_pomodoro.py` — Tests for pomodoro logic
  - `test_storage.py` — Tests for session storage
  - `test_cli.py` — Tests for CLI commands
- `pyproject.toml` — Project metadata and dependencies
- `README.md` — User-facing documentation
- `AGENTS.md` — This file: architecture and development notes
- `LICENSE` — MIT

## Building

```bash
pip install -e .
```

## Testing

```bash
# Run all tests
pytest

# Run with coverage
pytest --cov=src/termino

# Run specific test file
pytest tests/test_timer.py -v
```

## Key Design Decisions

1. **Click for CLI** — Well-established Python CLI framework with good composability
2. **Rich for display** — Beautiful terminal output with progress bars, tables, and colors
3. **JSON file storage** — Simple, portable session logging under `~/.termino/`
4. **Keyboard controls** — Real-time key handling during active timers (pausable)
5. **Platform notifications** — Uses `notify-send` on Linux, terminal bell as fallback
6. **Threading** — Timer runs in a background thread so keyboard input stays responsive

## Dependencies

- Python 3.10+
- `click>=8.0` — CLI framework
- `rich>=13.0` — Terminal formatting and display

## Testing Conventions

- `pytest` with standard conventions
- Test files named `test_*.py` in `tests/`
- Test both happy paths and error cases
- Use `pytest` fixtures for shared test data
- Mock time-dependent operations with `pytest-mock` (optional, not required)