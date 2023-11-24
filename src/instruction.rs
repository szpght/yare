pub struct Instruction {
    pub raw: i32,
    pub size: u64,
    pub opcode: i32,
    pub rd: i32,
    pub rs1: i32,
    pub rs2: i32,
    pub funct3: i32,
    pub funct7: i32,
    pub shamt: i32,
}

impl Instruction {
    pub fn decode(data: i32) -> Instruction {
        let opcode = (data & 0x7F);
        let rd = (data >> 7) & 0x1F;
        let funct3 = (data >> 12) & 0x07;
        let rs1 = (data >> 15) & 0x1F;
        let rs2 = (data >> 20) & 0x1F;
        let funct7 = (data >> 25) & 0x7F;

        Instruction {
            raw: data,
            size: 4,
            opcode,
            rd,
            funct3,
            rs1,
            rs2,
            funct7,
            shamt: rs2,
        }
    }
    
    // todo sprawdzić czy przez przypadek nie usuwam extendowanych jedynek którymś andem
    pub fn immediate_i(&self) -> i64 { (self.raw >> 20) as i64 }
    pub fn immediate_u(&self) -> i64 { (self.raw & 0xFFF) as i64 }
    pub fn immediate_s(&self) -> i64 { (((self.raw >> 7) & 0x1F) | ((self.raw >> 20) & 0xFE0)) as i64 }
    pub fn immediate_j(&self) -> i64 { ((((self.raw >> 21) & 0x3FF) | ((self.raw >> 9) & 0x1) | (self.raw & 0xFF000) | ((self.raw >> 11) & 0x1)) as i64) >> 1 } // to jest pewnie zepsute
    pub fn immediate_b(&self) -> i64 { (((self.raw >> 7) & 0x1E) | ((self.raw >> 20) & 0x7E0) | ((self.raw << 4) & 0x800) | ((self.raw >> 19) & 0xF800)) as i64 }
    pub fn immediate_i_unsigned(&self) -> u64 { self.immediate_i() as u64 }
    pub fn immediate_u_unsigned(&self) -> u64 { self.immediate_u() as u64 }
    pub fn immediate_s_unsigned(&self) -> u64 { self.immediate_s() as u64 }
}
