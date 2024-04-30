use super::{machine::Machine, mem_load, mem_store, MachineMutation, MemOpSize};

struct MemoryRegion {
    base: u32,
    size: u32,
    load: fn(&Machine, u32, MemOpSize) -> u32,
    store: fn(&mut Machine, u32, u32, MemOpSize),
}

pub struct Bus {
    handlers: Vec<MemoryRegion>,
    handlers_isc: Vec<MemoryRegion>,
    isolate_cache: bool,
}

fn bios_region(base: u32) -> MemoryRegion {
    MemoryRegion {
        // OSROM
        base,
        size: 0x80000,
        load: |m: &Machine, addr: u32, size: MemOpSize| mem_load(&m.bios, addr, size),
        store: |m: &mut Machine, addr: u32, val: u32, size: MemOpSize| {
            mem_store(&mut m.bios, addr, val, size)
        },
    }
}

fn ram_region(base: u32) -> MemoryRegion {
    MemoryRegion {
        base,
        size: 0x200000 * 4,
        load: |m: &Machine, addr: u32, size: MemOpSize| mem_load(&m.ram, addr & 0x001FFFFF, size),
        store: |m: &mut Machine, addr: u32, val: u32, size: MemOpSize| {
            mem_store(&mut m.ram, addr & 0x001FFFFF, val, size)
        },
    }
}

fn dcache_region(base: u32) -> MemoryRegion {
    MemoryRegion {
        base,
        size: 0x400,
        load: |m: &Machine, addr: u32, size: MemOpSize| mem_load(&m.dcache, addr, size),
        store: |m: &mut Machine, addr: u32, val: u32, size: MemOpSize| {
            mem_store(&mut m.dcache, addr, val, size)
        },
    }
}

// Expansion ROM 0
fn exp0_region(base: u32) -> MemoryRegion {
    MemoryRegion {
        base,
        size: 0x800000,
        load: |m: &Machine, addr: u32, size: MemOpSize| {
            println!("WARN: Unhandled EXP0 read at 0x{:08X}", addr);
            0
        },
        store: |m: &mut Machine, addr: u32, val: u32, size: MemOpSize| {
            println!("WARN: Unhandled EXP0 write at 0x{:08X}", addr);
        },
    }
}

fn io_region(base: u32) -> MemoryRegion {
    MemoryRegion {
        base,
        size: 0x1080,
        load: |m: &Machine, addr: u32, size: MemOpSize| {
            println!("WARN: Unhandled IO read at 0x{:08X}", addr);
            0
        },
        store: |m: &mut Machine, addr: u32, val: u32, size: MemOpSize| {
            println!("WARN: Unhandled IO write at 0x{:08X}", addr);
        },
    }
}

fn isolate_cache_region(base: u32) -> MemoryRegion {
    MemoryRegion {
        base,
        size: 0x40000000,
        load: |m: &Machine, addr: u32, size: MemOpSize| {
            println!("WARN: Unhandled isolate cache read at 0x{:08X}", addr);
            0
        },
        store: |m: &mut Machine, addr: u32, val: u32, size: MemOpSize| {
            println!("WARN: Unhandled isolate cache write at 0x{:08X}", addr);
        },
    }
}

fn cache_ctrl(base: u32) -> MemoryRegion {
    MemoryRegion {
        base,
        size: 0x10,
        load: |m: &Machine, addr: u32, size: MemOpSize| {
            println!("WARN: Unhandled cache control read at 0x{:08X}", addr);
            0
        },
        store: |m: &mut Machine, addr: u32, val: u32, size: MemOpSize| {
            println!("WARN: Unhandled cache control write at 0x{:08X}", addr);
        },
    }
}

impl Bus {
    pub fn new() -> Bus {
        const KUSEG: u32 = 0x0000_0000;
        const KSEG0: u32 = 0x8000_0000;
        const KSEG1: u32 = 0xa000_0000;

        Bus {
            handlers: vec![
                // KUSEG
                ram_region(KUSEG + 0x0000_0000),
                exp0_region(KUSEG + 0x1F00_0000),
                dcache_region(KUSEG + 0x1F80_0000),
                io_region(KUSEG + 0x1F80_1000),
                bios_region(KUSEG + 0x1FC0_0000),
                // KSEG0
                ram_region(KSEG0 + 0x0000_0000),
                exp0_region(KSEG0 + 0x1F00_0000),
                dcache_region(KSEG0 + 0x1F80_0000),
                io_region(KSEG0 + 0x1F80_1000),
                bios_region(KSEG0 + 0x1FC0_0000),
                // KSEG1
                ram_region(KSEG1 + 0x0000_0000),
                exp0_region(KSEG1 + 0x1F00_0000),
                io_region(KSEG1 + 0x1F80_1000),
                bios_region(KSEG1 + 0x1FC0_0000),
                // OTHER
                cache_ctrl(0xFFFE_0130),
            ],
            handlers_isc: vec![
                // KUSEG
                bios_region(KUSEG + 0x1FC0_0000),
                // KSEG0
                bios_region(KSEG0 + 0x1FC0_0000),
                // KSEG1
                bios_region(KSEG1 + 0x1FC0_0000),
                // cache fallback
                isolate_cache_region(KUSEG),
                isolate_cache_region(KSEG0),
            ],
            isolate_cache: false,
        }
    }

    pub fn lb(m: &Machine, addr: u32) -> Result<u32, String> {
        Bus::load(m, addr, MemOpSize::Byte)
    }

    pub fn lh(m: &Machine, addr: u32) -> Result<u32, String> {
        Bus::load(m, addr, MemOpSize::Half)
    }

    pub fn lw(m: &Machine, addr: u32) -> Result<u32, String> {
        Bus::load(m, addr, MemOpSize::Word)
    }

    fn load(m: &Machine, addr: u32, size: MemOpSize) -> Result<u32, String> {
        match m.bus.lookup_region(addr) {
            Some(region) => Ok((region.load)(m, addr - region.base, size)),
            None => Err(format!("Unhandled memory read at 0x{:08X}", addr)),
        }
    }

    fn store(m: &mut Machine, addr: u32, val: u32, size: MemOpSize) -> Result<(), String> {
        match m.bus.lookup_region(addr) {
            Some(region) => {
                println!("MEM_WRITE 0x{:08X} <= {:08X}", addr, val);
                (region.store)(m, addr - region.base, val, size);
                Ok(())
            }
            None => Err(format!("Unhandled memory write at 0x{:08X}", addr)),
        }
    }

    fn lookup_region(&self, addr: u32) -> Option<&MemoryRegion> {
        let handlers = if self.isolate_cache {
            // &self.handlers_isc
            &self.handlers_isc
        } else {
            &self.handlers
        };

        for region in handlers {
            if addr >= region.base {
                // println!("lookup_region: 0x{:08X} >= 0x{:08X}", addr, region.base);
                if addr < region.base + region.size {
                    return Some(region);
                }
            }
        }

        None
    }

    pub fn mutate(m: &mut Machine, mu: &MachineMutation) -> Result<(), String> {
        if let Some((addr, val, size)) = mu.bus_write {
            Bus::store(m, addr, val, size)?;
        }
        m.bus.isolate_cache = m.cop0.status_isc();
        Ok(())
    }
}
