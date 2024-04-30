pub trait Memory {
    fn load(&self, addr: u32) -> u32;
    fn store(&mut self, addr: u32, val: u32);
}
