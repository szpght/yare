mod opcodes;
mod bus;
mod instruction;
mod machine;

use std::fs::File;
use std::io::{self, BufReader, Read};
use crate::bus::Bus;
use crate::machine::{Cpu, Machine};

fn main() -> io::Result<()> {
    let mut file = File::open("foo.bin")?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;
    
    let machinussy = Machine::new();
    let mut bussy = Bus::new();
    let mut cpussy = Cpu::new(&machinussy);
    
    for i in 0..buf.len() {
        bussy.store8(i as u64, buf[i] as u64);
    }

    cpussy.step(&mut bussy);
    cpussy.step(&mut bussy);
    cpussy.step(&mut bussy);
    cpussy.step(&mut bussy);
    cpussy.step(&mut bussy);
    cpussy.step(&mut bussy);
    cpussy.step(&mut bussy);

    Ok(())
}
