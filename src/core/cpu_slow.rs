use std::collections::HashMap;
use std::rc::Rc;

use crate::core::bus::Bus;
use crate::core::{Cop0ExceptionParams, MemOpSize};

use super::cpu_regfile::CpuRegfile;
use super::{cpu_inst::*, Machine, MachineMutation};

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
struct CpuFetch {
    pc: u32,
    branch_target: Option<u32>,
}

impl CpuFetch {
    fn new() -> CpuFetch {
        CpuFetch {
            pc: 0,
            branch_target: None,
        }
    }

    fn reset(&mut self) {
        self.pc = 0xBFC00000;
    }

    fn branch_to(&self, mu: &mut MachineMutation, target: u32) {
        mu.branch_target = Some(target);
    }

    fn mutate(&mut self, mu: &MachineMutation) {
        if let Some(target) = mu.exception_branch {
            self.pc = target;
            self.branch_target = None;
        } else {
            self.pc = match self.branch_target {
                Some(target) => target,
                None => self.pc + 4,
            };

            // register write
            self.branch_target = mu.branch_target;
        }
    }
}

pub struct CpuInstEntry {
    decoded: CpuInst,
    inststr: String,
}

#[derive(Serialize, Deserialize)]
pub struct CpuSlow {
    pub reg: CpuRegfile,

    fetch: CpuFetch,

    #[serde(skip)]
    icache: HashMap<u32, Rc<CpuInstEntry>>,
}

impl CpuSlow {
    pub fn new() -> CpuSlow {
        CpuSlow {
            reg: CpuRegfile::new(),
            fetch: CpuFetch::new(),
            icache: HashMap::new(),
        }
    }

    pub fn clone(&self) -> CpuSlow {
        CpuSlow {
            reg: self.reg.clone(),
            fetch: self.fetch.clone(),
            icache: HashMap::new(),
        }
    }

    pub fn reset(&mut self) {
        self.fetch.reset()
    }

    pub fn current_pc(&self) -> u32 {
        self.fetch.pc
    }

    fn epc(&self) -> (u32, bool) {
        if self.fetch.branch_target.is_some() {
            (self.fetch.pc - 4, true)
        } else {
            (self.fetch.pc, false)
        }
    }

    pub fn cycle(&self, m: &Machine, mu: &mut MachineMutation) -> Result<(), String> {
        let cpu_entry: Rc<CpuInstEntry>;
        let (decoded, inststr) = match self.icache.get(&self.fetch.pc) {
            Some(entry) => (&entry.decoded, &entry.inststr),
            None => {
                let inst = Bus::lw(m, mu, self.fetch.pc)?;
                let decoded_val = CpuInst::new(inst);
                let inststr_val = format!(
                    "{:08X}: {:08X} {}",
                    self.fetch.pc,
                    inst,
                    decoded_val.to_string(),
                );
                cpu_entry = Rc::new(CpuInstEntry {
                    decoded: decoded_val,
                    inststr: inststr_val,
                });
                mu.icache_write = Some((m.cpu.fetch.pc, cpu_entry.clone()));
                (&cpu_entry.decoded, &cpu_entry.inststr)
            }
        };

        let pc = self.fetch.pc;
        println!("{}", inststr);

        match decoded.opcode {
            OP_SPECIAL => match decoded.funct {
                FUNCT_SLL => {
                    let rt = self.reg.load_gpr(decoded.rt);
                    mu.reg_write = Some((decoded.rd, rt << decoded.shamt));
                }
                FUNCT_SRL => {
                    let rt = self.reg.load_gpr(decoded.rt);
                    mu.reg_write = Some((decoded.rd, rt >> decoded.shamt));
                }
                FUNCT_SRA => {
                    let rt = self.reg.load_gpr(decoded.rt);
                    mu.reg_write = Some((decoded.rd, (rt as i32 >> decoded.shamt) as u32));
                }
                FUNCT_SLLV => {
                    let rt = self.reg.load_gpr(decoded.rt);
                    let rs = self.reg.load_gpr(decoded.rs) & 0x1F;
                    mu.reg_write = Some((decoded.rd, rt << rs));
                }
                FUNCT_SRLV => {
                    let rt = self.reg.load_gpr(decoded.rt);
                    let rs = self.reg.load_gpr(decoded.rs) & 0x1F;
                    mu.reg_write = Some((decoded.rd, rt >> rs));
                }
                FUNCT_SRAV => {
                    let rt = self.reg.load_gpr(decoded.rt);
                    let rs = self.reg.load_gpr(decoded.rs) & 0x1F;
                    mu.reg_write = Some((decoded.rd, (rt as i32 >> rs) as u32));
                }
                FUNCT_JR => {
                    let rs = self.reg.load_gpr(decoded.rs);
                    self.fetch.branch_to(mu, rs);
                }
                FUNCT_JALR => {
                    let rs = self.reg.load_gpr(decoded.rs);
                    mu.reg_write = Some((decoded.rd, pc + 8));
                    self.fetch.branch_to(mu, rs);
                }
                FUNCT_SYSCALL => {
                    mu.cop0_exception = Some(Cop0ExceptionParams {
                        cause: EXCEPTION_SYS,
                        epc: self.epc(),
                    });
                }
                FUNCT_MFHI => {
                    mu.reg_write = Some((decoded.rd, self.reg.load_hi()));
                }
                FUNCT_MTHI => {
                    let rs = self.reg.load_gpr(decoded.rs);
                    mu.hilo_write = Some((rs, self.reg.load_lo()));
                }
                FUNCT_MFLO => {
                    mu.reg_write = Some((decoded.rd, self.reg.load_lo()));
                }
                FUNCT_MTLO => {
                    let rs = self.reg.load_gpr(decoded.rs);
                    mu.hilo_write = Some((self.reg.load_hi(), rs));
                }
                FUNCT_DIV => {
                    let rs = self.reg.load_gpr(decoded.rs) as i32;
                    let rt = self.reg.load_gpr(decoded.rt) as i32;
                    if rt == 0 {
                        println!("WARN: DIV by zero");
                        mu.hilo_write = Some((0, 0)); // Undefined behavior
                    } else {
                        mu.hilo_write = Some(((rs % rt) as u32, (rs / rt) as u32));
                    }
                }
                FUNCT_DIVU => {
                    let rs = self.reg.load_gpr(decoded.rs);
                    let rt = self.reg.load_gpr(decoded.rt);
                    if rt == 0 {
                        println!("WARN: DIV by zero");
                        mu.hilo_write = Some((0, 0)); // Undefined behavior
                    } else {
                        mu.hilo_write = Some((rs % rt, rs / rt));
                    }
                }
                FUNCT_ADD => {
                    let rs = self.reg.load_gpr(decoded.rs);
                    let rt = self.reg.load_gpr(decoded.rt);
                    mu.reg_write = Some((decoded.rd, (rs as i32 + rt as i32) as u32));
                }
                FUNCT_ADDU => {
                    let rs = self.reg.load_gpr(decoded.rs);
                    let rt = self.reg.load_gpr(decoded.rt);
                    mu.reg_write = Some((decoded.rd, rs.wrapping_add(rt)));
                }
                FUNCT_SUB => {
                    let rs = self.reg.load_gpr(decoded.rs);
                    let rt = self.reg.load_gpr(decoded.rt);
                    mu.reg_write = Some((decoded.rd, (rs as i32 - rt as i32) as u32));
                }
                FUNCT_SUBU => {
                    let rs = self.reg.load_gpr(decoded.rs);
                    let rt = self.reg.load_gpr(decoded.rt);
                    mu.reg_write = Some((decoded.rd, rs.wrapping_sub(rt)));
                }
                FUNCT_AND => {
                    let rs = self.reg.load_gpr(decoded.rs);
                    let rt = self.reg.load_gpr(decoded.rt);
                    mu.reg_write = Some((decoded.rd, rs & rt));
                }
                FUNCT_OR => {
                    let rs = self.reg.load_gpr(decoded.rs);
                    let rt = self.reg.load_gpr(decoded.rt);
                    mu.reg_write = Some((decoded.rd, rs | rt));
                }
                FUNCT_SLT => {
                    let rs = self.reg.load_gpr(decoded.rs) as i32;
                    let rt = self.reg.load_gpr(decoded.rt) as i32;
                    mu.reg_write = Some((decoded.rd, if rs < rt { 1 } else { 0 }));
                }
                FUNCT_SLTU => {
                    let rs = self.reg.load_gpr(decoded.rs);
                    let rt = self.reg.load_gpr(decoded.rt);
                    mu.reg_write = Some((decoded.rd, if rs < rt { 1 } else { 0 }));
                }
                _ => return Err(format!("[SPECIAL] Unknown funct {}", decoded.funct)),
            },
            OP_BCOND => match decoded.rt {
                BCOND_BLTZ => {
                    let rs = self.reg.load_gpr(decoded.rs);
                    if (rs as i32) < 0 {
                        let target = (pc + 4).wrapping_add(decoded.imm_se() << 2);
                        self.fetch.branch_to(mu, target);
                    }
                }
                BCOND_BGEZ => {
                    let rs = self.reg.load_gpr(decoded.rs);
                    if (rs as i32) >= 0 {
                        let target = (pc + 4).wrapping_add(decoded.imm_se() << 2);
                        self.fetch.branch_to(mu, target);
                    }
                }
                BCOND_BLTZAL => {
                    let rs = self.reg.load_gpr(decoded.rs);
                    if (rs as i32) < 0 {
                        let target = (pc + 4).wrapping_add(decoded.imm_se() << 2);
                        mu.reg_write = Some((GPR_RA, pc + 8));
                        self.fetch.branch_to(mu, target);
                    }
                }
                BCOND_BGEZAL => {
                    let rs = self.reg.load_gpr(decoded.rs);
                    if (rs as i32) >= 0 {
                        let target = (pc + 4).wrapping_add(decoded.imm_se() << 2);
                        mu.reg_write = Some((GPR_RA, pc + 8));
                        self.fetch.branch_to(mu, target);
                    }
                }
                _ => return Err(format!("Unknown BCOND rt {}", decoded.rt)),
            },
            OP_J => {
                let target = (pc & 0xF000_0000) | decoded.target;
                self.fetch.branch_to(mu, target);
            }
            OP_JAL => {
                let target = (pc & 0xF000_0000) | decoded.target;
                mu.reg_write = Some((GPR_RA, pc + 8));
                self.fetch.branch_to(mu, target);
            }
            OP_BEQ => {
                let rs = self.reg.load_gpr(decoded.rs);
                let rt = self.reg.load_gpr(decoded.rt);
                if rs == rt {
                    let target = (pc + 4).wrapping_add(decoded.imm_se() << 2);
                    self.fetch.branch_to(mu, target);
                }
            }
            OP_BNE => {
                let rs = self.reg.load_gpr(decoded.rs);
                let rt = self.reg.load_gpr(decoded.rt);
                if rs != rt {
                    let target = (pc + 4).wrapping_add(decoded.imm_se() << 2);
                    self.fetch.branch_to(mu, target);
                }
            }
            OP_BLEZ => {
                let rs = self.reg.load_gpr(decoded.rs);
                if (rs as i32) <= 0 {
                    let target = (pc + 4).wrapping_add(decoded.imm_se() << 2);
                    self.fetch.branch_to(mu, target);
                }
            }
            OP_BGTZ => {
                let rs = self.reg.load_gpr(decoded.rs);
                if (rs as i32) > 0 {
                    let target = (pc + 4).wrapping_add(decoded.imm_se() << 2);
                    self.fetch.branch_to(mu, target);
                }
            }
            OP_ADDI => {
                let rs = self.reg.load_gpr(decoded.rs);
                mu.reg_write = Some((decoded.rt, (rs as i32 + decoded.imm_se_i()) as u32));
            }
            OP_ADDIU => {
                let rs = self.reg.load_gpr(decoded.rs);
                mu.reg_write = Some((decoded.rt, rs.wrapping_add(decoded.imm_se())));
            }
            OP_SLTI => {
                let rs = self.reg.load_gpr(decoded.rs);
                mu.reg_write = Some((
                    decoded.rt,
                    if (rs as i32) < decoded.imm_se_i() {
                        1
                    } else {
                        0
                    },
                ));
            }
            OP_SLTIU => {
                let rs = self.reg.load_gpr(decoded.rs);
                mu.reg_write = Some((decoded.rt, if rs < decoded.imm_se() { 1 } else { 0 }));
            }
            OP_ANDI => {
                let rs = self.reg.load_gpr(decoded.rs);
                mu.reg_write = Some((decoded.rt, rs & decoded.imm_ze()));
            }
            OP_ORI => {
                let rs = self.reg.load_gpr(decoded.rs);
                mu.reg_write = Some((decoded.rt, rs | decoded.imm_ze()));
            }
            OP_LUI => {
                mu.reg_write = Some((decoded.rt, decoded.imm_ze() << 16));
            }
            OP_LB => {
                let addr = self.reg.load_gpr(decoded.rs).wrapping_add(decoded.imm_se());
                let val = Bus::lb(m, mu, addr)?;
                mu.reg_write = Some((decoded.rt, val as i8 as u32));
            }
            OP_LH => {
                let addr = self.reg.load_gpr(decoded.rs).wrapping_add(decoded.imm_se());
                let val = Bus::lh(m, mu, addr)?;
                mu.reg_write = Some((decoded.rt, val as i16 as u32));
            }
            OP_LW => {
                let addr = self.reg.load_gpr(decoded.rs).wrapping_add(decoded.imm_se());
                let val = Bus::lw(m, mu, addr)?;
                mu.reg_write = Some((decoded.rt, val));
            }
            OP_LBU => {
                let addr = self.reg.load_gpr(decoded.rs).wrapping_add(decoded.imm_se());
                let val = Bus::lb(m, mu, addr)?;
                mu.reg_write = Some((decoded.rt, val));
            }
            OP_LHU => {
                let addr = self.reg.load_gpr(decoded.rs).wrapping_add(decoded.imm_se());
                let val = Bus::lh(m, mu, addr)?;
                mu.reg_write = Some((decoded.rt, val));
            }
            OP_SB => {
                let addr = self.reg.load_gpr(decoded.rs).wrapping_add(decoded.imm_se());
                let val = self.reg.load_gpr(decoded.rt);
                mu.bus_write = Some((addr, val, MemOpSize::Byte));
            }
            OP_SH => {
                let addr = self.reg.load_gpr(decoded.rs).wrapping_add(decoded.imm_se());
                let val = self.reg.load_gpr(decoded.rt);
                mu.bus_write = Some((addr, val, MemOpSize::Half));
            }
            OP_SW => {
                let addr = self.reg.load_gpr(decoded.rs).wrapping_add(decoded.imm_se());
                let val = self.reg.load_gpr(decoded.rt);
                mu.bus_write = Some((addr, val, MemOpSize::Word));
            }
            x if x & 0b010000 != 0 => {
                let copn = x & 0b11;
                match copn {
                    0 => match decoded.rs {
                        COP_MT => {
                            let value = self.reg.load_gpr(decoded.rt);
                            mu.cop0_write = Some((decoded.rd, value));
                        }
                        COP_MF => {
                            let value = m.cop0.load(decoded.rd)?;
                            mu.reg_write = Some((decoded.rt, value));
                        }
                        COP_CMD => match decoded.funct {
                            COP0_FUNCT_RFE => mu.exception_return = true,
                            _ => return Err(format!("Unknown cop0 funct {}", decoded.funct)),
                        },
                        _ => return Err(format!("Unknown cop0 rs {}", decoded.rs)),
                    },
                    _ => return Err(format!("Unsupported cop {}", copn)),
                }
            }
            _ => return Err(format!("Unknown opcode {}", decoded.opcode)),
        }

        Ok(())
    }

    pub fn mutate(&mut self, mu: &MachineMutation) {
        if let Some((reg, val)) = mu.reg_write {
            self.reg.store_gpr(reg, val);
        }
        if let Some((hi, lo)) = mu.hilo_write {
            self.reg.store_hi(hi);
            self.reg.store_lo(lo);
        }
        if let Some(tuple) = &mu.icache_write {
            self.icache.insert(tuple.0, tuple.1.clone());
        }
        self.fetch.mutate(mu);
    }
}
