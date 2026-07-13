use serde::{Deserialize, Serialize};

/// Pomodoro timer phases.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum PomodoroPhase {
    #[serde(rename = "work")]
    Work,
    #[serde(rename = "break")]
    Break,
    #[serde(rename = "long_break")]
    LongBreak,
}

/// A recorded pomodoro cycle.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PomodoroCycle {
    pub cycle: u32,
    #[serde(rename = "type")]
    pub cycle_type: String,
    pub duration: f64,
}

/// Pomodoro timer with work/break cycling.
#[derive(Debug)]
pub struct PomodoroTimer {
    pub work_duration: f64,
    pub break_duration: f64,
    pub long_break_duration: f64,
    pub cycles_target: u32,
    pub current_cycle: u32,
    pub phase: PomodoroPhase,
    pub cycles: Vec<PomodoroCycle>,
    pub completed_cycles: u32,
}

impl PomodoroTimer {
    pub fn new(
        work_duration: f64,
        break_duration: f64,
        long_break_duration: f64,
        cycles_target: u32,
    ) -> Self {
        Self {
            work_duration,
            break_duration,
            long_break_duration,
            cycles_target,
            current_cycle: 0,
            phase: PomodoroPhase::Work,
            cycles: Vec::new(),
            completed_cycles: 0,
        }
    }

    pub fn completed_cycles(&self) -> u32 {
        self.completed_cycles
    }

    pub fn get_current_duration(&self) -> f64 {
        match self.phase {
            PomodoroPhase::Work => self.work_duration,
            PomodoroPhase::LongBreak => self.long_break_duration,
            PomodoroPhase::Break => self.break_duration,
        }
    }

    /// Advance to the next phase.
    pub fn advance_phase(&mut self) -> PomodoroPhase {
        match self.phase {
            PomodoroPhase::Work => {
                self.current_cycle += 1;
                self.completed_cycles += 1;
                if self.current_cycle >= self.cycles_target {
                    self.phase = PomodoroPhase::LongBreak;
                    self.current_cycle = 0;
                } else {
                    self.phase = PomodoroPhase::Break;
                }
            }
            PomodoroPhase::Break | PomodoroPhase::LongBreak => {
                self.phase = PomodoroPhase::Work;
            }
        }
        self.phase
    }

    /// Record a completed cycle segment.
    pub fn record_cycle(&mut self, duration: f64) {
        self.cycles.push(PomodoroCycle {
            cycle: self.completed_cycles,
            cycle_type: match self.phase {
                PomodoroPhase::Work => "work".to_string(),
                PomodoroPhase::Break => "break".to_string(),
                PomodoroPhase::LongBreak => "long_break".to_string(),
            },
            duration,
        });
    }

    /// Check if the full pomodoro session is complete.
    pub fn is_complete(&self) -> bool {
        self.completed_cycles >= self.cycles_target && self.phase == PomodoroPhase::Break
    }
}