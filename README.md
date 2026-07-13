# Termino ⏱️

A sleek terminal-based timer and stopwatch CLI tool with countdown, lap timing, pomodoro mode, desktop notifications, and session logging.

## Features

- **⏱️ Stopwatch** — Start, stop, and lap timing with millisecond precision
- **⏳ Countdown** — Set a timer and get notified when it's done
- **🍅 Pomodoro Timer** — Work/break intervals with automatic cycling
- **🏁 Lap Tracking** — Record laps with split times during stopwatch mode
- **🔔 Desktop Notifications** — Get notified when timers expire (via `notify-send` or terminal bell)
- **📋 Session Logging** — All sessions saved to `~/.termino/sessions.json` for review
- **🎨 Rich Terminal UI** — Beautiful output with colors and progress bars using `rich`

## Installation

```bash
pip install termino
```

Or from source:

```bash
git clone https://github.com/EdgarOrtegaRamirez/termino
cd termino
pip install -e .
```

## Quick Start

```bash
# Start a stopwatch
termino stopwatch

# Start a 5-minute countdown
termino countdown 5m

# Start a pomodoro (25 min work, 5 min break)
termino pomodoro

# Show session history
termino history

# Show last 10 sessions
termino history --limit 10
```

## Usage

### Stopwatch Mode

```bash
termino stopwatch
```

Keyboard controls during stopwatch:
- `l` — Record a lap
- `Space` — Pause/Resume
- `q` or `Ctrl+C` — Stop and save

### Countdown Mode

```bash
# 5 minutes
termino countdown 5m

# 1 hour 30 minutes
termino countdown 1h30m

# 90 seconds
termino countdown 90s
```

Keyboard controls during countdown:
- `Space` — Pause/Resume
- `q` or `Ctrl+C` — Stop and save

### Pomodoro Mode

```bash
# Default: 25 min work, 5 min break, 4 cycles
termino pomodoro

# Custom: 50 min work, 10 min break, 3 cycles
termino pomodoro --work 50 --break 10 --cycles 3
```

Keyboard controls during pomodoro:
- `Space` — Pause/Resume
- `q` or `Ctrl+C` — Stop and save

### Session History

```bash
# Show all sessions
termino history

# Show last N sessions
termino history --limit 5

# Filter by session type
termino history --type stopwatch
termino history --type countdown
termino history --type pomodoro
```

## Session Logging

All sessions are automatically logged to `~/.termino/sessions.json` in JSON format:

```json
[
  {
    "type": "stopwatch",
    "started": "2026-07-13T10:00:00",
    "ended": "2026-07-13T10:05:30.123",
    "duration_seconds": 330.123,
    "status": "completed",
    "laps": [
      {"lap": 1, "split": 60.0, "total": 60.0},
      {"lap": 2, "split": 75.5, "total": 135.5}
    ]
  }
]
```

## License

MIT
