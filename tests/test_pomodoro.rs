use termino::pomodoro::{PomodoroPhase, PomodoroTimer};

#[test]
fn test_pomodoro_initial_state() {
    let pt = PomodoroTimer::new(25.0 * 60.0, 5.0 * 60.0, 15.0 * 60.0, 4);
    assert_eq!(pt.phase, PomodoroPhase::Work);
    assert_eq!(pt.completed_cycles(), 0);
    assert_eq!(pt.current_cycle, 0);
    assert!(!pt.is_complete());
}

#[test]
fn test_pomodoro_default_durations() {
    let pt = PomodoroTimer::new(25.0 * 60.0, 5.0 * 60.0, 15.0 * 60.0, 4);
    assert_eq!(pt.work_duration, 25.0 * 60.0);
    assert_eq!(pt.break_duration, 5.0 * 60.0);
    assert_eq!(pt.long_break_duration, 15.0 * 60.0);
    assert_eq!(pt.cycles_target, 4);
}

#[test]
fn test_pomodoro_get_current_duration_work() {
    let pt = PomodoroTimer::new(30.0 * 60.0, 5.0 * 60.0, 15.0 * 60.0, 4);
    assert_eq!(pt.phase, PomodoroPhase::Work);
    assert!((pt.get_current_duration() - 30.0 * 60.0).abs() < 0.001);
}

#[test]
fn test_pomodoro_get_current_duration_break() {
    let mut pt = PomodoroTimer::new(25.0 * 60.0, 7.0 * 60.0, 15.0 * 60.0, 4);
    pt.phase = PomodoroPhase::Break;
    assert!((pt.get_current_duration() - 7.0 * 60.0).abs() < 0.001);
}

#[test]
fn test_pomodoro_get_current_duration_long_break() {
    let mut pt = PomodoroTimer::new(25.0 * 60.0, 5.0 * 60.0, 20.0 * 60.0, 4);
    pt.phase = PomodoroPhase::LongBreak;
    assert!((pt.get_current_duration() - 20.0 * 60.0).abs() < 0.001);
}

#[test]
fn test_pomodoro_advance_phase_work_to_break() {
    let mut pt = PomodoroTimer::new(25.0 * 60.0, 5.0 * 60.0, 15.0 * 60.0, 4);
    let next = pt.advance_phase();
    assert_eq!(next, PomodoroPhase::Break);
    assert_eq!(pt.completed_cycles(), 1);
    assert_eq!(pt.phase, PomodoroPhase::Break);
}

#[test]
fn test_pomodoro_advance_phase_break_to_work() {
    let mut pt = PomodoroTimer::new(25.0 * 60.0, 5.0 * 60.0, 15.0 * 60.0, 4);
    pt.phase = PomodoroPhase::Break;
    let next = pt.advance_phase();
    assert_eq!(next, PomodoroPhase::Work);
}

#[test]
fn test_pomodoro_advance_after_all_cycles() {
    let mut pt = PomodoroTimer::new(25.0 * 60.0, 5.0 * 60.0, 15.0 * 60.0, 2);
    pt.advance_phase(); // work -> break (cycle 1)
    pt.advance_phase(); // break -> work (cycle 2)
    let next = pt.advance_phase(); // work -> long break
    assert_eq!(next, PomodoroPhase::LongBreak);
    assert_eq!(pt.phase, PomodoroPhase::LongBreak);
}

#[test]
fn test_pomodoro_advance_long_break_to_work() {
    let mut pt = PomodoroTimer::new(25.0 * 60.0, 5.0 * 60.0, 15.0 * 60.0, 2);
    pt.phase = PomodoroPhase::LongBreak;
    let next = pt.advance_phase();
    assert_eq!(next, PomodoroPhase::Work);
}

#[test]
fn test_pomodoro_record_cycle() {
    let mut pt = PomodoroTimer::new(25.0 * 60.0, 5.0 * 60.0, 15.0 * 60.0, 4);
    pt.record_cycle(25.0 * 60.0);
    assert_eq!(pt.cycles.len(), 1);
    assert_eq!(pt.cycles[0].cycle_type, "work");
    assert!((pt.cycles[0].duration - 25.0 * 60.0).abs() < 0.001);
}

#[test]
fn test_pomodoro_is_complete_after_full_session() {
    let mut pt = PomodoroTimer::new(25.0 * 60.0, 5.0 * 60.0, 15.0 * 60.0, 2);
    pt.completed_cycles = 2;
    pt.phase = PomodoroPhase::Break;
    assert!(pt.is_complete());
}

#[test]
fn test_pomodoro_is_complete_not_during_work() {
    let mut pt = PomodoroTimer::new(25.0 * 60.0, 5.0 * 60.0, 15.0 * 60.0, 4);
    pt.completed_cycles = 4;
    // Phase is still Work (shouldn't normally happen with proper flow)
    assert!(!pt.is_complete());
}

#[test]
fn test_pomodoro_cycles_list_stores_correct_data() {
    let mut pt = PomodoroTimer::new(25.0 * 60.0, 5.0 * 60.0, 15.0 * 60.0, 4);
    pt.record_cycle(25.0 * 60.0);
    pt.advance_phase();
    pt.record_cycle(5.0 * 60.0);
    assert_eq!(pt.cycles.len(), 2);
    assert_eq!(pt.cycles[0].cycle, 0);
    assert_eq!(pt.cycles[0].cycle_type, "work");
    assert!((pt.cycles[0].duration - 25.0 * 60.0).abs() < 0.001);
    assert_eq!(pt.cycles[1].cycle, 1);
    assert_eq!(pt.cycles[1].cycle_type, "break");
    assert!((pt.cycles[1].duration - 5.0 * 60.0).abs() < 0.001);
}