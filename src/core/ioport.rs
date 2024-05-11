use clap::builder::Str;
use serde::{Deserialize, Serialize};

use super::{timers::TimerPort, Machine, MachineMutation};

pub const IO_EXP1_BASE_ADDR: u32 = 0x0000;
pub const IO_EXP2_BASE_ADDR: u32 = 0x0004;
pub const IO_EXP1_DELAY_SIZE: u32 = 0x0008;
pub const IO_EXP3_DELAY_SIZE: u32 = 0x000C;
pub const IO_BIOS_ROM: u32 = 0x0010;
pub const IO_SPU_DELAY: u32 = 0x0014;
pub const IO_CDROM_DELAY: u32 = 0x0018;
pub const IO_EXP2_DELAY_SIZE: u32 = 0x001C;
pub const IO_COMMON_DELAY: u32 = 0x0020;
pub const IO_JOY_MCD_DATA: u32 = 0x0040;
pub const IO_JOY_MCD_STAT: u32 = 0x0044;
pub const IO_JOY_MCD_MODE: u32 = 0x0048;
pub const IO_JOY_MCD_CTRL: u32 = 0x004A;
pub const IO_JOY_MCD_BAUD: u32 = 0x004E;
pub const IO_SIO_DATA: u32 = 0x0050;
pub const IO_SIO_STAT: u32 = 0x0054;
pub const IO_SIO_MODE: u32 = 0x0058;
pub const IO_SIO_CTRL: u32 = 0x005A;
pub const IO_SIO_MISC: u32 = 0x005C;
pub const IO_SIO_BAUD: u32 = 0x005E;
pub const IO_RAM_SIZE: u32 = 0x0060;
pub const IO_I_STAT: u32 = 0x0070;
pub const IO_I_MASK: u32 = 0x0074;
pub const IO_DMA_MDEC_IN_MADR: u32 = 0x0080;
pub const IO_DMA_MDEC_IN_BCR: u32 = 0x0084;
pub const IO_DMA_MDEC_IN_CHCR: u32 = 0x0088;
pub const IO_DMA_MDEC_OUT_MADR: u32 = 0x0090;
pub const IO_DMA_MDEC_OUT_BCR: u32 = 0x0094;
pub const IO_DMA_MDEC_OUT_CHCR: u32 = 0x0098;
pub const IO_DMA_GPU_MADR: u32 = 0x00A0;
pub const IO_DMA_GPU_BCR: u32 = 0x00A4;
pub const IO_DMA_GPU_CHCR: u32 = 0x00A8;
pub const IO_DMA_CDROM_MADR: u32 = 0x00B0;
pub const IO_DMA_CDROM_BCR: u32 = 0x00B4;
pub const IO_DMA_CDROM_CHCR: u32 = 0x00B8;
pub const IO_DMA_SPU_MADR: u32 = 0x00C0;
pub const IO_DMA_SPU_BCR: u32 = 0x00C4;
pub const IO_DMA_SPU_CHCR: u32 = 0x00C8;
pub const IO_DMA_PIO_MADR: u32 = 0x00D0;
pub const IO_DMA_PIO_BCR: u32 = 0x00D4;
pub const IO_DMA_PIO_CHCR: u32 = 0x00D8;
pub const IO_DMA_OTC_MADR: u32 = 0x00E0;
pub const IO_DMA_OTC_BCR: u32 = 0x00E4;
pub const IO_DMA_OTC_CHCR: u32 = 0x00E8;
pub const IO_DMA_DPCR: u32 = 0x00F0;
pub const IO_DMA_DICR: u32 = 0x00F4;
pub const IO_TMR_DOTCLOCK_VAL: u32 = 0x0100;
pub const IO_TMR_DOTCLOCK_MODE: u32 = 0x0104;
pub const IO_TMR_DOTCLOCK_MAX: u32 = 0x0108;
pub const IO_TMR_HRETRACE_VAL: u32 = 0x0110;
pub const IO_TMR_HRETRACE_MODE: u32 = 0x0114;
pub const IO_TMR_HRETRACE_MAX: u32 = 0x0118;
pub const IO_TMR_SYSCLOCK_VAL: u32 = 0x0120;
pub const IO_TMR_SYSCLOCK_MODE: u32 = 0x0124;
pub const IO_TMR_SYSCLOCK_MAX: u32 = 0x0128;
pub const IO_CDROM_REG0: u32 = 0x0180;
pub const IO_CDROM_REG1: u32 = 0x0181;
pub const IO_CDROM_REG2: u32 = 0x0182;
pub const IO_CDROM_REG3: u32 = 0x0183;
pub const IO_GPU_REG0: u32 = 0x0180;
pub const IO_GPU_REG1: u32 = 0x0184;
pub const IO_MDEC_REG0: u32 = 0x0180;
pub const IO_MDEC_REG1: u32 = 0x0184;
pub const IO_VOICE_00_LEFT_RIGHT: u32 = 0x0C00;
pub const IO_VOICE_00_ADPCM_SAMPLE_RATE: u32 = 0x0C04;
pub const IO_VOICE_00_ADPCM_START_ADDR: u32 = 0x0C06;
pub const IO_VOICE_00_ADSR_ATT_DEC_SUS_REL: u32 = 0x0C08;
pub const IO_VOICE_00_ADSR_CURR_VOLUME: u32 = 0x0C0C;
pub const IO_VOICE_00_ADPCM_REPEAT_ADDR: u32 = 0x0C0E;
pub const IO_VOICE_01_LEFT_RIGHT: u32 = 0x0C10;
pub const IO_VOICE_01_ADPCM_SAMPLE_RATE: u32 = 0x0C14;
pub const IO_VOICE_01_ADPCM_START_ADDR: u32 = 0x0C16;
pub const IO_VOICE_01_ADSR_ATT_DEC_SUS_REL: u32 = 0x0C18;
pub const IO_VOICE_01_ADSR_CURR_VOLUME: u32 = 0x0C1C;
pub const IO_VOICE_01_ADPCM_REPEAT_ADDR: u32 = 0x0C1E;
pub const IO_VOICE_02_LEFT_RIGHT: u32 = 0x0C20;
pub const IO_VOICE_02_ADPCM_SAMPLE_RATE: u32 = 0x0C24;
pub const IO_VOICE_02_ADPCM_START_ADDR: u32 = 0x0C26;
pub const IO_VOICE_02_ADSR_ATT_DEC_SUS_REL: u32 = 0x0C28;
pub const IO_VOICE_02_ADSR_CURR_VOLUME: u32 = 0x0C2C;
pub const IO_VOICE_02_ADPCM_REPEAT_ADDR: u32 = 0x0C2E;
pub const IO_VOICE_03_LEFT_RIGHT: u32 = 0x0C30;
pub const IO_VOICE_03_ADPCM_SAMPLE_RATE: u32 = 0x0C34;
pub const IO_VOICE_03_ADPCM_START_ADDR: u32 = 0x0C36;
pub const IO_VOICE_03_ADSR_ATT_DEC_SUS_REL: u32 = 0x0C38;
pub const IO_VOICE_03_ADSR_CURR_VOLUME: u32 = 0x0C3C;
pub const IO_VOICE_03_ADPCM_REPEAT_ADDR: u32 = 0x0C3E;
pub const IO_VOICE_04_LEFT_RIGHT: u32 = 0x0C40;
pub const IO_VOICE_04_ADPCM_SAMPLE_RATE: u32 = 0x0C44;
pub const IO_VOICE_04_ADPCM_START_ADDR: u32 = 0x0C46;
pub const IO_VOICE_04_ADSR_ATT_DEC_SUS_REL: u32 = 0x0C48;
pub const IO_VOICE_04_ADSR_CURR_VOLUME: u32 = 0x0C4C;
pub const IO_VOICE_04_ADPCM_REPEAT_ADDR: u32 = 0x0C4E;
pub const IO_VOICE_05_LEFT_RIGHT: u32 = 0x0C50;
pub const IO_VOICE_05_ADPCM_SAMPLE_RATE: u32 = 0x0C54;
pub const IO_VOICE_05_ADPCM_START_ADDR: u32 = 0x0C56;
pub const IO_VOICE_05_ADSR_ATT_DEC_SUS_REL: u32 = 0x0C58;
pub const IO_VOICE_05_ADSR_CURR_VOLUME: u32 = 0x0C5C;
pub const IO_VOICE_05_ADPCM_REPEAT_ADDR: u32 = 0x0C5E;
pub const IO_VOICE_06_LEFT_RIGHT: u32 = 0x0C60;
pub const IO_VOICE_06_ADPCM_SAMPLE_RATE: u32 = 0x0C64;
pub const IO_VOICE_06_ADPCM_START_ADDR: u32 = 0x0C66;
pub const IO_VOICE_06_ADSR_ATT_DEC_SUS_REL: u32 = 0x0C68;
pub const IO_VOICE_06_ADSR_CURR_VOLUME: u32 = 0x0C6C;
pub const IO_VOICE_06_ADPCM_REPEAT_ADDR: u32 = 0x0C6E;
pub const IO_VOICE_07_LEFT_RIGHT: u32 = 0x0C70;
pub const IO_VOICE_07_ADPCM_SAMPLE_RATE: u32 = 0x0C74;
pub const IO_VOICE_07_ADPCM_START_ADDR: u32 = 0x0C76;
pub const IO_VOICE_07_ADSR_ATT_DEC_SUS_REL: u32 = 0x0C78;
pub const IO_VOICE_07_ADSR_CURR_VOLUME: u32 = 0x0C7C;
pub const IO_VOICE_07_ADPCM_REPEAT_ADDR: u32 = 0x0C7E;
pub const IO_VOICE_08_LEFT_RIGHT: u32 = 0x0C80;
pub const IO_VOICE_08_ADPCM_SAMPLE_RATE: u32 = 0x0C84;
pub const IO_VOICE_08_ADPCM_START_ADDR: u32 = 0x0C86;
pub const IO_VOICE_08_ADSR_ATT_DEC_SUS_REL: u32 = 0x0C88;
pub const IO_VOICE_08_ADSR_CURR_VOLUME: u32 = 0x0C8C;
pub const IO_VOICE_08_ADPCM_REPEAT_ADDR: u32 = 0x0C8E;
pub const IO_VOICE_09_LEFT_RIGHT: u32 = 0x0C90;
pub const IO_VOICE_09_ADPCM_SAMPLE_RATE: u32 = 0x0C94;
pub const IO_VOICE_09_ADPCM_START_ADDR: u32 = 0x0C96;
pub const IO_VOICE_09_ADSR_ATT_DEC_SUS_REL: u32 = 0x0C98;
pub const IO_VOICE_09_ADSR_CURR_VOLUME: u32 = 0x0C9C;
pub const IO_VOICE_09_ADPCM_REPEAT_ADDR: u32 = 0x0C9E;
pub const IO_VOICE_0A_LEFT_RIGHT: u32 = 0x0CA0;
pub const IO_VOICE_0A_ADPCM_SAMPLE_RATE: u32 = 0x0CA4;
pub const IO_VOICE_0A_ADPCM_START_ADDR: u32 = 0x0CA6;
pub const IO_VOICE_0A_ADSR_ATT_DEC_SUS_REL: u32 = 0x0CA8;
pub const IO_VOICE_0A_ADSR_CURR_VOLUME: u32 = 0x0CAC;
pub const IO_VOICE_0A_ADPCM_REPEAT_ADDR: u32 = 0x0CAE;
pub const IO_VOICE_0B_LEFT_RIGHT: u32 = 0x0CB0;
pub const IO_VOICE_0B_ADPCM_SAMPLE_RATE: u32 = 0x0CB4;
pub const IO_VOICE_0B_ADPCM_START_ADDR: u32 = 0x0CB6;
pub const IO_VOICE_0B_ADSR_ATT_DEC_SUS_REL: u32 = 0x0CB8;
pub const IO_VOICE_0B_ADSR_CURR_VOLUME: u32 = 0x0CBC;
pub const IO_VOICE_0B_ADPCM_REPEAT_ADDR: u32 = 0x0CBE;
pub const IO_VOICE_0C_LEFT_RIGHT: u32 = 0x0CC0;
pub const IO_VOICE_0C_ADPCM_SAMPLE_RATE: u32 = 0x0CC4;
pub const IO_VOICE_0C_ADPCM_START_ADDR: u32 = 0x0CC6;
pub const IO_VOICE_0C_ADSR_ATT_DEC_SUS_REL: u32 = 0x0CC8;
pub const IO_VOICE_0C_ADSR_CURR_VOLUME: u32 = 0x0CCC;
pub const IO_VOICE_0C_ADPCM_REPEAT_ADDR: u32 = 0x0CCE;
pub const IO_VOICE_0D_LEFT_RIGHT: u32 = 0x0CD0;
pub const IO_VOICE_0D_ADPCM_SAMPLE_RATE: u32 = 0x0CD4;
pub const IO_VOICE_0D_ADPCM_START_ADDR: u32 = 0x0CD6;
pub const IO_VOICE_0D_ADSR_ATT_DEC_SUS_REL: u32 = 0x0CD8;
pub const IO_VOICE_0D_ADSR_CURR_VOLUME: u32 = 0x0CDC;
pub const IO_VOICE_0D_ADPCM_REPEAT_ADDR: u32 = 0x0CDE;
pub const IO_VOICE_0E_LEFT_RIGHT: u32 = 0x0CE0;
pub const IO_VOICE_0E_ADPCM_SAMPLE_RATE: u32 = 0x0CE4;
pub const IO_VOICE_0E_ADPCM_START_ADDR: u32 = 0x0CE6;
pub const IO_VOICE_0E_ADSR_ATT_DEC_SUS_REL: u32 = 0x0CE8;
pub const IO_VOICE_0E_ADSR_CURR_VOLUME: u32 = 0x0CEC;
pub const IO_VOICE_0E_ADPCM_REPEAT_ADDR: u32 = 0x0CEE;
pub const IO_VOICE_0F_LEFT_RIGHT: u32 = 0x0CF0;
pub const IO_VOICE_0F_ADPCM_SAMPLE_RATE: u32 = 0x0CF4;
pub const IO_VOICE_0F_ADPCM_START_ADDR: u32 = 0x0CF6;
pub const IO_VOICE_0F_ADSR_ATT_DEC_SUS_REL: u32 = 0x0CF8;
pub const IO_VOICE_0F_ADSR_CURR_VOLUME: u32 = 0x0CFC;
pub const IO_VOICE_0F_ADPCM_REPEAT_ADDR: u32 = 0x0CFE;
pub const IO_VOICE_10_LEFT_RIGHT: u32 = 0x0D00;
pub const IO_VOICE_10_ADPCM_SAMPLE_RATE: u32 = 0x0D04;
pub const IO_VOICE_10_ADPCM_START_ADDR: u32 = 0x0D06;
pub const IO_VOICE_10_ADSR_ATT_DEC_SUS_REL: u32 = 0x0D08;
pub const IO_VOICE_10_ADSR_CURR_VOLUME: u32 = 0x0D0C;
pub const IO_VOICE_10_ADPCM_REPEAT_ADDR: u32 = 0x0D0E;
pub const IO_VOICE_11_LEFT_RIGHT: u32 = 0x0D10;
pub const IO_VOICE_11_ADPCM_SAMPLE_RATE: u32 = 0x0D14;
pub const IO_VOICE_11_ADPCM_START_ADDR: u32 = 0x0D16;
pub const IO_VOICE_11_ADSR_ATT_DEC_SUS_REL: u32 = 0x0D18;
pub const IO_VOICE_11_ADSR_CURR_VOLUME: u32 = 0x0D1C;
pub const IO_VOICE_11_ADPCM_REPEAT_ADDR: u32 = 0x0D1E;
pub const IO_VOICE_12_LEFT_RIGHT: u32 = 0x0D20;
pub const IO_VOICE_12_ADPCM_SAMPLE_RATE: u32 = 0x0D24;
pub const IO_VOICE_12_ADPCM_START_ADDR: u32 = 0x0D26;
pub const IO_VOICE_12_ADSR_ATT_DEC_SUS_REL: u32 = 0x0D28;
pub const IO_VOICE_12_ADSR_CURR_VOLUME: u32 = 0x0D2C;
pub const IO_VOICE_12_ADPCM_REPEAT_ADDR: u32 = 0x0D2E;
pub const IO_VOICE_13_LEFT_RIGHT: u32 = 0x0D30;
pub const IO_VOICE_13_ADPCM_SAMPLE_RATE: u32 = 0x0D34;
pub const IO_VOICE_13_ADPCM_START_ADDR: u32 = 0x0D36;
pub const IO_VOICE_13_ADSR_ATT_DEC_SUS_REL: u32 = 0x0D38;
pub const IO_VOICE_13_ADSR_CURR_VOLUME: u32 = 0x0D3C;
pub const IO_VOICE_13_ADPCM_REPEAT_ADDR: u32 = 0x0D3E;
pub const IO_VOICE_14_LEFT_RIGHT: u32 = 0x0D40;
pub const IO_VOICE_14_ADPCM_SAMPLE_RATE: u32 = 0x0D44;
pub const IO_VOICE_14_ADPCM_START_ADDR: u32 = 0x0D46;
pub const IO_VOICE_14_ADSR_ATT_DEC_SUS_REL: u32 = 0x0D48;
pub const IO_VOICE_14_ADSR_CURR_VOLUME: u32 = 0x0D4C;
pub const IO_VOICE_14_ADPCM_REPEAT_ADDR: u32 = 0x0D4E;
pub const IO_VOICE_15_LEFT_RIGHT: u32 = 0x0D50;
pub const IO_VOICE_15_ADPCM_SAMPLE_RATE: u32 = 0x0D54;
pub const IO_VOICE_15_ADPCM_START_ADDR: u32 = 0x0D56;
pub const IO_VOICE_15_ADSR_ATT_DEC_SUS_REL: u32 = 0x0D58;
pub const IO_VOICE_15_ADSR_CURR_VOLUME: u32 = 0x0D5C;
pub const IO_VOICE_15_ADPCM_REPEAT_ADDR: u32 = 0x0D5E;
pub const IO_VOICE_16_LEFT_RIGHT: u32 = 0x0D60;
pub const IO_VOICE_16_ADPCM_SAMPLE_RATE: u32 = 0x0D64;
pub const IO_VOICE_16_ADPCM_START_ADDR: u32 = 0x0D66;
pub const IO_VOICE_16_ADSR_ATT_DEC_SUS_REL: u32 = 0x0D68;
pub const IO_VOICE_16_ADSR_CURR_VOLUME: u32 = 0x0D6C;
pub const IO_VOICE_16_ADPCM_REPEAT_ADDR: u32 = 0x0D6E;
pub const IO_VOICE_17_LEFT_RIGHT: u32 = 0x0D70;
pub const IO_VOICE_17_ADPCM_SAMPLE_RATE: u32 = 0x0D74;
pub const IO_VOICE_17_ADPCM_START_ADDR: u32 = 0x0D76;
pub const IO_VOICE_17_ADSR_ATT_DEC_SUS_REL: u32 = 0x0D78;
pub const IO_VOICE_17_ADSR_CURR_VOLUME: u32 = 0x0D7C;
pub const IO_VOICE_17_ADPCM_REPEAT_ADDR: u32 = 0x0D7E;
pub const IO_SPU_MAIN_VOL_L: u32 = 0x0D80;
pub const IO_SPU_MAIN_VOL_R: u32 = 0x0D82;
pub const IO_SPU_REVERB_OUT_L: u32 = 0x0D84;
pub const IO_SPU_REVERB_OUT_R: u32 = 0x0D86;
pub const IO_SPU_VOICE_KEY_ON: u32 = 0x0D88;
pub const IO_SPU_VOICE_KEY_OFF: u32 = 0x0D8C;
pub const IO_SPU_VOICE_CHN_FM_MODE: u32 = 0x0D90;
pub const IO_SPU_VOICE_CHN_NOISE_MODE: u32 = 0x0D94;
pub const IO_SPU_VOICE_CHN_REVERB_MODE: u32 = 0x0D98;
pub const IO_SPU_VOICE_CHN_ON_OFF_STATUS: u32 = 0x0D9C;
pub const IO_SPU_UNKN_1DA0: u32 = 0x0DA0;
pub const IO_SOUND_RAM_REVERB_WORK_ADDR: u32 = 0x0DA2;
pub const IO_SOUND_RAM_IRQ_ADDR: u32 = 0x0DA4;
pub const IO_SOUND_RAM_DATA_TRANSFER_ADDR: u32 = 0x0DA6;
pub const IO_SOUND_RAM_DATA_TRANSFER_FIFO: u32 = 0x0DA8;
pub const IO_SPU_CTRL_REG_CPUCNT: u32 = 0x0DAA;
pub const IO_SOUND_RAM_DATA_TRANSTER_CTRL: u32 = 0x0DAC;
pub const IO_SPU_STATUS_REG_SPUSTAT: u32 = 0x0DAE;
pub const IO_CD_VOL_L: u32 = 0x0DB0;
pub const IO_CD_VOL_R: u32 = 0x0DB2;
pub const IO_EXT_VOL_L: u32 = 0x0DB4;
pub const IO_EXT_VOL_R: u32 = 0x0DB6;
pub const IO_CURR_MAIN_VOL_L: u32 = 0x0DB8;
pub const IO_CURR_MAIN_VOL_R: u32 = 0x0DBA;
pub const IO_SPU_UNKN_1DBC: u32 = 0x0DBC;
pub const IO_DEBUG_PORT: u32 = 0x1041;

pub const IO_SPU_BASE: u32 = 0x0C00;
pub const IO_SPU_SIZE: u32 = 0x0200;

pub const IO_REG_SIZE: u32 = 0x01080;

struct IoPortHandler {
    load: fn(&Machine, &mut MachineMutation, u32) -> Result<u32, String>,
    store: fn(&mut Machine, u32, u32) -> Result<(), String>,
}

const REG_HANDLER: IoPortHandler = IoPortHandler {
    load: |m: &Machine, mu: &mut MachineMutation, addr: u32| Ok(m.io.regs[addr as usize]),
    store: |m: &mut Machine, addr: u32, val: u32| {
        m.io.regs[addr as usize] = val;
        Ok(())
    },
};

const INTERRUPT_HANDLER: IoPortHandler = IoPortHandler {
    load: |m: &Machine, mu: &mut MachineMutation, addr: u32| match addr {
        IO_I_STAT => Ok(m.cop0.int_stat()),
        IO_I_MASK => Ok(m.cop0.int_mask()),
        _ => Err("Unexpected read from interrupt port".to_string()),
    },
    store: |m: &mut Machine, addr: u32, val: u32| match addr {
        IO_I_STAT => {
            m.cop0.int_set_stat(val);
            Ok(())
        }
        IO_I_MASK => {
            m.cop0.int_set_mask(val);
            Ok(())
        }
        _ => Err("Unexpected write to interrupt port".to_string()),
    },
};

const TIMER_HANDLER: IoPortHandler = IoPortHandler {
    load: |m: &Machine, mu: &mut MachineMutation, addr: u32| {
        let port: &dyn TimerPort = match (addr >> 4) & 0x0F {
            0 => &m.timers.dotclock,
            1 => &m.timers.hretrace,
            2 => &m.timers.sysclock,
            _ => return Err(format!("Unhandled timer port read at 0x{:08X}", addr)),
        };
        match addr & 0x0F {
            0 => Ok(port.read_count()),
            4 => {
                mu.timer_mode_read = Some(addr);
                Ok(port.read_mode())
            }
            8 => Ok(port.read_target()),
            _ => Err(format!("Unhandled timer port read at 0x{:08X}", addr)),
        }
    },
    store: |m: &mut Machine, addr: u32, val: u32| {
        let port: &mut dyn TimerPort = match (addr >> 4) & 0x0F {
            0 => &mut m.timers.dotclock,
            1 => &mut m.timers.hretrace,
            2 => &mut m.timers.sysclock,
            _ => return Err(format!("Unhandled timer port write at 0x{:08X}", addr)),
        };
        match addr & 0x0F {
            0 => Ok(port.write_count(val)),
            4 => Ok(port.write_mode(val)),
            8 => Ok(port.write_target(val)),
            _ => return Err(format!("Unhandled timer port write at 0x{:08X}", addr)),
        }
    },
};

const DEBUG_HANDLER: IoPortHandler = IoPortHandler {
    load: |m: &Machine, mu: &mut MachineMutation, addr: u32| {
        Err("Unexpected read from debug port".to_string())
    },
    store: |m: &mut Machine, addr: u32, val: u32| {
        println!("DEBUG_WRITE: {:08X}", val);
        Ok(())
    },
};

const SPU_HANDLER: IoPortHandler = IoPortHandler {
    load: |m: &Machine, mu: &mut MachineMutation, addr: u32| m.spu.load(addr),
    store: |m: &mut Machine, addr: u32, val: u32| m.spu.store(addr, val),
};

#[derive(Clone, Serialize, Deserialize)]
pub struct IoPort {
    regs: Vec<u32>,
}

impl IoPort {
    pub fn new() -> IoPort {
        IoPort {
            regs: vec![0; IO_REG_SIZE as usize],
        }
    }

    fn lookup_handler(addr: u32) -> Option<&'static IoPortHandler> {
        println!("lookup_handler: {:08X}", addr);
        match addr {
            IO_EXP1_BASE_ADDR | IO_EXP2_BASE_ADDR | IO_EXP1_DELAY_SIZE | IO_EXP3_DELAY_SIZE
            | IO_BIOS_ROM | IO_SPU_DELAY | IO_CDROM_DELAY | IO_EXP2_DELAY_SIZE
            | IO_COMMON_DELAY | IO_RAM_SIZE => Some(&REG_HANDLER),
            IO_I_MASK | IO_I_STAT => Some(&INTERRUPT_HANDLER),
            IO_TMR_DOTCLOCK_VAL | IO_TMR_DOTCLOCK_MODE | IO_TMR_DOTCLOCK_MAX
            | IO_TMR_HRETRACE_VAL | IO_TMR_HRETRACE_MODE | IO_TMR_HRETRACE_MAX
            | IO_TMR_SYSCLOCK_VAL | IO_TMR_SYSCLOCK_MODE | IO_TMR_SYSCLOCK_MAX => {
                Some(&TIMER_HANDLER)
            }
            IO_DEBUG_PORT => Some(&DEBUG_HANDLER),
            addr if addr >= IO_SPU_BASE && addr < IO_SPU_BASE + IO_SPU_SIZE => Some(&SPU_HANDLER),
            _ => None,
        }
    }

    pub fn load(m: &Machine, mu: &mut MachineMutation, addr: u32) -> Result<u32, String> {
        match IoPort::lookup_handler(addr) {
            Some(handler) => (handler.load)(m, mu, addr),
            None => Err(format!("Unhandled IO read at 0x{:08X}", addr)),
        }
    }

    pub fn store(m: &mut Machine, addr: u32, val: u32) -> Result<(), String> {
        match IoPort::lookup_handler(addr) {
            Some(handler) => (handler.store)(m, addr, val),
            None => Err(format!("Unhandled IO write at 0x{:08X}", addr)),
        }
    }
}
