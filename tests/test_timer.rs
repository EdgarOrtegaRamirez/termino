use std::time::Duration;

use termino::timer::{Countdown, Stopwatch, TimerState};

#[test]
fn test_stopwatch_initial_state() {
    let sw = Stopwatch::new();
    assert_eq!(sw.state(), TimerState::Idle);
    assert_eq!(sw.elapsed(), 0.0);
    assert!(sw.laps().is_empty());
}

#[test]
fn test_stopwatch_start_transitions_to_running() {
    let mut sw = Stopwatch::new();
    sw.start().unwrap();
    assert_eq!(sw.state(), TimerState::Running);
}

#[test]
fn test_stopwatch_elapsed_increases_while_running() {
    let mut sw = Stopwatch::new();
    sw.start().unwrap();
    std::thread::sleep(Duration::from_millis(50));
    let elapsed = sw.elapsed();
    assert!(elapsed > 0.0);
    assert!(elapsed < 0.5);
}

#[test]
fn test_stopwatch_pause_stops_elapsed() {
    let mut sw = Stopwatch::new();
    sw.start().unwrap();
    std::thread::sleep(Duration::from_millis(50));
    sw.pause().unwrap();
    let paused_elapsed = sw.elapsed();
    std::thread::sleep(Duration::from_millis(50));
    assert_eq!(sw.elapsed(), paused_elapsed);
    assert_eq!(sw.state(), TimerState::Paused);
}

#[test]
fn test_stopwatch_resume_continues_elapsed() {
    let mut sw = Stopwatch::new();
    sw.start().unwrap();
    std::thread::sleep(Duration::from_millis(50));
    sw.pause().unwrap();
    let paused_elapsed = sw.elapsed();
    sw.resume().unwrap();
    assert_eq!(sw.state(), TimerState::Running);
    std::thread::sleep(Duration::from_millis(50));
    assert!(sw.elapsed() > paused_elapsed);
}

#[test]
fn test_stopwatch_stop_returns_elapsed() {
    let mut sw = Stopwatch::new();
    sw.start().unwrap();
    std::thread::sleep(Duration::from_millis(50));
    let final_time = sw.stop();
    assert_eq!(sw.state(), TimerState::Stopped);
    assert!(final_time > 0.0);
    assert_eq!(sw.elapsed(), final_time);
}

#[test]
fn test_stopwatch_lap_recording() {
    let mut sw = Stopwatch::new();
    sw.start().unwrap();
    std::thread::sleep(Duration::from_millis(30));
    let lap1 = sw.lap().unwrap();
    std::thread::sleep(Duration::from_millis(30));
    let lap2 = sw.lap().unwrap();
    sw.stop();

    assert_eq!(sw.laps().len(), 2);
    assert_eq!(lap1.lap, 1);
    assert_eq!(lap2.lap, 2);
    assert!(lap2.total > lap1.total);
    assert!(lap2.split > 0.0);
}

#[test]
fn test_stopwatch_lap_while_not_running_raises() {
    let mut sw = Stopwatch::new();
    assert!(sw.lap().is_err());
}

#[test]
fn test_stopwatch_pause_while_not_running_raises() {
    let mut sw = Stopwatch::new();
    assert!(sw.pause().is_err());
}

#[test]
fn test_stopwatch_resume_while_not_paused_raises() {
    let mut sw = Stopwatch::new();
    assert!(sw.resume().is_err());
}

#[test]
fn test_stopwatch_start_while_running_raises() {
    let mut sw = Stopwatch::new();
    sw.start().unwrap();
    assert!(sw.start().is_err());
}

#[test]
fn test_stopwatch_start_after_stop() {
    let mut sw = Stopwatch::new();
    sw.start().unwrap();
    std::thread::sleep(Duration::from_millis(30));
    sw.stop();
    sw.start().unwrap();
    assert_eq!(sw.state(), TimerState::Running);
    std::thread::sleep(Duration::from_millis(30));
    assert!(sw.elapsed() > 0.0);
}

#[test]
fn test_countdown_initial_state() {
    let cd = Countdown::new(10.0);
    assert_eq!(cd.state(), TimerState::Idle);
    assert_eq!(cd.remaining(), 10.0);
    assert_eq!(cd.elapsed(), 0.0);
    assert!(!cd.is_finished());
}

#[test]
fn test_countdown_start_transitions_to_running() {
    let mut cd = Countdown::new(10.0);
    cd.start().unwrap();
    assert_eq!(cd.state(), TimerState::Running);
}

#[test]
fn test_countdown_remaining_decreases_while_running() {
    let mut cd = Countdown::new(1.0);
    cd.start().unwrap();
    std::thread::sleep(Duration::from_millis(200));
    assert!(cd.remaining() < 1.0);
}

#[test]
fn test_countdown_is_finished_when_time_expires() {
    let mut cd = Countdown::new(0.1);
    cd.start().unwrap();
    std::thread::sleep(Duration::from_millis(200));
    assert!(cd.is_finished());
    assert_eq!(cd.remaining(), 0.0);
}

#[test]
fn test_countdown_pause_stops_countdown() {
    let mut cd = Countdown::new(5.0);
    cd.start().unwrap();
    std::thread::sleep(Duration::from_millis(50));
    cd.pause().unwrap();
    let remaining_paused = cd.remaining();
    std::thread::sleep(Duration::from_millis(50));
    assert_eq!(cd.remaining(), remaining_paused);
}

#[test]
fn test_countdown_resume_continues_countdown() {
    let mut cd = Countdown::new(5.0);
    cd.start().unwrap();
    std::thread::sleep(Duration::from_millis(50));
    cd.pause().unwrap();
    let remaining_after_pause = cd.remaining();
    cd.resume().unwrap();
    std::thread::sleep(Duration::from_millis(50));
    assert!(cd.remaining() < remaining_after_pause);
}

#[test]
fn test_countdown_stop_returns_elapsed() {
    let mut cd = Countdown::new(5.0);
    cd.start().unwrap();
    std::thread::sleep(Duration::from_millis(50));
    let final_time = cd.stop();
    assert_eq!(cd.state(), TimerState::Stopped);
    assert!(final_time > 0.0);
}

#[test]
fn test_countdown_start_while_running_raises() {
    let mut cd = Countdown::new(5.0);
    cd.start().unwrap();
    assert!(cd.start().is_err());
}

#[test]
fn test_countdown_pause_while_not_running_raises() {
    let mut cd = Countdown::new(5.0);
    assert!(cd.pause().is_err());
}

#[test]
fn test_countdown_resume_while_not_paused_raises() {
    let mut cd = Countdown::new(5.0);
    assert!(cd.resume().is_err());
}