use std::rc::Rc;

use super::{bus::Bus, Cop0, Cop0ExceptionParams, CpuInstEntry, CpuSlow, MemOpSize};
use rand::{rngs::SmallRng, RngCore, SeedableRng};
use serde::{Deserialize, Serialize};

pub struct Machine {
    pub bios: Vec<u8>,
    pub ram: Vec<u8>,
    pub dcache: Vec<u8>,
    pub bus: Bus,
    pub cop0: Cop0,
    pub cpu: CpuSlow,
}

#[derive(Serialize, Deserialize)]
pub struct MachineState {
    pub bios: Vec<u8>,
    pub ram: Vec<u8>,
    pub dcache: Vec<u8>,
    pub cpu: CpuSlow,
    pub cop0: Cop0,
}

impl Machine {
    pub fn new(bios: Vec<u8>) -> Machine {
        let mut rng = SmallRng::from_entropy();
        let mut m = Machine {
            bios,
            ram: vec![0; 0x20_0000],
            dcache: vec![0; 0x400],
            bus: Bus::new(),
            cop0: Cop0::new(),
            cpu: CpuSlow::new(),
        };
        rng.fill_bytes(m.ram.as_mut_slice());
        rng.fill_bytes(m.dcache.as_mut_slice());
        m
    }

    pub fn save_state(&self) -> MachineState {
        MachineState {
            bios: self.bios.clone(),
            ram: self.ram.clone(),
            dcache: self.dcache.clone(),
            cpu: self.cpu.clone(),
            cop0: self.cop0.clone(),
        }
    }

    pub fn load_state(&mut self, state: MachineState) {
        self.bios = state.bios;
        self.ram = state.ram;
        self.dcache = state.dcache;
        self.cpu = state.cpu;
        self.cop0 = state.cop0;
    }

    pub fn reset(&mut self) {
        self.cpu.reset();
    }

    pub fn cycle(&mut self) -> Result<(), String> {
        let mut mu = MachineMutation::new();

        self.cop0.cycle();
        self.cpu.cycle(self, &mut mu)?;

        Bus::mutate(self, &mu)?;
        // 例外処理のためcop0がcpuより先
        self.cop0.mutate(&mut mu)?;
        self.cpu.mutate(&mu);

        Ok(())
    }
}

pub struct MachineMutation {
    pub branch_target: Option<u32>,
    pub reg_write: Option<(u8, u32)>,
    pub bus_write: Option<(u32, u32, MemOpSize)>,
    pub cop0_write: Option<(u8, u32)>,
    pub hilo_write: Option<(u32, u32)>,
    pub cop0_exception: Option<Cop0ExceptionParams>,
    pub exception_branch: Option<u32>,
    pub exception_return: bool,
    pub icache_write: Option<(u32, Rc<CpuInstEntry>)>,
}

impl MachineMutation {
    pub fn new() -> MachineMutation {
        MachineMutation {
            branch_target: None,
            reg_write: None,
            bus_write: None,
            cop0_write: None,
            hilo_write: None,
            cop0_exception: None,
            exception_branch: None,
            exception_return: false,
            icache_write: None,
        }
    }
}
