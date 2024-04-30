use crate::core::cpu_inst::reg_name;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct CpuRegfile {
    pub reg: [u32; 32],
    pub lo: u32,
    pub hi: u32,
}

impl CpuRegfile {
    pub fn new() -> CpuRegfile {
        CpuRegfile {
            reg: [0; 32],
            lo: 0,
            hi: 0,
        }
    }

    pub fn load_gpr(&self, idx: u8) -> u32 {
        self.reg[idx as usize]
    }

    pub fn store_gpr(&mut self, idx: u8, val: u32) {
        println!("WB: {}({}) <= {:08X}", reg_name(idx), idx, val);
        if idx != 0 {
            self.reg[idx as usize] = val;
        }
    }

    pub fn load_lo(&self) -> u32 {
        self.lo
    }
    pub fn store_lo(&mut self, val: u32) {
        println!("WB: LO <= {:08X}", val);
        self.lo = val;
    }

    pub fn load_hi(&self) -> u32 {
        self.hi
    }
    pub fn store_hi(&mut self, val: u32) {
        println!("WB: HI <= {:08X}", val);
        self.hi = val;
    }
}
