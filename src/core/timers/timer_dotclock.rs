use serde::{Deserialize, Serialize};

use super::timer_core::{Timer, TimerClockParams, TimerClockSource, TimerClockSourceAction};

#[derive(Clone, Serialize, Deserialize)]
pub struct DotclockSource {}

impl DotclockSource {
    pub fn new() -> Self {
        DotclockSource {}
    }
}

impl TimerClockSource for DotclockSource {
    fn cycle(&mut self, params: &TimerClockParams) -> TimerClockSourceAction {
        let mut pause = false;
        let mut action = TimerClockSourceAction::countup(false);
        match params.sync {
            Some(0) => pause = params.vtiming.is_in_hblank(),
            Some(1) => action.reset = params.vtiming.is_hblank_edge(),
            Some(2) => {
                action.reset = params.vtiming.is_hblank_edge();
                pause = !params.vtiming.is_in_hblank();
            }
            Some(3) => {
                if params.vtiming.is_hblank_edge() {
                    action.freerun = true;
                }
            }
            _ => {}
        };
        if !pause {
            match params.source {
                0 | 2 => action.countup = true,
                1 | 3 => action.countup = params.vtiming.is_dotclock_edge(),
                _ => {}
            };
        }
        action
    }

    fn reset(&mut self) {}
}

pub fn new_dotclock_timer() -> Timer<DotclockSource> {
    Timer::new(DotclockSource {})
}

mod test {
    use crate::core::timers::timer_videotimings::TimerVideoTimings;

    use super::*;

    fn mkparams(sync: u16, vtiming: &TimerVideoTimings) -> TimerClockParams {
        TimerClockParams {
            source: 0,
            sync: Some(sync),
            vtiming: vtiming,
        }
    }

    #[test]
    fn test_sync() {
        let mut source = DotclockSource::new();
        let mut vtiming = TimerVideoTimings {
            dotclock_edge: false,
            hblank_edge: false,
            in_hblank: false,
            in_vblank: false,
            vblank_edge: false,
        };
        // scenario1: sync=0(pause during hblank)
        // cycle1 before hblank edge
        let action = source.cycle(&mkparams(0, &vtiming));
        assert_eq!(action.countup, true);
        assert_eq!(action.reset, false);
        assert_eq!(action.freerun, false);
        // cycle1 hblank edge (count stop)
        vtiming.hblank_edge = true;
        vtiming.in_hblank = true;
        let action = source.cycle(&mkparams(0, &vtiming));
        assert_eq!(action.countup, false);
        assert_eq!(action.reset, false);
        assert_eq!(action.freerun, false);
        // cycle 2 after hblank edge
        vtiming.hblank_edge = false;
        let action = source.cycle(&mkparams(0, &vtiming));
        assert_eq!(action.countup, false);
        assert_eq!(action.reset, false);
        assert_eq!(action.freerun, false);
    }
}
