use serde::{Deserialize, Serialize};

use super::timer_core::{Timer, TimerClockParams, TimerClockSource, TimerClockSourceAction};

#[derive(Clone, Serialize, Deserialize)]
pub struct HretraceSource {}

impl TimerClockSource for HretraceSource {
    fn cycle(&mut self, params: &TimerClockParams) -> TimerClockSourceAction {
        let mut pause = false;
        let mut action = TimerClockSourceAction::countup(false);
        match params.sync {
            Some(0) => pause = params.vtiming.is_in_vblank(),
            Some(1) => action.reset = params.vtiming.is_vblank_edge(),
            Some(2) => {
                action.reset = params.vtiming.is_vblank_edge();
                pause = !params.vtiming.is_in_vblank();
            }
            Some(3) => {
                if params.vtiming.is_vblank_edge() {
                    action.freerun = true;
                }
            }
            _ => {}
        };
        if !pause {
            match params.source {
                0 | 2 => action.countup = true,
                1 | 3 => action.countup = params.vtiming.is_hblank_edge(),
                _ => {}
            };
        }
        action
    }

    fn reset(&mut self) {}
}

pub fn new_hretrace_timer() -> Timer<HretraceSource> {
    Timer::new(HretraceSource {})
}
