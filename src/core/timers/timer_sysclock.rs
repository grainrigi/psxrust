use serde::{Deserialize, Serialize};

use super::timer_core::{Timer, TimerClockParams, TimerClockSource, TimerClockSourceAction};

#[derive(Clone, Serialize, Deserialize)]
pub struct SysclockSource {
    cycles: u32,
}

impl TimerClockSource for SysclockSource {
    fn cycle(&mut self, params: &TimerClockParams) -> TimerClockSourceAction {
        match params.sync {
            Some(0) | Some(3) => TimerClockSourceAction::countup(false),
            _ => match params.source {
                0 | 1 => TimerClockSourceAction::countup(true),
                2 | 3 => {
                    self.cycles += 1;
                    if self.cycles >= 7 {
                        self.cycles = 0;
                        TimerClockSourceAction::countup(true)
                    } else {
                        TimerClockSourceAction::countup(false)
                    }
                }
                _ => TimerClockSourceAction::countup(false),
            },
        }
    }

    fn reset(&mut self) {
        self.cycles = 0;
    }
}

pub fn new_sysclock_timer() -> Timer<SysclockSource> {
    Timer::new(SysclockSource { cycles: 0 })
}
