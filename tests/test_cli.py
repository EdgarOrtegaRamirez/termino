"""Tests for the CLI module."""

import pytest
from click.testing import CliRunner

from termino.cli import cli, _parse_duration


class TestParseDuration:
    """Tests for the _parse_duration helper."""

    def test_seconds(self):
        assert _parse_duration("30s") == 30.0

    def test_minutes(self):
        assert _parse_duration("5m") == 300.0

    def test_hours(self):
        assert _parse_duration("2h") == 7200.0

    def test_combined(self):
        assert _parse_duration("1h30m") == 5400.0

    def test_combined_all(self):
        assert _parse_duration("1h30m15s") == 5415.0

    def test_bare_number_treated_as_seconds(self):
        assert _parse_duration("60") == 60.0

    def test_empty_raises(self):
        with pytest.raises(Exception):
            _parse_duration("")

    def test_unknown_unit_raises(self):
        with pytest.raises(Exception):
            _parse_duration("5x")

    def test_zero_raises(self):
        with pytest.raises(Exception):
            _parse_duration("0s")


class TestCLI:
    """Tests for CLI commands."""

    def test_help(self):
        """The CLI should display help."""
        runner = CliRunner()
        result = runner.invoke(cli, ["--help"])
        assert result.exit_code == 0
        assert "Termino" in result.output

    def test_version(self):
        """The CLI should display version."""
        runner = CliRunner()
        result = runner.invoke(cli, ["--version"])
        assert result.exit_code == 0
        assert "0.1.0" in result.output

    def test_countdown_help(self):
        """The countdown command should display help."""
        runner = CliRunner()
        result = runner.invoke(cli, ["countdown", "--help"])
        assert result.exit_code == 0
        assert "Countdown" in result.output

    def test_stopwatch_help(self):
        """The stopwatch command should display help."""
        runner = CliRunner()
        result = runner.invoke(cli, ["stopwatch", "--help"])
        assert result.exit_code == 0
        assert "stopwatch" in result.output

    def test_pomodoro_help(self):
        """The pomodoro command should display help."""
        runner = CliRunner()
        result = runner.invoke(cli, ["pomodoro", "--help"])
        assert result.exit_code == 0
        assert "pomodoro" in result.output

    def test_history_help(self):
        """The history command should display help."""
        runner = CliRunner()
        result = runner.invoke(cli, ["history", "--help"])
        assert result.exit_code == 0
        assert "history" in result.output

    def test_history_no_sessions(self):
        """History with no sessions should show appropriate message."""
        runner = CliRunner()
        result = runner.invoke(cli, ["history"])
        assert result.exit_code == 0
        assert "No sessions found" in result.output

    def test_history_with_limit(self):
        """History command should accept limit option."""
        runner = CliRunner()
        result = runner.invoke(cli, ["history", "--limit", "5"])
        assert result.exit_code == 0

    def test_history_with_type(self):
        """History command should accept type option."""
        runner = CliRunner()
        result = runner.invoke(cli, ["history", "--type", "stopwatch"])
        assert result.exit_code == 0

    def test_history_invalid_type(self):
        """History command should reject invalid type."""
        runner = CliRunner()
        result = runner.invoke(cli, ["history", "--type", "invalid"])
        assert result.exit_code != 0