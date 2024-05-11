pub trait TimerPort {
    fn read_count(&self) -> u32;
    fn write_count(&mut self, count: u32);

    fn read_mode(&self) -> u32;
    fn write_mode(&mut self, control: u32);
    fn commit_read_mode(&mut self);

    fn read_target(&self) -> u32;
    fn write_target(&mut self, target: u32);
}

pub mod timer_core;
pub mod timer_dotclock;
pub mod timer_hretrace;
pub mod timer_sysclock;
pub mod timer_videotimings;

use serde::{Deserialize, Serialize};
use timer_core::Timer;
use timer_dotclock::DotclockSource;
use timer_hretrace::HretraceSource;
use timer_sysclock::SysclockSource;

use super::MachineMutation;

#[derive(Clone, Serialize, Deserialize)]
pub struct Timers {
    pub dotclock: Timer<DotclockSource>,
    pub hretrace: Timer<HretraceSource>,
    pub sysclock: Timer<SysclockSource>,
}

impl Timers {
    pub fn new() -> Timers {
        Timers {
            dotclock: timer_dotclock::new_dotclock_timer(),
            hretrace: timer_hretrace::new_hretrace_timer(),
            sysclock: timer_sysclock::new_sysclock_timer(),
        }
    }

    // must be called before any write
    pub fn mutate(&mut self, mu: &MachineMutation) {
        self.dotclock.mutate();
        self.hretrace.mutate();
        self.sysclock.mutate();
        match mu.timer_mode_read {
            Some(addr) => match (addr >> 4) & 0x0F {
                0 => self.dotclock.commit_read_mode(),
                1 => self.hretrace.commit_read_mode(),
                2 => self.sysclock.commit_read_mode(),
                _ => {}
            },
            _ => {}
        }
    }
}
