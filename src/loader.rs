use elf::{ElfBytes, ParseError};
use elf::abi::PT_LOAD;
use elf::endian::{LittleEndian};

use crate::bus::Bus;

pub fn load_elf_file(bus: &mut Bus, elf_bytes: &[u8]) -> Result<EntryPoint, LoaderError> {
    let elf_file = ElfBytes::<LittleEndian>::minimal_parse(elf_bytes)
        .or_else(|x| Err(LoaderError::ParseError(x)))?;

    let segments_to_load = elf_file
        .segments().ok_or(LoaderError::NoSegments)?
        .iter().filter(|x| x.p_type == PT_LOAD);

    for segment in segments_to_load {
        let from = segment.p_offset as usize;
        let length = segment.p_filesz as usize;

        bus.store_bytes(segment.p_paddr, &elf_bytes[from..from + length]);
    }

    Ok(EntryPoint(elf_file.ehdr.e_entry))
}

pub struct EntryPoint(u64);

impl EntryPoint {
    pub fn virtual_address(&self) -> u64 {
        self.0
    }
}

#[derive(Debug)]
pub enum LoaderError {
    NoSegments,
    ParseError(ParseError),
}

