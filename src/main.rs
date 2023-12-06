mod opcodes;
mod bus;
mod instruction;
mod machine;
mod loader;

use std::fs::File;
use std::io::{self, Read};

use crate::bus::Bus;
use crate::machine::{Cpu, Machine};

fn main() -> io::Result<()> {
    let machinussy = Machine::new();
    let mut bussy = Bus::new(64 * 1024 * 1024);
    let mut cpussy = Cpu::new(&machinussy);

    {
        let mut file = File::open("kod.elf")?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        let entry_point = loader::load_elf_file(&mut bussy, buf.as_ref())
            .expect("ELF parse error");
        cpussy.pc = entry_point.virtual_address();
    }

    loop {
        cpussy.step(&mut bussy);
    }
}
