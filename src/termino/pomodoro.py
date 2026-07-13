"""Pomodoro timer logic for Termino."""

from dataclasses import dataclass, field
from typing import Optional
from enum import Enum


class PomodoroPhase(Enum):
    """Pomodoro timer phases."""
    WORK = "work"
    BREAK = "break"
    LONG_BREAK = "long_break"


@dataclass
class PomodoroCycle:
    """A recorded pomodoro cycle."""
    cycle: int
    type: str  # 'work', 'break', 'long_break'
    duration: float


@dataclass
class PomodoroTimer:
    """Pomodoro timer with work/break cycling.

    Default: 25 min work, 5 min break, 4 cycles, 15 min long break.
    """
    work_duration: float = 25 * 60  # 25 minutes
    break_duration: float = 5 * 60   # 5 minutes
    long_break_duration: float = 15 * 60  # 15 minutes
    cycles_target: int = 4
    current_cycle: int = 0
    phase: PomodoroPhase = PomodoroPhase.WORK
    cycles: list = field(default_factory=list)
    _completed_cycles: int = 0

    @property
    def completed_cycles(self) -> int:
        return self._completed_cycles

    def get_current_duration(self) -> float:
        """Get the duration for the current phase."""
        if self.phase == PomodoroPhase.WORK:
            return self.work_duration
        elif self.phase == PomodoroPhase.LONG_BREAK:
            return self.long_break_duration
        return self.break_duration

    def advance_phase(self) -> PomodoroPhase:
        """Advance to the next phase in the pomodoro cycle.

        Returns:
            The next phase.
        """
        if self.phase == PomodoroPhase.WORK:
            self.current_cycle += 1
            self._completed_cycles += 1
            if self.current_cycle >= self.cycles_target:
                self.phase = PomodoroPhase.LONG_BREAK
                self.current_cycle = 0
            else:
                self.phase = PomodoroPhase.BREAK
        elif self.phase in (PomodoroPhase.BREAK, PomodoroPhase.LONG_BREAK):
            self.phase = PomodoroPhase.WORK
            if self.current_cycle >= self.cycles_target:
                self.current_cycle = 0
        return self.phase

    def record_cycle(self, duration: float) -> None:
        """Record a completed cycle segment.

        Args:
            duration: Duration of the completed segment in seconds.
        """
        self.cycles.append(PomodoroCycle(
            cycle=self._completed_cycles,
            type=self.phase.value,
            duration=duration,
        ))

    @property
    def is_complete(self) -> bool:
        """Check if the full pomodoro session is complete.

        Returns True after completing all work cycles and their breaks.
        """
        return self._completed_cycles >= self.cycles_target and self.phase == PomodoroPhase.BREAK