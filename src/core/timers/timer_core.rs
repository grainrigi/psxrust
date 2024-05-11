use serde::{Deserialize, Serialize};

use super::{timer_videotimings::TimerVideoTimings, TimerPort};

const TIMER_MODE_MASK: u16 = 0x07FF;
const TIMER_SYNC_ENABLE: u16 = 0x0001;
const TIMER_RESET_MODE: u16 = 0x0008;
const TIMER_REACHED_TARGET: u16 = 0x1000;
const TIMER_REACHED_FFFF: u16 = 0x2000;
const TIMER_IRQ_ON_TARGET: u16 = 0x0010;
const TIMER_IRQ_ON_FFFF: u16 = 0x0020;
const TIMER_IRQ_REPEATED: u16 = 0x0040;
const TIMER_IRQ_TOGGLE: u16 = 0x0080;
const TIMER_IRQ_BIT: u16 = 0x0400;

#[derive(Clone, Serialize, Deserialize)]
pub struct Timer<T: TimerClockSource> {
    count: u16,
    freeze: bool,

    target: u16,

    mode: u16,
    reached_target: bool,
    reached_ffff: bool,
    interrupt_raised: bool,
    interrupt_raised_onshot: bool,

    source: T,
}

impl<T: TimerClockSource> Timer<T> {
    pub fn new(source: T) -> Timer<T> {
        Timer {
            count: 0,
            freeze: true,
            target: 0,
            mode: 0 | TIMER_IRQ_BIT,
            reached_target: false,
            reached_ffff: false,
            interrupt_raised: false,
            interrupt_raised_onshot: false,
            source,
        }
    }

    pub fn mutate(&mut self) {
        self.interrupt_raised = false;
        if self.mode & TIMER_IRQ_TOGGLE == 0 {
            self.mode |= TIMER_IRQ_BIT;
        }
        // clock source process
        let action = self.source.cycle(&TimerClockParams {
            source: self.clock_source(),
            sync: self.clock_sync(),
            vtiming: &TimerVideoTimings::new(),
        });
        if action.countup {
            let mut raise_interrupt = false;
            // count process
            if !self.freeze {
                if self.reset_on_target() {
                    if self.count == self.target {
                        self.count = 0;
                        self.freeze = true;
                    } else {
                        self.count = self.count.wrapping_add(1);
                    }
                    if self.count == self.target {
                        self.reached_target = true;
                        if self.irq_on_target() {
                            raise_interrupt = true;
                        }
                    }
                } else {
                    self.count = self.count.wrapping_add(1);
                    if self.count == 0xFFFF {
                        self.reached_ffff = true;
                        if self.irq_on_ffff() {
                            raise_interrupt = true;
                        }
                    }
                }
                if raise_interrupt {
                    self.raise_interrupt();
                }
            } else {
                self.freeze = false;
            }
        }
        if action.reset {
            self.count = 0;
        }
        if action.freerun {
            self.mode &= !TIMER_SYNC_ENABLE;
        }
    }

    fn raise_interrupt(&mut self) {
        if self.interrupt_raised_onshot {
            return;
        }

        let prev_mode = self.mode;
        if self.mode & TIMER_IRQ_TOGGLE != 0 {
            self.mode ^= TIMER_IRQ_BIT;
        } else {
            // 0がIRQ状態を表す点に注意
            self.mode &= !TIMER_IRQ_BIT;
        }

        if prev_mode & TIMER_IRQ_BIT != 0 && self.mode & TIMER_IRQ_BIT == 0 {
            // 1 -> 0 にセットされたときに例外発生
            self.interrupt_raised = true;
            if self.mode & TIMER_IRQ_REPEATED == 0 {
                self.interrupt_raised_onshot = true;
            }
        }
    }

    fn clock_source(&self) -> u16 {
        (self.mode >> 8) & 0x03
    }

    fn clock_sync(&self) -> Option<u16> {
        if self.mode & TIMER_SYNC_ENABLE != 0 {
            Some((self.mode >> 1) & 0x03)
        } else {
            None
        }
    }

    fn reset_on_target(&self) -> bool {
        (self.mode & TIMER_RESET_MODE) != 0
    }
    fn irq_on_target(&self) -> bool {
        (self.mode & TIMER_IRQ_ON_TARGET) != 0
    }
    fn irq_on_ffff(&self) -> bool {
        (self.mode & TIMER_IRQ_ON_FFFF) != 0
    }

    pub fn interrupt_raised(&self) -> bool {
        self.interrupt_raised
    }
}

impl<T: TimerClockSource> TimerPort for Timer<T> {
    fn read_count(&self) -> u32 {
        self.count as u32
    }

    fn write_count(&mut self, count: u32) {
        self.count = count as u16;
        self.freeze = true;
    }

    fn read_mode(&self) -> u32 {
        ((self.mode & TIMER_MODE_MASK)
            | if self.reached_target {
                TIMER_REACHED_TARGET
            } else {
                0
            }
            | if self.reached_ffff {
                TIMER_REACHED_FFFF
            } else {
                0
            }) as u32
    }

    fn write_mode(&mut self, control: u32) {
        self.count = 0;
        self.interrupt_raised_onshot = false;
        self.mode = control as u16 | TIMER_IRQ_BIT;
        self.freeze = true;
        self.source.reset();
    }

    fn commit_read_mode(&mut self) {
        self.reached_target = false;
        self.reached_ffff = false;
    }

    fn read_target(&self) -> u32 {
        self.target as u32
    }

    fn write_target(&mut self, target: u32) {
        self.target = target as u16;
    }
}

pub struct TimerClockParams<'a> {
    pub source: u16,
    pub sync: Option<u16>,
    pub vtiming: &'a TimerVideoTimings,
}

pub struct TimerClockSourceAction {
    pub countup: bool,
    pub reset: bool,
    pub freerun: bool,
}

impl TimerClockSourceAction {
    pub fn countup(value: bool) -> TimerClockSourceAction {
        TimerClockSourceAction {
            countup: value,
            reset: false,
            freerun: false,
        }
    }
}

pub trait TimerClockSource {
    fn cycle(&mut self, params: &TimerClockParams) -> TimerClockSourceAction;
    fn reset(&mut self);
}

#[cfg(test)]
mod tests {
    use crate::core::timers::timer_sysclock::new_sysclock_timer;

    use super::*;

    #[test]
    fn test_sysclock_wrap_target() {
        // let mut timer = SysclockTimer::new();
        let mut timer = new_sysclock_timer();

        // cycle1
        timer.mutate();
        timer.write_target(0x0002);
        timer.write_mode(0x0008);
        // cycle2
        assert_eq!(timer.read_count(), 0);
        timer.mutate();
        // cycle3
        assert_eq!(timer.read_count(), 0);
        timer.mutate();
        // cycle4
        assert_eq!(timer.read_count(), 1);
        timer.mutate();
        // cycle5
        assert_eq!(timer.read_count(), 2);
        assert_eq!(timer.read_mode() & 0x1000, 0x1000);
        timer.mutate();
        // cycle6
        assert_eq!(timer.read_count(), 0);
        timer.mutate();
        // cycle7
        assert_eq!(timer.read_count(), 0);
        timer.mutate();
        // cycle8
        assert_eq!(timer.read_count(), 1);
    }

    #[test]
    fn test_sysclock_wrap_ffff() {
        // let mut timer = SysclockTimer::new();
        let mut timer = new_sysclock_timer();

        // cycle1
        timer.mutate();
        timer.write_mode(0x0000);
        // cycle2
        assert_eq!(timer.read_count(), 0);
        timer.mutate();
        // cycle3
        assert_eq!(timer.read_count(), 0);
        timer.mutate();
        // cycle4-65537
        for i in 4..=65537 {
            assert_eq!(timer.read_count(), i - 3);
            timer.mutate();
        }
        // cycle65538
        assert_eq!(timer.read_count(), 0xFFFF);
        timer.mutate();
        // cycle65539
        assert_eq!(timer.read_count(), 0);
        timer.mutate();
        // cycle65540
        assert_eq!(timer.read_count(), 1);
    }

    #[test]
    fn test_irq_pulse_oneshot() {
        let mut timer = new_sysclock_timer();

        // cycle1
        timer.mutate();
        timer.write_target(0x0002);
        timer.commit_read_mode();
        // Pulse, Repeat, IRQ on target, Reset on target
        timer.write_mode(0x0418);
        // cycle 2
        timer.mutate();
        // cycle 3
        assert_eq!(timer.read_mode(), 0x0418);
        assert_eq!(timer.read_count(), 0);
        timer.mutate();
        // cycle 4
        assert_eq!(timer.read_mode(), 0x0418);
        assert_eq!(timer.read_count(), 1);
        timer.mutate();
        // cycle 5
        // reaced target, irq raised(irq bit = 0)
        assert_eq!(timer.read_mode(), 0x1018);
        assert_eq!(timer.read_count(), 2);
        assert_eq!(timer.interrupt_raised(), true);
        timer.mutate();
        timer.commit_read_mode();
        // cycle 6
        assert_eq!(timer.read_mode(), 0x0418);
        assert_eq!(timer.read_count(), 0);
        assert_eq!(timer.interrupt_raised(), false);
        timer.mutate();
        // cycle 7
        assert_eq!(timer.read_mode(), 0x0418);
        assert_eq!(timer.read_count(), 0);
        timer.mutate();
        // cycle 8
        assert_eq!(timer.read_mode(), 0x0418);
        assert_eq!(timer.read_count(), 1);
        timer.mutate();
        // cycle 9
        // irq won't be triggered
        assert_eq!(timer.read_mode(), 0x1418);
        assert_eq!(timer.read_count(), 2);
        assert_eq!(timer.interrupt_raised(), false);

        // then reset the one-shot flag by writing mode
        timer.mutate();
        timer.commit_read_mode();
        timer.write_mode(0x0418);
        // cycle 10
        assert_eq!(timer.read_mode(), 0x0418);
        assert_eq!(timer.read_count(), 0);
        timer.mutate();
        // cycle 11
        assert_eq!(timer.read_mode(), 0x0418);
        assert_eq!(timer.read_count(), 0);
        timer.mutate();
        // cycle 12
        assert_eq!(timer.read_mode(), 0x0418);
        assert_eq!(timer.read_count(), 1);
        timer.mutate();
        // cycle 13
        // irq is triggered again
        assert_eq!(timer.read_mode(), 0x1018);
        assert_eq!(timer.read_count(), 2);
    }

    #[test]
    fn test_irq_repeat() {
        let mut timer = new_sysclock_timer();

        // cycle1
        timer.mutate();
        timer.write_target(0x0002);
        // Pulse, One-shot, IRQ on target, Reset on target
        timer.write_mode(0x0058);
        // cycle 2
        timer.mutate();
        // cycle 3
        assert_eq!(timer.read_mode(), 0x0458);
        assert_eq!(timer.read_count(), 0);
        timer.mutate();
        // cycle 4
        assert_eq!(timer.read_mode(), 0x0458);
        assert_eq!(timer.read_count(), 1);
        timer.mutate();
        // cycle 5
        // reaced target, irq raised(irq bit = 0)
        assert_eq!(timer.read_mode(), 0x1058);
        assert_eq!(timer.read_count(), 2);
        assert_eq!(timer.interrupt_raised(), true);
        timer.mutate();
        timer.commit_read_mode();
        // cycle 6
        assert_eq!(timer.read_mode(), 0x0458);
        assert_eq!(timer.read_count(), 0);
        assert_eq!(timer.interrupt_raised(), false);
        timer.mutate();
        // cycle 7
        assert_eq!(timer.read_mode(), 0x0458);
        assert_eq!(timer.read_count(), 0);
        timer.mutate();
        // cycle 8
        assert_eq!(timer.read_mode(), 0x0458);
        assert_eq!(timer.read_count(), 1);
        timer.mutate();
        // cycle 9
        assert_eq!(timer.read_mode(), 0x1058);
        assert_eq!(timer.read_count(), 2);
        // interrupt is re-raised without reset
        assert_eq!(timer.interrupt_raised(), true);
    }

    #[test]
    fn test_irq_toggle() {
        let mut timer = new_sysclock_timer();

        // cycle1
        timer.mutate();
        timer.write_target(0x0002);
        // Toggle, Repeat, IRQ on target, Reset on target
        timer.write_mode(0x00D8);
        // cycle 2
        assert_eq!(timer.read_mode(), 0x04D8);
        assert_eq!(timer.read_count(), 0);
        timer.mutate();
        // cycle 3
        assert_eq!(timer.read_mode(), 0x04D8);
        assert_eq!(timer.read_count(), 0);
        timer.mutate();
        // cycle 4
        assert_eq!(timer.read_mode(), 0x04D8);
        assert_eq!(timer.read_count(), 1);
        timer.mutate();
        // cycle 5
        assert_eq!(timer.read_mode(), 0x10D8);
        assert_eq!(timer.read_count(), 2);
        // irq went 1 -> 0 so interrupt should be raised
        assert_eq!(timer.interrupt_raised(), true);
        timer.mutate();
        timer.commit_read_mode();
        // cycle 6
        assert_eq!(timer.read_mode(), 0x00D8);
        assert_eq!(timer.read_count(), 0);
        timer.mutate();
        // cycle 7
        assert_eq!(timer.read_mode(), 0x00D8);
        assert_eq!(timer.read_count(), 0);
        timer.mutate();
        // cycle 8
        assert_eq!(timer.read_mode(), 0x00D8);
        assert_eq!(timer.read_count(), 1);
        timer.mutate();
        // cycle 9
        assert_eq!(timer.read_mode(), 0x14D8);
        assert_eq!(timer.read_count(), 2);
        // irq went 0 -> 1 so interrupt won't be raised
        assert_eq!(timer.interrupt_raised(), false);
        timer.mutate();
        timer.commit_read_mode();
        // cycle 10
        assert_eq!(timer.read_mode(), 0x04D8);
        assert_eq!(timer.read_count(), 0);
        timer.mutate();
        // cycle 11
        assert_eq!(timer.read_mode(), 0x04D8);
        assert_eq!(timer.read_count(), 0);
        timer.mutate();
        // cycle 12
        assert_eq!(timer.read_mode(), 0x04D8);
        assert_eq!(timer.read_count(), 1);
        timer.mutate();
        // cycle 13
        assert_eq!(timer.read_mode(), 0x10D8);
        assert_eq!(timer.read_count(), 2);
        // 1 -> 0 again
        assert_eq!(timer.interrupt_raised(), true);
    }
}
