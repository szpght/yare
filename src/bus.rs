#[derive(Debug)]
pub struct Bus {
    pub memory: Vec<u8>,
}

impl Bus {
    pub fn new() -> Bus {
        let mut bus = Bus {
            memory: Vec::with_capacity(1024 * 1024)
        };
        bus.memory.resize(1024 * 1024, 0);
        bus
    }

    pub fn load8(&self, addr: u64) -> u64 {
        let index = addr as usize;
        self.memory[index] as u64
    }

    pub fn load16(&self, addr: u64) -> u64 {
        let index = addr as usize;
        (self.memory[index] as u64)
            | ((self.memory[index + 1] as u64) << 8)
    }

    pub fn load32(&self, addr: u64) -> u64 {
        let index = addr as usize;
        return (self.memory[index] as u64)
            | ((self.memory[index + 1] as u64) << 8)
            | ((self.memory[index + 2] as u64) << 16)
            | ((self.memory[index + 3] as u64) << 24);
    }

    pub fn load64(&self, addr: u64) -> u64 {
        self.load32(addr) | self.load32(addr + 4) << 32
    }

    // pub fn load128(&self, addr: u64) -> u128 {
    //     self.load32(addr) | self.load32(addr + 4) << 32
    // }

    pub fn store8(&mut self, addr: u64, value: u64) {
        let index = addr as usize;
        self.memory[index] = (value & 0xff) as u8;
    }

    pub fn store16(&mut self, addr: u64, value: u64) {
        let index = addr as usize;
        self.memory[index] = (value & 0xff) as u8;
        self.memory[index + 1] = ((value >> 8) & 0xff) as u8;
    }

    pub fn store32(&mut self, addr: u64, value: u64) {
        let index = addr as usize;
        self.memory[index] = (value & 0xff) as u8;
        self.memory[index + 1] = ((value >> 8) & 0xff) as u8;
        self.memory[index + 2] = ((value >> 16) & 0xff) as u8;
        self.memory[index + 3] = ((value >> 24) & 0xff) as u8;
    }

    pub fn store64(&mut self, addr: u64, value: u64) {
        let index = addr as usize;
        self.memory[index] = (value & 0xff) as u8;
        self.memory[index + 1] = ((value >> 8) & 0xff) as u8;
        self.memory[index + 2] = ((value >> 16) & 0xff) as u8;
        self.memory[index + 3] = ((value >> 24) & 0xff) as u8;
        self.memory[index + 4] = ((value >> 32) & 0xff) as u8;
        self.memory[index + 5] = ((value >> 40) & 0xff) as u8;
        self.memory[index + 6] = ((value >> 48) & 0xff) as u8;
        self.memory[index + 7] = ((value >> 56) & 0xff) as u8;
    }
    
    pub fn store_instruction(&mut self, addr: u64, data: (u64, u64)) {
        self.store64(addr, data.0);
        self.store64(addr + 8, data.1);
    }

    pub fn store128(&mut self, addr: u64, value: u128) {
        self.store64(addr, value as u64);
        self.store64(addr + 8, (value >> 64) as u64);
    }
}