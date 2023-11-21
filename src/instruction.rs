pub struct Instruction {
    pub opcode: u8,
    pub reg_a: u8,
    pub reg_b: u8,
    pub reg_target: u8,
    pub offset: i32,
    pub immediate: u64,
}

impl Instruction {
    /// Size of single instruction in memory
    pub const SIZE: u64 = 16;

    pub fn decode(data: u64, immediate: u64) -> Instruction {
        let opcode = (data & 0xFF) as u8;
        let reg_a = ((data >> 8) & 0xFF) as u8;
        let reg_b = ((data >> 16) & 0xFF) as u8;
        let reg_target = ((data >> 24) & 0xFF) as u8;
        let offset = (data >> 32) as u32 as i32;

        Instruction {
            opcode, reg_a, reg_b, reg_target, immediate, offset
        }
    }

    pub fn encode(&self) -> (u64, u64) {
        let data = self.opcode as u64 | (self.reg_a as u64) << 8 | (self.reg_b as u64) << 16 | (self.reg_target as u64) << 24 | (self.offset as u32 as u64) << 32;
        (data, self.immediate)
    }
}
