"""Core timer/stopwatch logic for Termino."""

import time
import threading
from enum import Enum
from dataclasses import dataclass, field
from typing import Optional


class TimerState(Enum):
    """Possible states of a timer."""
    IDLE = "idle"
    RUNNING = "running"
    PAUSED = "paused"
    STOPPED = "stopped"
    FINISHED = "finished"


@dataclass
class Lap:
    """A recorded lap."""
    lap: int
    split: float  # Time for this lap only
    total: float   # Total elapsed time


@dataclass
class Stopwatch:
    """A stopwatch with lap timing capability.

    Supports start, stop, pause, resume, and lap recording.
    """
    _state: TimerState = TimerState.IDLE
    _start_time: Optional[float] = None
    _elapsed: float = 0.0
    _pause_start: Optional[float] = None
    _laps: list = field(default_factory=list)
    _lock: threading.Lock = field(default_factory=threading.Lock)

    @property
    def state(self) -> TimerState:
        return self._state

    @property
    def elapsed(self) -> float:
        with self._lock:
            if self._state == TimerState.RUNNING:
                return self._elapsed + (time.monotonic() - self._start_time)
            return self._elapsed

    @property
    def laps(self) -> list:
        return list(self._laps)

    def start(self) -> None:
        """Start the stopwatch."""
        with self._lock:
            if self._state not in (TimerState.IDLE, TimerState.STOPPED):
                raise RuntimeError(f"Cannot start from state {self._state.value}")
            self._start_time = time.monotonic()
            self._elapsed = 0.0
            self._laps = []
            self._state = TimerState.RUNNING

    def pause(self) -> None:
        """Pause the stopwatch."""
        with self._lock:
            if self._state != TimerState.RUNNING:
                raise RuntimeError(f"Cannot pause from state {self._state.value}")
            self._elapsed += time.monotonic() - self._start_time
            self._pause_start = time.monotonic()
            self._state = TimerState.PAUSED

    def resume(self) -> None:
        """Resume the stopwatch."""
        with self._lock:
            if self._state != TimerState.PAUSED:
                raise RuntimeError(f"Cannot resume from state {self._state.value}")
            self._start_time = time.monotonic()
            self._pause_start = None
            self._state = TimerState.RUNNING

    def stop(self) -> float:
        """Stop the stopwatch and return final elapsed time.

        Returns:
            Total elapsed time in seconds.
        """
        with self._lock:
            if self._state == TimerState.RUNNING:
                self._elapsed += time.monotonic() - self._start_time
            self._state = TimerState.STOPPED
            return self._elapsed

    def lap(self) -> Lap:
        """Record a lap.

        Returns:
            The recorded Lap.

        Raises:
            RuntimeError: If not running.
        """
        with self._lock:
            if self._state != TimerState.RUNNING:
                raise RuntimeError("Cannot record lap when stopwatch is not running")
            current = self._elapsed + (time.monotonic() - self._start_time)
            prev_total = self._laps[-1].total if self._laps else 0.0
            lap_number = len(self._laps) + 1
            lap_obj = Lap(lap=lap_number, split=current - prev_total, total=current)
            self._laps.append(lap_obj)
            return lap_obj


@dataclass
class Countdown:
    """A countdown timer.

    Runs for a specified duration and can be paused/resumed.
    """
    duration: float  # Total duration in seconds
    _state: TimerState = TimerState.IDLE
    _start_time: Optional[float] = None
    _elapsed: float = 0.0
    _pause_start: Optional[float] = None
    _lock: threading.Lock = field(default_factory=threading.Lock)

    @property
    def state(self) -> TimerState:
        return self._state

    @property
    def remaining(self) -> float:
        """Get remaining time in seconds."""
        with self._lock:
            if self._state == TimerState.RUNNING:
                elapsed = self._elapsed + (time.monotonic() - self._start_time)
                return max(self.duration - elapsed, 0.0)
            return max(self.duration - self._elapsed, 0.0)

    @property
    def elapsed(self) -> float:
        """Get elapsed time in seconds."""
        with self._lock:
            if self._state == TimerState.RUNNING:
                return self._elapsed + (time.monotonic() - self._start_time)
            return self._elapsed

    @property
    def is_finished(self) -> bool:
        """Check if countdown has finished."""
        return self.remaining <= 0.0

    def start(self) -> None:
        """Start the countdown."""
        with self._lock:
            if self._state not in (TimerState.IDLE, TimerState.STOPPED):
                raise RuntimeError(f"Cannot start from state {self._state.value}")
            self._start_time = time.monotonic()
            self._elapsed = 0.0
            self._state = TimerState.RUNNING

    def pause(self) -> None:
        """Pause the countdown."""
        with self._lock:
            if self._state != TimerState.RUNNING:
                raise RuntimeError(f"Cannot pause from state {self._state.value}")
            self._elapsed += time.monotonic() - self._start_time
            self._pause_start = time.monotonic()
            self._state = TimerState.PAUSED

    def resume(self) -> None:
        """Resume the countdown."""
        with self._lock:
            if self._state != TimerState.PAUSED:
                raise RuntimeError(f"Cannot resume from state {self._state.value}")
            self._start_time = time.monotonic()
            self._pause_start = None
            self._state = TimerState.RUNNING

    def stop(self) -> float:
        """Stop the countdown and return elapsed time.

        Returns:
            Elapsed time in seconds.
        """
        with self._lock:
            if self._state == TimerState.RUNNING:
                self._elapsed += time.monotonic() - self._start_time
            self._state = TimerState.STOPPED
            return self._elapsed