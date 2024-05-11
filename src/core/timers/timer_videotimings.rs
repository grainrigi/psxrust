pub struct TimerVideoTimings {
    pub dotclock_edge: bool,
    pub hblank_edge: bool,
    pub in_hblank: bool,
    pub in_vblank: bool,
    pub vblank_edge: bool,
}

impl TimerVideoTimings {
    pub fn new() -> TimerVideoTimings {
        TimerVideoTimings {
            dotclock_edge: false,
            hblank_edge: false,
            in_hblank: false,
            in_vblank: false,
            vblank_edge: false,
        }
    }

    pub fn is_dotclock_edge(&self) -> bool {
        println!("WARN: Stub Video Timing (Dotclock Edge)");
        self.dotclock_edge
    }

    pub fn is_hblank_edge(&self) -> bool {
        println!("WARN: Stub Video Timing (HBlank Edge)");
        self.hblank_edge
    }

    pub fn is_in_hblank(&self) -> bool {
        println!("WARN: Stub Video Timing (In HBlank)");
        self.in_hblank
    }

    pub fn is_in_vblank(&self) -> bool {
        println!("WARN: Stub Video Timing (In VBlank)");
        self.in_vblank
    }

    pub fn is_vblank_edge(&self) -> bool {
        println!("WARN: Stub Video Timing (VBlank Edge)");
        self.vblank_edge
    }
}
