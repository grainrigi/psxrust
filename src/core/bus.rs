use super::{ioport::IoPort, machine::Machine, mem_load, mem_store, MachineMutation, MemOpSize};

struct MemoryRegion {
    base: u32,
    size: u32,
    load: fn(&Machine, &mut MachineMutation, u32, MemOpSize) -> Result<u32, String>,
    store: fn(&mut Machine, &mut MachineMutation, u32, u32, MemOpSize) -> Result<(), String>,
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
        load: |m: &Machine, _: &mut MachineMutation, addr: u32, size: MemOpSize| {
            Ok(mem_load(&m.bios, addr, size))
        },
        store: |m: &mut Machine, _: &mut MachineMutation, addr: u32, val: u32, size: MemOpSize| {
            mem_store(&mut m.bios, addr, val, size);
            Ok(())
        },
    }
}

fn ram_region(base: u32) -> MemoryRegion {
    MemoryRegion {
        base,
        size: 0x200000 * 4,
        load: |m: &Machine, _: &mut MachineMutation, addr: u32, size: MemOpSize| {
            Ok(mem_load(&m.ram, addr & 0x001FFFFF, size))
        },
        store: |m: &mut Machine, _: &mut MachineMutation, addr: u32, val: u32, size: MemOpSize| {
            mem_store(&mut m.ram, addr & 0x001FFFFF, val, size);
            Ok(())
        },
    }
}

fn dcache_region(base: u32) -> MemoryRegion {
    MemoryRegion {
        base,
        size: 0x400,
        load: |m: &Machine, _: &mut MachineMutation, addr: u32, size: MemOpSize| {
            Ok(mem_load(&m.dcache, addr, size))
        },
        store: |m: &mut Machine, _: &mut MachineMutation, addr: u32, val: u32, size: MemOpSize| {
            mem_store(&mut m.dcache, addr, val, size);
            Ok(())
        },
    }
}

// Expansion ROM 0
fn exp0_region(base: u32) -> MemoryRegion {
    MemoryRegion {
        base,
        size: 0x800000,
        load: |_: &Machine, _: &mut MachineMutation, addr: u32, _: MemOpSize| {
            println!("WARN: Unhandled EXP0 read at 0x{:08X}", addr);
            Ok(0)
        },
        store: |_: &mut Machine, _: &mut MachineMutation, addr: u32, _: u32, _: MemOpSize| {
            println!("WARN: Unhandled EXP0 write at 0x{:08X}", addr);
            Ok(())
        },
    }
}

fn io_region(base: u32) -> MemoryRegion {
    MemoryRegion {
        base,
        size: 0x1080,
        load: |m: &Machine, mu: &mut MachineMutation, addr: u32, _: MemOpSize| {
            IoPort::load(m, mu, addr)
        },
        store: |m: &mut Machine, _: &mut MachineMutation, addr: u32, val: u32, _: MemOpSize| {
            IoPort::store(m, addr, val)
        },
    }
}

fn isolate_cache_region(base: u32) -> MemoryRegion {
    MemoryRegion {
        base,
        size: 0x40000000,
        load: |_: &Machine, _: &mut MachineMutation, addr: u32, _: MemOpSize| {
            println!("WARN: Unhandled isolate cache read at 0x{:08X}", addr);
            Ok(0)
        },
        store: |_: &mut Machine, _: &mut MachineMutation, addr: u32, _: u32, _: MemOpSize| {
            println!("WARN: Unhandled isolate cache write at 0x{:08X}", addr);
            Ok(())
        },
    }
}

fn cache_ctrl(base: u32) -> MemoryRegion {
    MemoryRegion {
        base,
        size: 0x10,
        load: |_: &Machine, _: &mut MachineMutation, addr: u32, _: MemOpSize| {
            println!("WARN: Unhandled cache control read at 0x{:08X}", addr);
            Ok(0)
        },
        store: |_: &mut Machine, _: &mut MachineMutation, addr: u32, _: u32, _: MemOpSize| {
            println!("WARN: Unhandled cache control write at 0x{:08X}", addr);
            Ok(())
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

    pub fn lb(m: &Machine, mu: &mut MachineMutation, addr: u32) -> Result<u32, String> {
        Bus::load(m, mu, addr, MemOpSize::Byte)
    }

    pub fn lh(m: &Machine, mu: &mut MachineMutation, addr: u32) -> Result<u32, String> {
        Bus::load(m, mu, addr, MemOpSize::Half)
    }

    pub fn lw(m: &Machine, mu: &mut MachineMutation, addr: u32) -> Result<u32, String> {
        Bus::load(m, mu, addr, MemOpSize::Word)
    }

    fn load(
        m: &Machine,
        mu: &mut MachineMutation,
        addr: u32,
        size: MemOpSize,
    ) -> Result<u32, String> {
        match m.bus.lookup_region(addr) {
            Some(region) => (region.load)(m, mu, addr - region.base, size),
            None => Err(format!("Unhandled memory read at 0x{:08X}", addr)),
        }
    }

    fn store(
        m: &mut Machine,
        mu: &mut MachineMutation,
        addr: u32,
        val: u32,
        size: MemOpSize,
    ) -> Result<(), String> {
        match m.bus.lookup_region(addr) {
            Some(region) => {
                println!("MEM_WRITE 0x{:08X} <= {:08X}", addr, val);
                (region.store)(m, mu, addr - region.base, val, size)
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

    pub fn mutate(m: &mut Machine, mu: &mut MachineMutation) -> Result<(), String> {
        if let Some((addr, val, size)) = mu.bus_write {
            Bus::store(m, mu, addr, val, size)?;
        }
        m.bus.isolate_cache = m.cop0.status_isc();
        Ok(())
    }
}
