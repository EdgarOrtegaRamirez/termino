"""Session logging for Termino.

All sessions are stored as JSON in ~/.termino/sessions.json.
"""

import json
import os
from datetime import datetime, timezone
from typing import Optional


DATA_DIR = os.path.expanduser("~/.termino")
SESSIONS_FILE = os.path.join(DATA_DIR, "sessions.json")


def _ensure_data_dir() -> None:
    """Create the data directory if it doesn't exist."""
    os.makedirs(DATA_DIR, exist_ok=True)


def _load_sessions() -> list:
    """Load all sessions from the sessions file.

    Returns:
        List of session dicts.
    """
    _ensure_data_dir()
    if not os.path.exists(SESSIONS_FILE):
        return []
    try:
        with open(SESSIONS_FILE, "r") as f:
            return json.load(f)
    except (json.JSONDecodeError, OSError):
        return []


def _save_sessions(sessions: list) -> None:
    """Save sessions to the sessions file.

    Args:
        sessions: List of session dicts.
    """
    _ensure_data_dir()
    with open(SESSIONS_FILE, "w") as f:
        json.dump(sessions, f, indent=2)


def save_session(
    session_type: str,
    duration_seconds: float,
    status: str = "completed",
    laps: Optional[list] = None,
    cycles: Optional[list] = None,
) -> dict:
    """Save a session to the session log.

    Args:
        session_type: Type of session (stopwatch, countdown, pomodoro).
        duration_seconds: Total duration in seconds.
        status: Session status (completed, interrupted).
        laps: List of lap dicts (for stopwatch).
        cycles: List of cycle dicts (for pomodoro).

    Returns:
        The saved session dict.
    """
    now = datetime.now(timezone.utc).isoformat()
    session = {
        "type": session_type,
        "started": now,
        "ended": now,
        "duration_seconds": round(duration_seconds, 3),
        "status": status,
    }
    if laps:
        session["laps"] = [
            {"lap": l["lap"], "split": round(l["split"], 3), "total": round(l["total"], 3)}
            for l in laps
        ]
    if cycles:
        session["cycles"] = [
            {"cycle": c["cycle"], "type": c["type"], "duration": round(c["duration"], 3)}
            for c in cycles
        ]

    sessions = _load_sessions()
    sessions.append(session)
    _save_sessions(sessions)
    return session


def get_sessions(session_type: Optional[str] = None, limit: Optional[int] = None) -> list:
    """Get sessions, optionally filtered by type and limited.

    Args:
        session_type: Optional filter by session type.
        limit: Optional max number of sessions to return (most recent first).

    Returns:
        List of session dicts.
    """
    sessions = _load_sessions()
    if session_type:
        sessions = [s for s in sessions if s.get("type") == session_type]
    # Reverse so most recent first
    sessions.reverse()
    if limit is not None:
        sessions = sessions[:limit]
    return sessions


def get_session_count() -> int:
    """Get the total number of logged sessions.

    Returns:
        Session count.
    """
    return len(_load_sessions())