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
    pub shamtw: i32,
}

impl Instruction {
    pub fn decode(data: i32) -> Instruction {
        if data & 3 == 3 {
            Instruction {
                raw: data,
                size: 4,
                opcode: data & 0x7F,
                rd: (data >> 7) & 0x1F,
                funct3: (data >> 12) & 0x07,
                rs1: (data >> 15) & 0x1F,
                rs2: (data >> 20) & 0x1F,
                funct7: (data >> 25) & 0x7F,
                shamt: (data >> 20) & 0x3F,
                shamtw: (data >> 20) & 0x1F,
            }
        } else {
            unimplemented!("Instruction with length other that 4 encountered");
        }
    }

    pub fn csr(&self) -> u64 { ((self.raw as u64) >> 20) & 0xFFF }
    pub fn immediate_i(&self) -> i64 { (self.raw >> 20) as i64 }
    pub fn immediate_u(&self) -> i64 { (self.raw & !0xFFF) as i64 }
    pub fn immediate_s(&self) -> i64 { (((self.raw >> 7) & 0x1F) | ((self.raw >> 20) & !0x1F)) as i64 }

    pub fn immediate_b(&self) -> i64 {
        (((self.raw >> 8 - 1) & 0b_1111_0) | ((self.raw >> 25 - 5) & 0b_11_1111_00000) | ((self.raw << -(7 - 11)) & 0b1_000_0000_0000) | (self.raw >> 31 - 12) & !0b111111111111) as i64
    }

    pub fn immediate_j(&self) -> i64 {
        ((self.raw >> 21 - 1) & 0b11111111110 | (self.raw >> 20 - 11) & 0b100000000000 | self.raw & 0b11111111000000000000 | (self.raw >> 31-20) & !0xFFFFF) as i64
    }

    pub fn immediate_i_unsigned(&self) -> u64 { self.immediate_i() as u64 }
    pub fn immediate_u_unsigned(&self) -> u64 { self.immediate_u() as u64 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_immediate_b() {
        let i = Instruction::decode(-1);
        assert_eq!(i.immediate_b(), -2);
    }

    #[test]
    fn test_immediate_j() {
        let i = Instruction::decode(-1);
        assert_eq!(i.immediate_j(), -2);
    }

    #[test]
    fn test_immediate_u() {
        let i = Instruction::decode(-1);
        assert_eq!(i.immediate_u_unsigned(), u64::MAX - 4095);
    }

    #[test]
    fn test_immediate_s() {
        let i = Instruction::decode(-1);
        assert_eq!(i.immediate_s(), -1);
    }
}