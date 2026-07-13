"""Tests for the timer module."""

import time
import pytest
from termino.timer import Stopwatch, Countdown, TimerState


class TestStopwatch:
    """Tests for the Stopwatch class."""

    def test_initial_state(self):
        """A new stopwatch should be IDLE with zero elapsed time."""
        sw = Stopwatch()
        assert sw.state == TimerState.IDLE
        assert sw.elapsed == 0.0
        assert sw.laps == []

    def test_start_transitions_to_running(self):
        """Starting the stopwatch should set state to RUNNING."""
        sw = Stopwatch()
        sw.start()
        assert sw.state == TimerState.RUNNING

    def test_elapsed_increases_while_running(self):
        """Elapsed time should increase while the stopwatch is running."""
        sw = Stopwatch()
        sw.start()
        time.sleep(0.05)
        elapsed = sw.elapsed
        assert elapsed > 0.0
        # Allow some tolerance
        assert 0.01 < elapsed < 0.5

    def test_pause_stops_elapsed(self):
        """Pausing should stop the elapsed time from increasing."""
        sw = Stopwatch()
        sw.start()
        time.sleep(0.05)
        sw.pause()
        paused_elapsed = sw.elapsed
        time.sleep(0.05)
        # Should not increase while paused
        assert sw.elapsed == paused_elapsed
        assert sw.state == TimerState.PAUSED

    def test_resume_continues_elapsed(self):
        """Resuming should continue the elapsed time."""
        sw = Stopwatch()
        sw.start()
        time.sleep(0.05)
        sw.pause()
        paused_elapsed = sw.elapsed
        sw.resume()
        assert sw.state == TimerState.RUNNING
        time.sleep(0.05)
        assert sw.elapsed > paused_elapsed

    def test_stop_returns_elapsed(self):
        """Stopping should return the elapsed time."""
        sw = Stopwatch()
        sw.start()
        time.sleep(0.05)
        final = sw.stop()
        assert sw.state == TimerState.STOPPED
        assert final > 0.0
        # Elapsed should not change after stop
        assert sw.elapsed == final

    def test_lap_recording(self):
        """Laps should be recorded correctly."""
        sw = Stopwatch()
        sw.start()
        time.sleep(0.05)
        lap1 = sw.lap()
        time.sleep(0.05)
        lap2 = sw.lap()
        sw.stop()

        assert len(sw.laps) == 2
        assert lap1.lap == 1
        assert lap2.lap == 2
        assert lap2.total > lap1.total
        assert lap2.split > 0.0

    def test_lap_while_not_running_raises(self):
        """Recording a lap while not running should raise RuntimeError."""
        sw = Stopwatch()
        with pytest.raises(RuntimeError):
            sw.lap()

    def test_pause_while_not_running_raises(self):
        """Pausing while not running should raise RuntimeError."""
        sw = Stopwatch()
        with pytest.raises(RuntimeError):
            sw.pause()

    def test_resume_while_not_paused_raises(self):
        """Resuming while not paused should raise RuntimeError."""
        sw = Stopwatch()
        with pytest.raises(RuntimeError):
            sw.resume()

    def test_start_while_running_raises(self):
        """Starting while already running should raise RuntimeError."""
        sw = Stopwatch()
        sw.start()
        with pytest.raises(RuntimeError):
            sw.start()

    def test_start_after_stop(self):
        """Starting after stopping should reset the stopwatch."""
        sw = Stopwatch()
        sw.start()
        time.sleep(0.05)
        sw.stop()
        sw.start()
        assert sw.state == TimerState.RUNNING
        time.sleep(0.05)
        assert sw.elapsed > 0.0


class TestCountdown:
    """Tests for the Countdown class."""

    def test_initial_state(self):
        """A new countdown should be IDLE with full remaining time."""
        cd = Countdown(duration=10.0)
        assert cd.state == TimerState.IDLE
        assert cd.remaining == 10.0
        assert cd.elapsed == 0.0
        assert not cd.is_finished

    def test_start_transitions_to_running(self):
        """Starting should set state to RUNNING."""
        cd = Countdown(duration=10.0)
        cd.start()
        assert cd.state == TimerState.RUNNING

    def test_remaining_decreases_while_running(self):
        """Remaining time should decrease while running."""
        cd = Countdown(duration=1.0)
        cd.start()
        time.sleep(0.2)
        assert cd.remaining < 1.0
        assert cd.elapsed > 0.0

    def test_is_finished_when_time_expires(self):
        """is_finished should be True when time is up."""
        cd = Countdown(duration=0.1)
        cd.start()
        time.sleep(0.2)
        assert cd.is_finished
        assert cd.remaining == 0.0

    def test_pause_stops_countdown(self):
        """Pausing should stop the countdown from decreasing."""
        cd = Countdown(duration=5.0)
        cd.start()
        time.sleep(0.05)
        cd.pause()
        remaining_paused = cd.remaining
        time.sleep(0.05)
        assert cd.remaining == remaining_paused

    def test_resume_continues_countdown(self):
        """Resuming should continue the countdown."""
        cd = Countdown(duration=5.0)
        cd.start()
        time.sleep(0.05)
        cd.pause()
        remaining_after_pause = cd.remaining
        cd.resume()
        time.sleep(0.05)
        assert cd.remaining < remaining_after_pause

    def test_stop_returns_elapsed(self):
        """Stopping should return the elapsed time."""
        cd = Countdown(duration=5.0)
        cd.start()
        time.sleep(0.05)
        final = cd.stop()
        assert cd.state == TimerState.STOPPED
        assert final > 0.0

    def test_start_while_running_raises(self):
        """Starting while already running should raise RuntimeError."""
        cd = Countdown(duration=5.0)
        cd.start()
        with pytest.raises(RuntimeError):
            cd.start()

    def test_pause_while_not_running_raises(self):
        """Pausing while not running should raise RuntimeError."""
        cd = Countdown(duration=5.0)
        with pytest.raises(RuntimeError):
            cd.pause()

    def test_resume_while_not_paused_raises(self):
        """Resuming while not paused should raise RuntimeError."""
        cd = Countdown(duration=5.0)
        with pytest.raises(RuntimeError):
            cd.resume()