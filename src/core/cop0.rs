use super::cpu_inst::*;
use super::MachineMutation;

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Cop0 {
    reg_status: u32,
    reg_cause: u32,
    reg_cache: u32,
    reg_epc: u32,
    int_mask: u16,
    int_reqs: u16,
}

#[derive(Copy, Clone)]
pub struct Cop0ExceptionParams {
    // 0: Address, 1: BD
    pub epc: (u32, bool),
    pub cause: u8,
}

const STATUS_PUSH_MASK: u32 = 0x0000_000F;
const STATUS_POP_MASK: u32 = 0x0000_003C;
const CACHE_PUSH_MASK: u32 = 0x0000_0F00;
const CACHE_POP_MASK: u32 = 0x0000_3C00;
const CAUSE_BD_MASK: u32 = 0x8000_0000;
const CAUSE_EXC_MASK: u32 = 0x0000_007C;

impl Cop0 {
    pub fn new() -> Cop0 {
        Cop0 {
            reg_status: 0,
            reg_cause: 0,
            reg_cache: 0,
            reg_epc: 0,
            int_mask: 0,
            int_reqs: 0,
        }
    }

    pub fn status_isc(&self) -> bool {
        return self.reg_status & 0x10000 != 0;
    }

    pub fn load(&self, idx: u8) -> Result<u32, String> {
        match idx {
            COP0_CONFIG => {
                println!("COP0: CONFIG => {:08X}", 0);
                Ok(0)
            }
            COP0_UNK5 => {
                println!("COP0: UNK5 => {:08X}", 0);
                Ok(0)
            }
            COP0_UNK6 => {
                println!("COP0: UNK6 => {:08X}", 0);
                Ok(0)
            }
            COP0_CACHE => {
                println!("COP0: CACHE => {:08X}", self.reg_cache);
                Ok(self.reg_cache)
            }
            COP0_UNK9 => {
                println!("COP0: UNK9 => {:08X}", 0);
                Ok(0)
            }
            COP0_UNK11 => {
                println!("COP0: UNK11 => {:08X}", 0);
                Ok(0)
            }
            COP0_STATUS => {
                println!("COP0: STATUS => {:08X}", self.reg_status);
                Ok(self.reg_status)
            }
            COP0_CAUSE => {
                println!("COP0: CAUSE => {:08X}", self.reg_cause);
                Ok(self.reg_cause)
            }
            COP0_EPC => {
                println!("COP0: EPC => {:08X}", self.reg_epc);
                Ok(self.reg_epc)
            }
            _ => Err(format!("Unknown COP0 register: {:02X}", idx)),
        }
    }

    pub fn cycle(&self) {}

    pub fn mutate(&mut self, mu: &mut MachineMutation) -> Result<(), String> {
        match mu.cop0_write {
            Some((idx, val)) => match idx {
                COP0_CONFIG => {
                    println!("COP0: CONFIG <= {:08X}", val);
                }
                COP0_UNK5 => {
                    println!("COP0: UNK5 <= {:08X}", val);
                }
                COP0_UNK6 => {
                    println!("COP0: UNK6 <= {:08X}", val);
                }
                COP0_CACHE => {
                    println!("COP0: CACHE <= {:08X}", val);
                    self.reg_cache = val;
                }
                COP0_UNK9 => {
                    println!("COP0: UNK9 <= {:08X}", val);
                }
                COP0_UNK11 => {
                    println!("COP0: UNK11 <= {:08X}", val);
                }
                COP0_STATUS => {
                    println!("COP0: STATUS <= {:08X}", val);
                    self.reg_status = val;
                }
                COP0_CAUSE => {
                    println!("COP0: CAUSE <= {:08X}", val);
                    self.reg_cause = val;
                }
                COP0_EPC => {
                    println!("COP0: EPC <= {:08X}", val);
                    self.reg_epc = val;
                }
                _ => {
                    return Err(format!("Unknown COP0 register: {:02X}", idx));
                }
            },
            None => {}
        };
        if let Some(e) = mu.cop0_exception {
            self.raise_exception(&e, mu);
        }
        if mu.exception_return {
            self.return_from_exception();
        }
        Ok(())
    }

    pub fn int_stat(&self) -> u32 {
        self.int_reqs as u32
    }

    pub fn int_mask(&self) -> u32 {
        self.int_mask as u32
    }

    pub fn int_set_mask(&mut self, mask: u32) {
        self.int_mask = mask as u16;
    }

    pub fn int_set_stat(&mut self, stat: u32) {
        self.int_reqs = stat as u16;
    }

    fn raise_exception(&mut self, e: &Cop0ExceptionParams, mu: &mut MachineMutation) {
        self.push_mode();
        self.reg_cause =
            (self.reg_cause & !CAUSE_BD_MASK) | if e.epc.1 { CAUSE_BD_MASK } else { 0 };
        self.reg_cause = (self.reg_cause & !CAUSE_EXC_MASK) | ((e.cause as u32) << 2);
        self.reg_epc = e.epc.0;
        mu.exception_branch = match e.cause {
            EXCEPTION_TLBL | EXCEPTION_TLBS => Some(0x80000000),
            _ => Some(0x80000080),
        };
        println!("COP0: Exception {:02X} at {:08X}", e.cause, e.epc.0);
        println!("COP0: STATUS <= {:08X}", self.reg_status);
        println!("COP0: CAUSE <= {:08X}", self.reg_cause);
    }

    fn return_from_exception(&mut self) {
        self.pop_mode();
        println!("COP0: STATUS <= {:08X}", self.reg_status);
        println!("COP0: CACHE <= {:08X}", self.reg_cache);
    }

    fn push_mode(&mut self) {
        self.reg_status =
            (self.reg_status & !STATUS_PUSH_MASK) | ((self.reg_status << 2) & STATUS_POP_MASK);
        self.reg_cache =
            (self.reg_cache & !CACHE_PUSH_MASK) | ((self.reg_cache << 2) & CACHE_POP_MASK);
    }

    fn pop_mode(&mut self) {
        self.reg_status =
            (self.reg_status & !STATUS_PUSH_MASK) | ((self.reg_status >> 2) & STATUS_PUSH_MASK);
        self.reg_cache =
            (self.reg_cache & !CACHE_PUSH_MASK) | ((self.reg_cache >> 2) & CACHE_PUSH_MASK);
    }
}
