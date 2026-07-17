#![allow(clippy::collapsible_if)]

use std::time::Instant;

/// Possible states of a timer.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TimerState {
    Idle,
    Running,
    Paused,
    Stopped,
    Finished,
}

/// A recorded lap.
#[derive(Debug, Clone)]
pub struct Lap {
    pub lap: u32,
    pub split: f64,
    pub total: f64,
}

/// A stopwatch with lap timing capability.
#[derive(Debug)]
pub struct Stopwatch {
    state: TimerState,
    start_time: Option<Instant>,
    elapsed: f64,
    pause_start: Option<Instant>,
    laps: Vec<Lap>,
}

impl Stopwatch {
    pub fn new() -> Self {
        Self {
            state: TimerState::Idle,
            start_time: None,
            elapsed: 0.0,
            pause_start: None,
            laps: Vec::new(),
        }
    }

    pub fn state(&self) -> TimerState {
        self.state
    }

    pub fn elapsed(&self) -> f64 {
        match self.state {
            TimerState::Running => {
                self.elapsed
                    + self
                        .start_time
                        .map(|t| t.elapsed().as_secs_f64())
                        .unwrap_or(0.0)
            }
            _ => self.elapsed,
        }
    }

    pub fn laps(&self) -> &[Lap] {
        &self.laps
    }

    pub fn start(&mut self) -> anyhow::Result<()> {
        if self.state != TimerState::Idle && self.state != TimerState::Stopped {
            anyhow::bail!("Cannot start from state {:?}", self.state);
        }
        self.start_time = Some(Instant::now());
        self.elapsed = 0.0;
        self.laps.clear();
        self.state = TimerState::Running;
        Ok(())
    }

    pub fn pause(&mut self) -> anyhow::Result<()> {
        if self.state != TimerState::Running {
            anyhow::bail!("Cannot pause from state {:?}", self.state);
        }
        if let Some(st) = self.start_time {
            self.elapsed += st.elapsed().as_secs_f64();
        }
        self.pause_start = Some(Instant::now());
        self.state = TimerState::Paused;
        Ok(())
    }

    pub fn resume(&mut self) -> anyhow::Result<()> {
        if self.state != TimerState::Paused {
            anyhow::bail!("Cannot resume from state {:?}", self.state);
        }
        self.start_time = Some(Instant::now());
        self.pause_start = None;
        self.state = TimerState::Running;
        Ok(())
    }

    pub fn stop(&mut self) -> f64 {
        if self.state == TimerState::Running {
            if let Some(st) = self.start_time {
                self.elapsed += st.elapsed().as_secs_f64();
            }
        }
        self.state = TimerState::Stopped;
        self.elapsed
    }

    pub fn lap(&mut self) -> anyhow::Result<Lap> {
        if self.state != TimerState::Running {
            anyhow::bail!("Cannot record lap when stopwatch is not running");
        }
        let current = self.elapsed()
            + self
                .start_time
                .map(|t| t.elapsed().as_secs_f64())
                .unwrap_or(0.0);
        let prev_total = self.laps.last().map(|l| l.total).unwrap_or(0.0);
        let lap_number = self.laps.len() as u32 + 1;
        let lap = Lap {
            lap: lap_number,
            split: current - prev_total,
            total: current,
        };
        self.laps.push(lap.clone());
        Ok(lap)
    }
}

impl Default for Stopwatch {
    fn default() -> Self {
        Self::new()
    }
}

/// A countdown timer that runs for a specified duration.
#[derive(Debug)]
pub struct Countdown {
    duration: f64,
    state: TimerState,
    start_time: Option<Instant>,
    elapsed: f64,
    pause_start: Option<Instant>,
}

impl Countdown {
    pub fn new(duration: f64) -> Self {
        Self {
            duration,
            state: TimerState::Idle,
            start_time: None,
            elapsed: 0.0,
            pause_start: None,
        }
    }

    pub fn state(&self) -> TimerState {
        self.state
    }

    pub fn duration(&self) -> f64 {
        self.duration
    }

    pub fn remaining(&self) -> f64 {
        match self.state {
            TimerState::Running => {
                let elapsed = self.elapsed
                    + self
                        .start_time
                        .map(|t| t.elapsed().as_secs_f64())
                        .unwrap_or(0.0);
                (self.duration - elapsed).max(0.0)
            }
            _ => (self.duration - self.elapsed).max(0.0),
        }
    }

    pub fn elapsed(&self) -> f64 {
        match self.state {
            TimerState::Running => {
                self.elapsed
                    + self
                        .start_time
                        .map(|t| t.elapsed().as_secs_f64())
                        .unwrap_or(0.0)
            }
            _ => self.elapsed,
        }
    }

    pub fn is_finished(&self) -> bool {
        self.remaining() <= 0.0
    }

    pub fn start(&mut self) -> anyhow::Result<()> {
        if self.state != TimerState::Idle && self.state != TimerState::Stopped {
            anyhow::bail!("Cannot start from state {:?}", self.state);
        }
        self.start_time = Some(Instant::now());
        self.elapsed = 0.0;
        self.state = TimerState::Running;
        Ok(())
    }

    pub fn pause(&mut self) -> anyhow::Result<()> {
        if self.state != TimerState::Running {
            anyhow::bail!("Cannot pause from state {:?}", self.state);
        }
        if let Some(st) = self.start_time {
            self.elapsed += st.elapsed().as_secs_f64();
        }
        self.pause_start = Some(Instant::now());
        self.state = TimerState::Paused;
        Ok(())
    }

    pub fn resume(&mut self) -> anyhow::Result<()> {
        if self.state != TimerState::Paused {
            anyhow::bail!("Cannot resume from state {:?}", self.state);
        }
        self.start_time = Some(Instant::now());
        self.pause_start = None;
        self.state = TimerState::Running;
        Ok(())
    }

    pub fn stop(&mut self) -> f64 {
        if self.state == TimerState::Running {
            if let Some(st) = self.start_time {
                self.elapsed += st.elapsed().as_secs_f64();
            }
        }
        self.state = TimerState::Stopped;
        self.elapsed
    }
}
