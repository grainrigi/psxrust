use super::ioport::*;

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Spu {
    master_volume_left: u16,
    master_volume_right: u16,
    reverb_volume_left: u16,
    reverb_volume_right: u16,
}

impl Spu {
    pub fn new() -> Spu {
        Spu {
            master_volume_left: 0,
            master_volume_right: 0,
            reverb_volume_left: 0,
            reverb_volume_right: 0,
        }
    }

    pub fn load(&self, addr: u32) -> Result<u32, String> {
        match addr {
            _ => Err(format!("SPU: Unimplemented load at 0x{:08X}", addr)),
        }
    }

    pub fn store(&mut self, addr: u32, val: u32) -> Result<(), String> {
        match addr {
            IO_SPU_MAIN_VOL_L => {
                self.master_volume_left = val as u16;
                Ok(())
            }
            IO_SPU_MAIN_VOL_R => {
                self.master_volume_right = val as u16;
                Ok(())
            }
            IO_SPU_REVERB_OUT_L => {
                self.reverb_volume_left = val as u16;
                Ok(())
            }
            IO_SPU_REVERB_OUT_R => {
                self.reverb_volume_right = val as u16;
                Ok(())
            }
            _ => Err(format!("SPU: Unimplemented store at 0x{:08X}", addr)),
        }
    }
}
