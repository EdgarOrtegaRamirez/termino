#![allow(clippy::collapsible_if)]

use std::io::{Write, stdout};
use std::time::Duration;

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    execute, queue,
    style::{self, Stylize},
    terminal::{self, ClearType},
};

use crate::notifications::send_notification;
use crate::pomodoro::{PomodoroPhase, PomodoroTimer};
use crate::storage::{SessionCycle, SessionData, SessionLap};
use crate::timer::{Countdown, Stopwatch, TimerState};

/// Format a duration in seconds to HH:MM:SS.mmm
pub fn format_duration(seconds: f64) -> String {
    let total_secs = seconds as u64;
    let hours = total_secs / 3600;
    let minutes = (total_secs % 3600) / 60;
    let secs = total_secs % 60;
    let millis = ((seconds - seconds.floor()) * 1000.0) as u64;

    if hours > 0 {
        format!("{:02}:{:02}:{:02}.{:03}", hours, minutes, secs, millis)
    } else {
        format!("{:02}:{:02}.{:03}", minutes, secs, millis)
    }
}

/// Print the termino banner.
pub fn print_banner() {
    println!(
        "{}",
        style::style("╔══════════════════════════════╗").dark_blue()
    );
    println!(
        "{} {} {}",
        style::style("║").dark_blue(),
        style::style("Termino ⏱️").bold().cyan(),
        style::style("     ║").dark_blue()
    );
    println!(
        "{}",
        style::style("╚══════════════════════════════╝").dark_blue()
    );
}

/// Print controls for a given mode.
fn print_controls(mode: &str) {
    match mode {
        "stopwatch" => {
            println!(
                "{}",
                style::style("Controls: [l] Lap  [Space] Pause/Resume  [q/Ctrl+C] Stop").dim()
            );
        }
        _ => {
            println!(
                "{}",
                style::style("Controls: [Space] Pause/Resume  [q/Ctrl+C] Stop").dim()
            );
        }
    }
}

/// Check if a key event is a quit signal (q or Ctrl+C).
fn is_quit(key: &KeyEvent) -> bool {
    key.kind == KeyEventKind::Press
        && match key.code {
            KeyCode::Char('q') => key.modifiers == KeyModifiers::NONE,
            KeyCode::Char('c') => key.modifiers == KeyModifiers::CONTROL,
            _ => false,
        }
}

/// Check if a key event is a space (pause/resume).
fn is_space(key: &KeyEvent) -> bool {
    key.kind == KeyEventKind::Press && key.code == KeyCode::Char(' ')
}

/// Check if a key event is 'l' (lap).
fn is_lap(key: &KeyEvent) -> bool {
    key.kind == KeyEventKind::Press
        && key.code == KeyCode::Char('l')
        && key.modifiers == KeyModifiers::NONE
}

/// Poll for a key event with timeout.
fn poll_key(timeout_ms: u64) -> Option<KeyEvent> {
    if event::poll(Duration::from_millis(timeout_ms)).ok()? {
        if let Ok(Event::Key(key)) = event::read() {
            return Some(key);
        }
    }
    None
}

/// Run the stopwatch display loop.
pub fn run_stopwatch_loop(sw: &mut Stopwatch) -> anyhow::Result<SessionData> {
    terminal::enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, cursor::Hide)?;

    sw.start()?;
    println!("{}", style::style("▶ Stopwatch started").bold().green());
    print_controls("stopwatch");

    let mut paused = false;
    let mut done = false;
    let mut final_elapsed = 0.0;
    let mut status = "completed";

    while !done {
        if let Some(key) = poll_key(50) {
            if is_quit(&key) {
                done = true;
                final_elapsed = sw.stop();
                status = "interrupted";
            } else if is_space(&key) {
                if paused {
                    let _ = sw.resume();
                    paused = false;
                } else if sw.state() == TimerState::Running {
                    let _ = sw.pause();
                    paused = true;
                }
            } else if is_lap(&key) {
                if let Ok(lap) = sw.lap() {
                    println!(
                        "{} {} {}",
                        style::style(format!("  Lap {}:", lap.lap)).bold().yellow(),
                        style::style(format_duration(lap.split)).green(),
                        style::style(format!("(total: {})", format_duration(lap.total))).cyan()
                    );
                }
            }
        }

        if !done {
            let elapsed = sw.elapsed();
            let display = format_duration(elapsed);
            queue!(
                stdout,
                cursor::MoveToColumn(0),
                terminal::Clear(ClearType::CurrentLine),
            )?;
            write!(
                stdout,
                "  {} {}",
                style::style("⏱️").cyan(),
                style::style(display).bold().cyan()
            )?;
            stdout.flush()?;
        }
    }

    println!();
    terminal::disable_raw_mode()?;
    execute!(stdout, cursor::Show)?;

    let laps = sw
        .laps()
        .iter()
        .map(|l| SessionLap {
            lap: l.lap,
            split: l.split,
            total: l.total,
        })
        .collect::<Vec<_>>();

    Ok(SessionData {
        session_type: "stopwatch".to_string(),
        started: chrono::Utc::now().to_rfc3339(),
        ended: chrono::Utc::now().to_rfc3339(),
        duration_seconds: (final_elapsed * 1000.0).round() / 1000.0,
        status: status.to_string(),
        laps: Some(laps),
        cycles: None,
    })
}

/// Run the countdown display loop.
pub fn run_countdown_loop(cd: &mut Countdown) -> anyhow::Result<SessionData> {
    terminal::enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, cursor::Hide)?;

    cd.start()?;
    println!(
        "{} {}",
        style::style("▶ Countdown started:").bold().green(),
        style::style(format_duration(cd.duration())).bold()
    );
    print_controls("countdown");

    let mut paused = false;
    let mut status = "completed";
    let mut final_duration = cd.duration();

    loop {
        if let Some(key) = poll_key(100) {
            if is_quit(&key) {
                final_duration = cd.stop();
                status = "interrupted";
                break;
            } else if is_space(&key) {
                if paused {
                    let _ = cd.resume();
                    paused = false;
                } else if cd.state() == TimerState::Running {
                    let _ = cd.pause();
                    paused = true;
                }
            }
        }

        if cd.is_finished() {
            println!();
            println!("{}", style::style("⏰ Time's up!").bold().green());
            send_notification("Termino", "Countdown finished!");
            break;
        }

        if cd.state() == TimerState::Stopped {
            break;
        }

        let remaining = cd.remaining();
        let elapsed = cd.elapsed();
        let duration = cd.duration();
        let progress = if duration > 0.0 {
            (elapsed / duration).min(1.0)
        } else {
            0.0
        };

        let bar_len = 30;
        let filled = (bar_len as f64 * progress) as usize;
        let bar: String = "█".repeat(filled) + &"░".repeat(bar_len - filled);

        queue!(
            stdout,
            cursor::MoveToColumn(0),
            terminal::Clear(ClearType::CurrentLine),
        )?;
        write!(
            stdout,
            "  {} {} {}%",
            style::style(format_duration(remaining)).bold().yellow(),
            style::style(bar).cyan(),
            style::style((progress * 100.0) as u32).cyan()
        )?;
        stdout.flush()?;
    }

    println!();
    terminal::disable_raw_mode()?;
    execute!(stdout, cursor::Show)?;

    Ok(SessionData {
        session_type: "countdown".to_string(),
        started: chrono::Utc::now().to_rfc3339(),
        ended: chrono::Utc::now().to_rfc3339(),
        duration_seconds: (final_duration * 1000.0).round() / 1000.0,
        status: status.to_string(),
        laps: None,
        cycles: None,
    })
}

/// Run the pomodoro display loop.
pub fn run_pomodoro_loop(pt: &mut PomodoroTimer) -> anyhow::Result<SessionData> {
    let mut total_duration = 0.0;
    let mut status = "completed".to_string();

    while !pt.is_complete() {
        let phase_name = match pt.phase {
            PomodoroPhase::Work => "Work",
            PomodoroPhase::Break => "Break",
            PomodoroPhase::LongBreak => "Long Break",
        };
        let dur = pt.get_current_duration();
        let mut cd = Countdown::new(dur);

        terminal::enable_raw_mode()?;
        let mut stdout = stdout();
        execute!(stdout, cursor::Hide)?;

        cd.start()?;
        let cycle_label = if pt.phase == PomodoroPhase::Work {
            format!("Cycle {}", pt.completed_cycles() + 1)
        } else {
            "Break".to_string()
        };
        println!(
            "\n{} — {} ({})",
            style::style(cycle_label).bold(),
            phase_name,
            format_duration(dur)
        );
        print_controls("pomodoro");

        let mut paused = false;
        let mut phase_interrupted = false;

        loop {
            if let Some(key) = poll_key(100) {
                if is_quit(&key) {
                    cd.stop();
                    pt.record_cycle(cd.elapsed());
                    total_duration += cd.elapsed();
                    status = "interrupted".to_string();
                    phase_interrupted = true;
                    println!();
                    println!("{}", style::style("Pomodoro interrupted.").yellow());
                    break;
                } else if is_space(&key) {
                    if paused {
                        let _ = cd.resume();
                        paused = false;
                    } else if cd.state() == TimerState::Running {
                        let _ = cd.pause();
                        paused = true;
                    }
                }
            }

            if cd.is_finished() {
                println!();
                if pt.phase == PomodoroPhase::Work {
                    send_notification(
                        "Termino",
                        &format!(
                            "Work cycle {} complete! Take a break.",
                            pt.completed_cycles() + 1
                        ),
                    );
                } else {
                    send_notification("Termino", "Break over! Time to focus.");
                }
                pt.record_cycle(dur);
                total_duration += dur;
                pt.advance_phase();
                break;
            }

            if cd.state() == TimerState::Stopped {
                break;
            }

            let remaining = cd.remaining();
            let elapsed = cd.elapsed();
            let progress = if dur > 0.0 {
                (elapsed / dur).min(1.0)
            } else {
                0.0
            };

            let (phase_color, emoji) = match pt.phase {
                PomodoroPhase::Work => ("green", "🍅"),
                PomodoroPhase::Break => ("yellow", "☕"),
                PomodoroPhase::LongBreak => ("yellow", "☕"),
            };

            queue!(
                stdout,
                cursor::MoveToColumn(0),
                terminal::Clear(ClearType::CurrentLine),
            )?;
            write!(
                stdout,
                "  {} {} {}%",
                emoji,
                style::style(format_duration(remaining)).with(match phase_color {
                    "green" => style::Color::Green,
                    "yellow" => style::Color::Yellow,
                    _ => style::Color::White,
                }),
                style::style((progress * 100.0) as u32).cyan()
            )?;
            stdout.flush()?;
        }

        terminal::disable_raw_mode()?;
        execute!(stdout, cursor::Show)?;

        if phase_interrupted {
            break;
        }
    }

    if status != "interrupted" {
        println!();
        println!(
            "{}",
            style::style("🎉 Pomodoro session complete!").bold().green()
        );
        send_notification("Termino", "Pomodoro session complete! Great work!");
    }

    let cycles: Vec<SessionCycle> = pt
        .cycles
        .iter()
        .map(|c| SessionCycle {
            cycle: c.cycle,
            cycle_type: c.cycle_type.clone(),
            duration: c.duration,
        })
        .collect();

    Ok(SessionData {
        session_type: "pomodoro".to_string(),
        started: chrono::Utc::now().to_rfc3339(),
        ended: chrono::Utc::now().to_rfc3339(),
        duration_seconds: (total_duration * 1000.0).round() / 1000.0,
        status,
        laps: None,
        cycles: Some(cycles),
    })
}

/// Print a session summary after completion.
pub fn print_session_summary(data: &SessionData) {
    println!();
    println!("{}", style::style("✓ Session Complete").bold().green());
    println!("  Type:     {}", data.session_type);
    println!("  Duration: {}", format_duration(data.duration_seconds));
    println!("  Status:   {}", data.status);

    if let Some(ref laps) = data.laps {
        if !laps.is_empty() {
            println!("\n  Laps:");
            for lap in laps {
                println!(
                    "    Lap #{}: split={}, total={}",
                    lap.lap,
                    format_duration(lap.split),
                    format_duration(lap.total)
                );
            }
        }
    }

    if let Some(ref cycles) = data.cycles {
        if !cycles.is_empty() {
            println!("\n  Pomodoro Cycles:");
            for cycle in cycles {
                println!(
                    "    Cycle {} ({}): {}",
                    cycle.cycle,
                    cycle.cycle_type,
                    format_duration(cycle.duration)
                );
            }
        }
    }

    println!();
}

/// Print session history as a table.
pub fn print_history(sessions: &[SessionData], limit: usize) {
    if sessions.is_empty() {
        println!("{}", style::style("No sessions found.").yellow());
        return;
    }

    println!("\nSession History (last {}):", limit.min(sessions.len()));
    for (i, session) in sessions.iter().enumerate().take(limit) {
        let status_style = match session.status.as_str() {
            "completed" => style::style(&session.status).green(),
            "interrupted" => style::style(&session.status).red(),
            _ => style::style(&session.status).yellow(),
        };
        println!(
            "  {}. {} | {} | {} | {}",
            i + 1,
            style::style(&session.session_type).cyan(),
            format_duration(session.duration_seconds),
            &session.started[..19],
            status_style
        );
    }
    println!("\nTotal sessions: {}", sessions.len());
}
