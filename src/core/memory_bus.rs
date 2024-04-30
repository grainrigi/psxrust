use super::Memory;

pub struct MemoryBus {
    bios: Vec<u8>,
}

impl MemoryBus {
    pub fn new(bios: Vec<u8>) -> MemoryBus {
        MemoryBus { bios }
    }
}

impl Memory for MemoryBus {
    fn load(&self, addr: u32) -> u32 {
        let real_addr = addr & 0x5fff_ffff;

        let value = if real_addr < 0x1fc0_0000 {
            0
        } else if real_addr < 0x1fc0_0000 + 0x80000 {
            let base = (real_addr - 0x1fc0_0000) as usize;
            u32::from_le_bytes(self.bios[base..base + 4].try_into().unwrap())
        } else {
            0
        };

        // println!("MEM_READ 0x{:08X} => {:08X}", real_addr, value);

        value
    }

    fn store(&mut self, addr: u32, val: u32) {
        let real_addr = addr & 0x5fff_ffff;

        println!("MEM_WRITE 0x{:08X} <= {:08X}", real_addr, val);

        if real_addr < 0x1fc0_0000 {
            return;
        } else if real_addr < 0x1fc0_0000 + 0x80000 {
            let base = (real_addr - 0x1fc0_0000) as usize;
            self.bios[base..base + 4].copy_from_slice(&val.to_le_bytes());
        } else {
            return;
        }
    }
}
