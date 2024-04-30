pub const OP_SPECIAL: u8 = 0x00;
pub const OP_BCOND: u8 = 0x01;
pub const OP_J: u8 = 0x02;
pub const OP_JAL: u8 = 0x03;
pub const OP_BEQ: u8 = 0x04;
pub const OP_BNE: u8 = 0x05;
pub const OP_BLEZ: u8 = 0x06;
pub const OP_BGTZ: u8 = 0x07;
pub const OP_ADDI: u8 = 0x08;
pub const OP_ADDIU: u8 = 0x09;
pub const OP_SLTI: u8 = 0x0A;
pub const OP_SLTIU: u8 = 0x0B;
pub const OP_ANDI: u8 = 0x0C;
pub const OP_ORI: u8 = 0x0D;
pub const OP_XORI: u8 = 0x0E;
pub const OP_LUI: u8 = 0x0F;
pub const OP_COP0: u8 = 0x10;
pub const OP_COP1: u8 = 0x11;
pub const OP_COP2: u8 = 0x12;
pub const OP_COP3: u8 = 0x13;
pub const OP_LB: u8 = 0x20;
pub const OP_LH: u8 = 0x21;
pub const OP_LWL: u8 = 0x22;
pub const OP_LW: u8 = 0x23;
pub const OP_LBU: u8 = 0x24;
pub const OP_LHU: u8 = 0x25;
pub const OP_LWR: u8 = 0x26;
pub const OP_SB: u8 = 0x28;
pub const OP_SH: u8 = 0x29;
pub const OP_SWL: u8 = 0x2A;
pub const OP_SW: u8 = 0x2B;
pub const OP_SWR: u8 = 0x2E;
pub const OP_LWC0: u8 = 0x30;
pub const OP_LWC1: u8 = 0x31;
pub const OP_LWC2: u8 = 0x32;
pub const OP_LWC3: u8 = 0x33;
pub const OP_SWC0: u8 = 0x38;
pub const OP_SWC1: u8 = 0x39;
pub const OP_SWC2: u8 = 0x3A;
pub const OP_SWC3: u8 = 0x3B;

pub const FUNCT_SLL: u8 = 0x00;
pub const FUNCT_SRL: u8 = 0x02;
pub const FUNCT_SRA: u8 = 0x03;
pub const FUNCT_SLLV: u8 = 0x04;
pub const FUNCT_SRLV: u8 = 0x06;
pub const FUNCT_SRAV: u8 = 0x07;
pub const FUNCT_JR: u8 = 0x08;
pub const FUNCT_JALR: u8 = 0x09;
pub const FUNCT_SYSCALL: u8 = 0x0C;
pub const FUNCT_BREAK: u8 = 0x0D;
pub const FUNCT_MFHI: u8 = 0x10;
pub const FUNCT_MTHI: u8 = 0x11;
pub const FUNCT_MFLO: u8 = 0x12;
pub const FUNCT_MTLO: u8 = 0x13;
pub const FUNCT_MULT: u8 = 0x18;
pub const FUNCT_MULTU: u8 = 0x19;
pub const FUNCT_DIV: u8 = 0x1A;
pub const FUNCT_DIVU: u8 = 0x1B;
pub const FUNCT_ADD: u8 = 0x20;
pub const FUNCT_ADDU: u8 = 0x21;
pub const FUNCT_SUB: u8 = 0x22;
pub const FUNCT_SUBU: u8 = 0x23;
pub const FUNCT_AND: u8 = 0x24;
pub const FUNCT_OR: u8 = 0x25;
pub const FUNCT_XOR: u8 = 0x26;
pub const FUNCT_NOR: u8 = 0x27;
pub const FUNCT_SLT: u8 = 0x2A;
pub const FUNCT_SLTU: u8 = 0x2B;

pub const BCOND_BLTZ: u8 = 0x00;
pub const BCOND_BGEZ: u8 = 0x01;
pub const BCOND_BLTZAL: u8 = 0x10;
pub const BCOND_BGEZAL: u8 = 0x11;

pub const COP_MF: u8 = 0x00;
pub const COP_CF: u8 = 0x02;
pub const COP_MT: u8 = 0x04;
pub const COP_CT: u8 = 0x06;
pub const COP_BC: u8 = 0x08;
pub const COP_CMD: u8 = 0x10;

pub const GPR_ZERO: u8 = 0x00;
pub const GPR_AT: u8 = 0x01;
pub const GPR_V0: u8 = 0x02;
pub const GPR_V1: u8 = 0x03;
pub const GPR_A0: u8 = 0x04;
pub const GPR_A1: u8 = 0x05;
pub const GPR_A2: u8 = 0x06;
pub const GPR_A3: u8 = 0x07;
pub const GPR_T0: u8 = 0x08;
pub const GPR_T1: u8 = 0x09;
pub const GPR_T2: u8 = 0x0A;
pub const GPR_T3: u8 = 0x0B;
pub const GPR_T4: u8 = 0x0C;
pub const GPR_T5: u8 = 0x0D;
pub const GPR_T6: u8 = 0x0E;
pub const GPR_T7: u8 = 0x0F;
pub const GPR_S0: u8 = 0x10;
pub const GPR_S1: u8 = 0x11;
pub const GPR_S2: u8 = 0x12;
pub const GPR_S3: u8 = 0x13;
pub const GPR_S4: u8 = 0x14;
pub const GPR_S5: u8 = 0x15;
pub const GPR_S6: u8 = 0x16;
pub const GPR_S7: u8 = 0x17;
pub const GPR_T8: u8 = 0x18;
pub const GPR_T9: u8 = 0x19;
pub const GPR_K0: u8 = 0x1A;
pub const GPR_K1: u8 = 0x1B;
pub const GPR_GP: u8 = 0x1C;
pub const GPR_SP: u8 = 0x1D;
pub const GPR_FP: u8 = 0x1E;
pub const GPR_RA: u8 = 0x1F;

pub const COP0_CONFIG: u8 = 0x03;
pub const COP0_UNK5: u8 = 0x05;
pub const COP0_UNK6: u8 = 0x06;
pub const COP0_CACHE: u8 = 0x07;
pub const COP0_UNK9: u8 = 0x09;
pub const COP0_UNK11: u8 = 0x0B;
pub const COP0_STATUS: u8 = 0x0C;
pub const COP0_CAUSE: u8 = 0x0D;
pub const COP0_EPC: u8 = 0x0E;
pub const COP0_FUNCT_RFE: u8 = 0x10;

pub const EXCEPTION_INT: u8 = 0;
pub const EXCEPTION_MOD: u8 = 1;
pub const EXCEPTION_TLBL: u8 = 2;
pub const EXCEPTION_TLBS: u8 = 3;
pub const EXCEPTION_ADEL: u8 = 4;
pub const EXCEPTION_ADES: u8 = 5;
pub const EXCEPTION_IBE: u8 = 6;
pub const EXCEPTION_DBE: u8 = 7;
pub const EXCEPTION_SYS: u8 = 8;
pub const EXCEPTION_BP: u8 = 9;
pub const EXCEPTION_RI: u8 = 10;
pub const EXCEPTION_CPU: u8 = 11;
pub const EXCEPTION_OV: u8 = 12;

pub struct CpuInst {
    pub opcode: u8,
    pub rs: u8,
    pub rt: u8,
    pub rd: u8,
    pub shamt: u8,
    pub funct: u8,
    pub imm: u16,
    pub target: u32,
}

impl CpuInst {
    #[inline(always)]
    pub fn imm_se(&self) -> u32 {
        self.imm as i16 as u32
    }

    #[inline(always)]
    pub fn imm_se_i(&self) -> i32 {
        self.imm as i16 as i32
    }

    #[inline(always)]
    pub fn imm_ze(&self) -> u32 {
        self.imm as u32
    }

    fn is_r(&self) -> bool {
        self.opcode == OP_SPECIAL
    }

    fn is_i(&self) -> bool {
        self.opcode != OP_SPECIAL && self.opcode != OP_J && self.opcode != OP_JAL
    }

    fn is_j(&self) -> bool {
        self.opcode == OP_J || self.opcode == OP_JAL
    }

    fn opname(&self) -> &str {
        match self.opcode {
            OP_SPECIAL => match self.funct {
                FUNCT_ADD => "ADD",
                FUNCT_ADDU => "ADDU",
                FUNCT_AND => "AND",
                FUNCT_JR => "JR",
                FUNCT_JALR => "JALR",
                FUNCT_OR => "OR",
                FUNCT_SLL => "SLL",
                FUNCT_SLLV => "SLLV",
                FUNCT_SRA => "SRA",
                FUNCT_SRAV => "SRAV",
                FUNCT_SRL => "SRL",
                FUNCT_SRLV => "SRLV",
                FUNCT_SUB => "SUB",
                FUNCT_SUBU => "SUBU",
                FUNCT_SYSCALL => "SYSCALL",
                FUNCT_BREAK => "BREAK",
                FUNCT_MFHI => "MFHI",
                FUNCT_MTHI => "MTHI",
                FUNCT_MFLO => "MFLO",
                FUNCT_MTLO => "MTLO",
                FUNCT_MULT => "MULT",
                FUNCT_MULTU => "MULTU",
                FUNCT_DIV => "DIV",
                FUNCT_DIVU => "DIVU",
                FUNCT_XOR => "XOR",
                FUNCT_NOR => "NOR",
                FUNCT_SLT => "SLT",
                FUNCT_SLTU => "SLTU",
                _ => "",
            },
            OP_BCOND => match self.rt {
                BCOND_BLTZ => "BLTZ",
                BCOND_BGEZ => "BGEZ",
                BCOND_BLTZAL => "BLTZAL",
                BCOND_BGEZAL => "BGEZAL",
                _ => "",
            },
            OP_J => "J",
            OP_JAL => "JAL",
            OP_BEQ => "BEQ",
            OP_BNE => "BNE",
            OP_BLEZ => "BLEZ",
            OP_BGTZ => "BGTZ",
            OP_ADDI => "ADDI",
            OP_ADDIU => "ADDIU",
            OP_SLTI => "SLTI",
            OP_SLTIU => "SLTIU",
            OP_ANDI => "ANDI",
            OP_ORI => "ORI",
            OP_XORI => "XORI",
            OP_LUI => "LUI",
            OP_COP0 => match self.rs {
                COP_MF => "MFC0",
                COP_CF => "CFC0",
                COP_MT => "MTC0",
                COP_CT => "CTC0",
                COP_BC => match self.rt {
                    0x00 => "BC0F",
                    0x01 => "BC0T",
                    _ => "<COP0 bad rt>",
                },
                COP_CMD => match self.funct {
                    COP0_FUNCT_RFE => "RFE",
                    _ => "COP0",
                },
                _ => "<COP0 bad rs>",
            },
            OP_COP1 => match self.rs {
                COP_MF => "MFC1",
                COP_CF => "CFC1",
                COP_MT => "MTC1",
                COP_CT => "CTC1",
                COP_BC => match self.rt {
                    0x00 => "BC1F",
                    0x01 => "BC1T",
                    _ => "<COP1 bad rt>",
                },
                COP_CMD => "COP1",
                _ => "<COP1 bad rs>",
            },
            OP_COP2 => match self.rs {
                COP_MF => "MFC2",
                COP_CF => "CFC2",
                COP_MT => "MTC2",
                COP_CT => "CTC2",
                COP_BC => match self.rt {
                    0x00 => "BC2F",
                    0x01 => "BC2T",
                    _ => "<COP2 bad rt>",
                },
                COP_CMD => "COP2",
                _ => "<COP2 bad rs>",
            },
            OP_COP3 => match self.rs {
                COP_MF => "MFC3",
                COP_CF => "CFC3",
                COP_MT => "MTC3",
                COP_CT => "CTC3",
                COP_BC => match self.rt {
                    0x00 => "BC3F",
                    0x01 => "BC3T",
                    _ => "<COP3 bad rt>",
                },
                COP_CMD => "COP3",
                _ => "<COP3 bad rs>",
            },
            OP_LB => "LB",
            OP_LH => "LH",
            OP_LWL => "LWL",
            OP_LW => "LW",
            OP_LBU => "LBU",
            OP_LHU => "LHU",
            OP_LWR => "LWR",
            OP_SB => "SB",
            OP_SH => "SH",
            OP_SWL => "SWL",
            OP_SW => "SW",
            OP_SWR => "SWR",
            OP_LWC0 => "LWC0",
            OP_LWC1 => "LWC1",
            OP_LWC2 => "LWC2",
            OP_LWC3 => "LWC3",
            OP_SWC0 => "SWC0",
            OP_SWC1 => "SWC1",
            OP_SWC2 => "SWC2",
            OP_SWC3 => "SWC3",
            _ => "",
        }
    }

    pub fn to_string(&self) -> String {
        if self.is_r() {
            format!(
                "{} ${}, ${}, ${}",
                self.opname(),
                reg_name(self.rd),
                reg_name(self.rs),
                reg_name(self.rt)
            )
        } else if self.is_i() {
            if self.opcode >> 4 == 0b10 {
                format!(
                    "{} ${}, [${} + 0x{:04X}]",
                    self.opname(),
                    reg_name(self.rt),
                    reg_name(self.rs),
                    self.imm
                )
            } else if self.opcode & 0b010000 != 0 {
                let copn = self.opcode & 0b11;
                if self.rs & 0b10000 == 0 {
                    if self.rs & 0b01000 == 0 {
                        format!(
                            "{} ${}, 0x{:02X}",
                            self.opname(),
                            reg_name(self.rt),
                            self.rd,
                        )
                    } else {
                        format!("{} ${:04X}", self.opname(), self.imm,)
                    }
                } else if copn == 0 {
                    if self.funct == COP0_FUNCT_RFE {
                        format!("{}", self.opname())
                    } else {
                        format!("{} 0x{:02X}", self.opname(), self.funct)
                    }
                } else {
                    format!("{} 0x{:08X}", self.opname(), self.target & 0x1FFFFFF)
                }
            } else {
                format!(
                    "{} ${}, ${}, 0x{:04X}",
                    self.opname(),
                    reg_name(self.rt),
                    reg_name(self.rs),
                    self.imm
                )
            }
        } else {
            format!("{} 0x{:08X}", self.opname(), self.target)
        }
    }

    pub fn new(inst: u32) -> CpuInst {
        CpuInst {
            opcode: ((inst >> 26) & 0x3F) as u8,
            rs: ((inst >> 21) & 0x1F) as u8,
            rt: ((inst >> 16) & 0x1F) as u8,
            rd: ((inst >> 11) & 0x1F) as u8,
            shamt: ((inst >> 6) & 0x1F) as u8,
            funct: (inst & 0x3F) as u8,
            imm: (inst & 0xFFFF) as u16,
            target: (inst & 0x3FFFFFF) << 2,
        }
    }
}

pub fn reg_name(reg: u8) -> &'static str {
    match reg {
        GPR_ZERO => "zero",
        GPR_AT => "at",
        GPR_V0 => "v0",
        GPR_V1 => "v1",
        GPR_A0 => "a0",
        GPR_A1 => "a1",
        GPR_A2 => "a2",
        GPR_A3 => "a3",
        GPR_T0 => "t0",
        GPR_T1 => "t1",
        GPR_T2 => "t2",
        GPR_T3 => "t3",
        GPR_T4 => "t4",
        GPR_T5 => "t5",
        GPR_T6 => "t6",
        GPR_T7 => "t7",
        GPR_S0 => "s0",
        GPR_S1 => "s1",
        GPR_S2 => "s2",
        GPR_S3 => "s3",
        GPR_S4 => "s4",
        GPR_S5 => "s5",
        GPR_S6 => "s6",
        GPR_S7 => "s7",
        GPR_T8 => "t8",
        GPR_T9 => "t9",
        GPR_K0 => "k0",
        GPR_K1 => "k1",
        GPR_GP => "gp",
        GPR_SP => "sp",
        GPR_FP => "fp",
        GPR_RA => "ra",
        _ => "",
    }
}
