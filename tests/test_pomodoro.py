"""Tests for the pomodoro module."""

import pytest
from termino.pomodoro import PomodoroTimer, PomodoroPhase


class TestPomodoroTimer:
    """Tests for the PomodoroTimer class."""

    def test_initial_state(self):
        """A new pomodoro timer should start in WORK phase."""
        pt = PomodoroTimer()
        assert pt.phase == PomodoroPhase.WORK
        assert pt.completed_cycles == 0
        assert pt.current_cycle == 0
        assert not pt.is_complete

    def test_default_durations(self):
        """Default durations should be 25min work, 5min break, 15min long break."""
        pt = PomodoroTimer()
        assert pt.work_duration == 25 * 60
        assert pt.break_duration == 5 * 60
        assert pt.long_break_duration == 15 * 60
        assert pt.cycles_target == 4

    def test_custom_durations(self):
        """Custom durations should be honored."""
        pt = PomodoroTimer(
            work_duration=50 * 60,
            break_duration=10 * 60,
            long_break_duration=20 * 60,
            cycles_target=3,
        )
        assert pt.work_duration == 50 * 60
        assert pt.break_duration == 10 * 60
        assert pt.long_break_duration == 20 * 60
        assert pt.cycles_target == 3

    def test_get_current_duration_work(self):
        """get_current_duration should return work duration in WORK phase."""
        pt = PomodoroTimer(work_duration=30 * 60)
        assert pt.phase == PomodoroPhase.WORK
        assert pt.get_current_duration() == 30 * 60

    def test_get_current_duration_break(self):
        """get_current_duration should return break duration in BREAK phase."""
        pt = PomodoroTimer(break_duration=7 * 60)
        pt.phase = PomodoroPhase.BREAK
        assert pt.get_current_duration() == 7 * 60

    def test_get_current_duration_long_break(self):
        """get_current_duration should return long break duration in LONG_BREAK phase."""
        pt = PomodoroTimer(long_break_duration=20 * 60)
        pt.phase = PomodoroPhase.LONG_BREAK
        assert pt.get_current_duration() == 20 * 60

    def test_advance_phase_work_to_break(self):
        """Advancing from WORK should go to BREAK and increment completed cycles."""
        pt = PomodoroTimer(cycles_target=4)
        next_phase = pt.advance_phase()
        assert next_phase == PomodoroPhase.BREAK
        assert pt.completed_cycles == 1
        assert pt.phase == PomodoroPhase.BREAK

    def test_advance_phase_break_to_work(self):
        """Advancing from BREAK should go to WORK."""
        pt = PomodoroTimer(cycles_target=4)
        pt.phase = PomodoroPhase.BREAK
        next_phase = pt.advance_phase()
        assert next_phase == PomodoroPhase.WORK
        assert pt.phase == PomodoroPhase.WORK

    def test_advance_phase_after_all_cycles(self):
        """After completing all work cycles, advance should go to LONG_BREAK."""
        pt = PomodoroTimer(cycles_target=2)
        # Complete 2 work cycles
        pt.advance_phase()  # work -> break (cycle 1)
        pt.advance_phase()  # break -> work (cycle 2)
        next_phase = pt.advance_phase()  # work -> long break
        assert next_phase == PomodoroPhase.LONG_BREAK
        assert pt.phase == PomodoroPhase.LONG_BREAK

    def test_advance_phase_long_break_to_work(self):
        """Advancing from LONG_BREAK should go to WORK."""
        pt = PomodoroTimer(cycles_target=2)
        pt.phase = PomodoroPhase.LONG_BREAK
        next_phase = pt.advance_phase()
        assert next_phase == PomodoroPhase.WORK
        assert pt.phase == PomodoroPhase.WORK

    def test_record_cycle(self):
        """Recording a cycle should add to the cycles list."""
        pt = PomodoroTimer()
        pt.record_cycle(25 * 60)
        assert len(pt.cycles) == 1
        assert pt.cycles[0].type == "work"
        assert pt.cycles[0].duration == 25 * 60

    def test_is_complete_after_full_session(self):
        """is_complete should be True after completing all cycles."""
        pt = PomodoroTimer(cycles_target=2)
        pt._completed_cycles = 2
        pt.phase = PomodoroPhase.BREAK
        assert pt.is_complete

    def test_is_complete_not_during_work(self):
        """is_complete should be False during work phase."""
        pt = PomodoroTimer(cycles_target=4)
        pt._completed_cycles = 4
        # Phase is still WORK (shouldn't normally happen with proper flow)
        assert not pt.is_complete

    def test_cycles_list_stores_correct_data(self):
        """The cycles list should store cycle data correctly."""
        pt = PomodoroTimer()
        pt.record_cycle(25 * 60)
        pt.advance_phase()
        pt.record_cycle(5 * 60)
        assert len(pt.cycles) == 2
        assert pt.cycles[0].cycle == 0
        assert pt.cycles[0].type == "work"
        assert pt.cycles[0].duration == 25 * 60
        assert pt.cycles[1].cycle == 1
        assert pt.cycles[1].type == "break"
        assert pt.cycles[1].duration == 5 * 60