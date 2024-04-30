#[derive(Copy, Clone)]
pub enum MemOpSize {
    Byte,
    Half,
    Word,
}

pub fn mem_store(mem: &mut [u8], addr: u32, val: u32, size: MemOpSize) {
    match size {
        MemOpSize::Byte => mem_store_u8(mem, addr, val as u8),
        MemOpSize::Half => mem_store_u16(mem, addr, val as u16),
        MemOpSize::Word => mem_store_u32(mem, addr, val),
    }
}

pub fn mem_load(mem: &[u8], addr: u32, size: MemOpSize) -> u32 {
    match size {
        MemOpSize::Byte => mem_load_u8(mem, addr) as u32,
        MemOpSize::Half => mem_load_u16(mem, addr) as u32,
        MemOpSize::Word => mem_load_u32(mem, addr),
    }
}

pub fn mem_store_u8(mem: &mut [u8], addr: u32, val: u8) {
    mem[addr as usize] = val;
}

pub fn mem_store_u16(mem: &mut [u8], addr: u32, val: u16) {
    let bytes = val.to_le_bytes();
    mem[addr as usize..addr as usize + 2].copy_from_slice(&bytes);
}

pub fn mem_store_u32(mem: &mut [u8], addr: u32, val: u32) {
    let bytes = val.to_le_bytes();
    mem[addr as usize..addr as usize + 4].copy_from_slice(&bytes);
}

pub fn mem_load_u8(mem: &[u8], addr: u32) -> u8 {
    mem[addr as usize]
}

pub fn mem_load_u16(mem: &[u8], addr: u32) -> u16 {
    let bytes: [u8; 2] = mem[addr as usize..addr as usize + 2].try_into().unwrap();
    u16::from_le_bytes(bytes)
}

pub fn mem_load_u32(mem: &[u8], addr: u32) -> u32 {
    let bytes: [u8; 4] = mem[addr as usize..addr as usize + 4].try_into().unwrap();
    u32::from_le_bytes(bytes)
}
