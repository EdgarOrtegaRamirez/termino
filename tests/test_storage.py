"""Tests for the storage module."""

import json
import os
import tempfile
import pytest
from unittest.mock import patch

from termino import storage


@pytest.fixture(autouse=True)
def temp_data_dir(monkeypatch):
    """Use a temporary directory for session data."""
    with tempfile.TemporaryDirectory() as tmpdir:
        monkeypatch.setattr(storage, "DATA_DIR", tmpdir)
        monkeypatch.setattr(storage, "SESSIONS_FILE", os.path.join(tmpdir, "sessions.json"))
        yield


class TestStorage:
    """Tests for session storage functions."""

    def test_save_session_creates_file(self):
        """Saving a session should create the sessions file."""
        storage.save_session("stopwatch", 30.5, "completed")
        assert os.path.exists(storage.SESSIONS_FILE)

    def test_save_session_returns_valid_dict(self):
        """save_session should return a valid session dict."""
        session = storage.save_session("stopwatch", 30.5, "completed")
        assert session["type"] == "stopwatch"
        assert session["duration_seconds"] == 30.5
        assert session["status"] == "completed"
        assert "started" in session
        assert "ended" in session

    def test_save_session_with_laps(self):
        """Saving a session with laps should include them."""
        laps = [
            {"lap": 1, "split": 10.0, "total": 10.0},
            {"lap": 2, "split": 15.5, "total": 25.5},
        ]
        session = storage.save_session("stopwatch", 25.5, "completed", laps=laps)
        assert len(session["laps"]) == 2
        assert session["laps"][0]["lap"] == 1
        assert session["laps"][0]["split"] == 10.0

    def test_save_session_with_cycles(self):
        """Saving a session with pomodoro cycles should include them."""
        cycles = [
            {"cycle": 1, "type": "work", "duration": 1500.0},
            {"cycle": 1, "type": "break", "duration": 300.0},
        ]
        session = storage.save_session("pomodoro", 1800.0, "completed", cycles=cycles)
        assert len(session["cycles"]) == 2
        assert session["cycles"][0]["type"] == "work"
        assert session["cycles"][0]["duration"] == 1500.0

    def test_get_sessions_empty(self):
        """get_sessions should return empty list when no sessions exist."""
        assert storage.get_sessions() == []

    def test_get_sessions_returns_all(self):
        """get_sessions should return all saved sessions."""
        storage.save_session("stopwatch", 30.0, "completed")
        storage.save_session("countdown", 60.0, "completed")
        sessions = storage.get_sessions()
        assert len(sessions) == 2

    def test_get_sessions_filter_by_type(self):
        """get_sessions should filter by type."""
        storage.save_session("stopwatch", 30.0, "completed")
        storage.save_session("countdown", 60.0, "completed")
        storage.save_session("stopwatch", 45.0, "completed")

        stopwatch_sessions = storage.get_sessions(session_type="stopwatch")
        assert len(stopwatch_sessions) == 2

        countdown_sessions = storage.get_sessions(session_type="countdown")
        assert len(countdown_sessions) == 1

    def test_get_sessions_limit(self):
        """get_sessions should respect the limit parameter."""
        for i in range(5):
            storage.save_session("stopwatch", float(i * 10), "completed")
        sessions = storage.get_sessions(limit=3)
        assert len(sessions) == 3

    def test_get_sessions_most_recent_first(self):
        """get_sessions should return most recent first."""
        storage.save_session("stopwatch", 10.0, "completed")
        storage.save_session("stopwatch", 20.0, "completed")
        sessions = storage.get_sessions()
        assert sessions[0]["duration_seconds"] == 20.0
        assert sessions[1]["duration_seconds"] == 10.0

    def test_get_session_count(self):
        """get_session_count should return the total count."""
        assert storage.get_session_count() == 0
        storage.save_session("stopwatch", 30.0, "completed")
        assert storage.get_session_count() == 1
        storage.save_session("countdown", 60.0, "completed")
        assert storage.get_session_count() == 2

    def test_duration_rounding(self):
        """Duration should be rounded to 3 decimal places."""
        session = storage.save_session("stopwatch", 30.123456, "completed")
        assert session["duration_seconds"] == 30.123

    def test_corrupted_file_returns_empty(self):
        """A corrupted sessions file should return an empty list."""
        os.makedirs(storage.DATA_DIR, exist_ok=True)
        with open(storage.SESSIONS_FILE, "w") as f:
            f.write("not valid json")
        assert storage.get_sessions() == []

    def test_save_preserves_existing(self):
        """Saving new sessions should preserve existing ones."""
        storage.save_session("stopwatch", 10.0, "completed")
        storage.save_session("countdown", 20.0, "completed")
        sessions = storage.get_sessions()
        assert len(sessions) == 2