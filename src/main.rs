use std::fs;
use std::fs::File;
use std::io::stdin;
use std::io::Read;

use clap::Parser;
use pprof::protos::Message;
use psxrust::core::Machine;
use psxrust::core::MachineState;
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    bios: String,
    #[arg(short, long)]
    state: Option<String>,
}

fn load_bios(path: &str) -> Vec<u8> {
    let mut file = File::open(path).expect("file not found");
    let metadata = fs::metadata(path).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    file.read(&mut buffer).expect("buffer overflow");

    buffer
}

fn save_state(path: &str, state: MachineState) {
    let mut file = File::create(path).expect("file not found");
    let mut serializer = rmp_serde::Serializer::new(&mut file);
    state.serialize(&mut serializer);
}

fn load_state(path: &str) -> Option<MachineState> {
    match File::open(path) {
        Ok(mut file) => {
            let mut deserializer = rmp_serde::Deserializer::new(&mut file);
            Some(MachineState::deserialize(&mut deserializer).expect("failed to deserialize"))
        }
        Err(_) => None,
    }
}

fn main() {
    let guard = pprof::ProfilerGuardBuilder::default()
        .frequency(1000)
        .blocklist(&["libc", "libgcc", "pthread", "vdso"])
        .build()
        .unwrap();

    let args = Args::parse();

    let bios = load_bios(&args.bios);

    let mut machine = Machine::new(bios);

    machine.reset();

    if let Some(state_path) = args.state {
        if let Some(state) = load_state(&state_path) {
            machine.load_state(state);
        }
    }

    loop {
        machine.cycle().expect("Failed to cycle");
        if machine.cpu.current_pc() == 0x80030000 {
            let state = machine.save_state();
            save_state("state.bin", state);
            println!("Saved state");
            if let Ok(report) = guard.report().build() {
                let mut file = File::create("profile.pb").expect("failed to create file");
                let profile = report.pprof().unwrap();

                profile
                    .write_to_writer(&mut file)
                    .expect("failed to write profile");
            }
        }
        continue;

        let mut throwaway = String::new();
        stdin()
            .read_line(&mut throwaway)
            .expect("Failed to read line");
    }
}
