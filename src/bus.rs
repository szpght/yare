#[derive(Debug)]
pub struct Bus {
    pub memory: Vec<u8>,
}

impl Bus {
    pub fn new(memory_size: usize) -> Bus {
        let mut bus = Bus {
            memory: Vec::new()
        };
        bus.memory.resize(memory_size, 0);
        bus
    }

    pub fn load8(&self, addr: u64) -> u64 {
        let index = addr as usize;
        self.memory[index] as u64
    }

    pub fn load16(&self, addr: u64) -> u64 {
        let index = addr as usize;
        let bytes = &self.memory[index..index + 2];
        u16::from_le_bytes(bytes.try_into().unwrap()) as u64
    }

    pub fn load32(&self, addr: u64) -> u64 {
        let index = addr as usize;
        let bytes = &self.memory[index..index + 4];
        u32::from_le_bytes(bytes.try_into().unwrap()) as u64
    }

    pub fn load64(&self, addr: u64) -> u64 {
        let index = addr as usize;
        let bytes = &self.memory[index..index + 8];
        u64::from_le_bytes(bytes.try_into().unwrap())
    }

    pub fn store8(&mut self, addr: u64, value: u64) {
        let index = addr as usize;
        self.memory[index] = value as u8;
    }

    pub fn store16(&mut self, addr: u64, value: u64) {
        let index = addr as usize;
        self.memory[index..index + 2].copy_from_slice(&(value as u16).to_le_bytes());
    }

    pub fn store32(&mut self, addr: u64, value: u64) {
        let index = addr as usize;
        self.memory[index..index + 4].copy_from_slice(&(value as u32).to_le_bytes());
    }

    pub fn store64(&mut self, addr: u64, value: u64) {
        let index = addr as usize;
        self.memory[index..index + 8].copy_from_slice(&value.to_le_bytes());
    }
}