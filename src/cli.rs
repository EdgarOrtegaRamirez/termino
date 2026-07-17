use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "termino",
    version = "0.1.0",
    about = "⏱️ A sleek terminal-based timer and stopwatch CLI tool"
)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Start a stopwatch with lap timing
    Stopwatch,
    /// Start a countdown timer
    Countdown {
        /// Countdown duration (e.g., 5m, 1h30m, 90s)
        #[arg(default_value = "25m")]
        duration: String,
    },
    /// Start a pomodoro timer with work/break cycling
    Pomodoro {
        /// Work duration in minutes
        #[arg(short = 'w', long, default_value = "25")]
        work: u32,
        /// Break duration in minutes
        #[arg(short = 'b', long, default_value = "5")]
        break_duration: u32,
        /// Number of work cycles before long break
        #[arg(short = 'c', long, default_value = "4")]
        cycles: u32,
        /// Long break duration in minutes
        #[arg(short = 'l', long, default_value = "15")]
        long_break: u32,
    },
    /// Show session history
    History {
        /// Number of sessions to show
        #[arg(short = 'n', long, default_value = "10")]
        limit: usize,
        /// Filter by session type (stopwatch, countdown, pomodoro)
        #[arg(short = 't', long)]
        session_type: Option<String>,
    },
}

/// Parse a duration string like 5m, 1h30m, 90s, 2h into seconds.
pub fn parse_duration(value: &str) -> anyhow::Result<f64> {
    if value.is_empty() {
        anyhow::bail!("Duration cannot be empty");
    }

    let mut total = 0.0;
    let mut i = 0;
    let n = value.len();
    let chars: Vec<char> = value.chars().collect();

    while i < n {
        let start = i;
        while i < n && chars[i].is_ascii_digit() {
            i += 1;
        }
        if start == i {
            anyhow::bail!("Expected number at position {} in '{}'", i, value);
        }
        let num: f64 = value[start..i].parse::<f64>()?;
        if i >= n {
            total += num; // Treat as seconds
            break;
        }
        let unit = chars[i];
        i += 1;
        match unit {
            'h' => total += num * 3600.0,
            'm' => total += num * 60.0,
            's' => total += num,
            _ => anyhow::bail!("Unknown unit '{}' in '{}' (use h, m, s)", unit, value),
        }
    }

    if total <= 0.0 {
        anyhow::bail!("Duration must be positive");
    }
    Ok(total)
}

pub fn run(args: Args) -> anyhow::Result<()> {
    match args.command {
        Commands::Stopwatch => run_stopwatch(),
        Commands::Countdown { duration } => run_countdown(&duration),
        Commands::Pomodoro {
            work,
            break_duration,
            cycles,
            long_break,
        } => run_pomodoro(work, break_duration, cycles, long_break),
        Commands::History {
            limit,
            session_type,
        } => run_history(limit, session_type),
    }
}

fn run_stopwatch() -> anyhow::Result<()> {
    crate::display::print_banner();
    println!("Stopwatch");

    let mut sw = crate::timer::Stopwatch::new();
    let data = crate::display::run_stopwatch_loop(&mut sw)?;
    crate::storage::save_session(&data)?;
    crate::display::print_session_summary(&data);
    Ok(())
}

fn run_countdown(duration_str: &str) -> anyhow::Result<()> {
    let seconds = parse_duration(duration_str)?;
    crate::display::print_banner();
    println!(
        "Countdown: {} ({})",
        duration_str,
        crate::display::format_duration(seconds)
    );

    let mut cd = crate::timer::Countdown::new(seconds);
    let data = crate::display::run_countdown_loop(&mut cd)?;
    crate::storage::save_session(&data)?;
    crate::display::print_session_summary(&data);
    Ok(())
}

fn run_pomodoro(
    work: u32,
    break_duration: u32,
    cycles: u32,
    long_break: u32,
) -> anyhow::Result<()> {
    if work == 0 || break_duration == 0 || cycles == 0 || long_break == 0 {
        anyhow::bail!("All durations and cycles must be positive");
    }

    crate::display::print_banner();
    println!(
        "Pomodoro: {}min work, {}min break, {} cycles, {}min long break",
        work, break_duration, cycles, long_break
    );

    let mut pt = crate::pomodoro::PomodoroTimer::new(
        work as f64 * 60.0,
        break_duration as f64 * 60.0,
        long_break as f64 * 60.0,
        cycles,
    );
    let data = crate::display::run_pomodoro_loop(&mut pt)?;
    crate::storage::save_session(&data)?;
    crate::display::print_session_summary(&data);
    Ok(())
}

fn run_history(limit: usize, session_type: Option<String>) -> anyhow::Result<()> {
    #[allow(clippy::collapsible_if)]
    if let Some(ref st) = session_type {
        if st != "stopwatch" && st != "countdown" && st != "pomodoro" {
            anyhow::bail!(
                "Invalid session type '{}'. Use: stopwatch, countdown, or pomodoro",
                st
            );
        }
    }

    let sessions = crate::storage::get_sessions(session_type.as_deref(), Some(limit))?;
    crate::display::print_banner();
    crate::display::print_history(&sessions, limit);
    Ok(())
}
