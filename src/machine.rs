use std::time::Instant;
use crate::bus::Bus;
use crate::instruction::Instruction;
use crate::opcodes::*;

#[derive(Debug)]
pub struct Machine {
    pub start: Instant,
}

impl Machine {
    pub fn new() -> Machine {
        Machine { start: Instant::now() }
    }
}

#[derive(Debug)]
pub struct Cpu<'a> {
    pub machine: &'a Machine,
    pub registers: [u64; 32],
    pub pc: u64,
    pub cycles: u64,
    pub instructions_retired: u64,
}

impl Cpu<'_> {
    pub fn new(machine: &Machine) -> Cpu {
        Cpu { registers: [0; 32], pc: 0, cycles: 0, instructions_retired: 0, machine }
    }

    pub fn work(&mut self, bus: &mut Bus) {
        loop {
            self.step(bus);
        }
    }

    pub fn step(&mut self, bus: &mut Bus) {
        let instruction = self.fetch(bus);
        self.execute(&instruction, bus);
    }

    pub fn fetch(&mut self, bus: &mut Bus) -> Instruction {
        let data = bus.load32(self.pc) as i32;
        Instruction::decode(data)
    }

    pub fn execute(&mut self, instruction: &Instruction, bus: &mut Bus) {
        let pc = self.pc;
        let next_instruction_address = self.pc + instruction.size;
        let mut new_pc = next_instruction_address;
        let rs1_value = self.read_register(instruction.rs1);
        let rs2_value = self.read_register(instruction.rs2);
        let rs1_value_signed = rs1_value as i64;
        let rs2_value_signed = rs2_value as i64;

        let mut new_rd_value = None;
        let mut write_rd = |value: u64| new_rd_value = Some(value);

        match (instruction.opcode, instruction.funct3, instruction.funct7) {
            (OPCODE_OP_IMM, F3_ADD, _) => write_rd(rs1_value.wrapping_add_signed(instruction.immediate_i())),
            (OPCODE_OP_IMM, F3_SLT, _) => write_rd((rs1_value_signed < instruction.immediate_i()) as u64),
            (OPCODE_OP_IMM, F3_SLTU, _) => write_rd((rs1_value < instruction.immediate_i_unsigned()) as u64),
            (OPCODE_OP_IMM, F3_AND, _) => write_rd(instruction.immediate_i_unsigned() & rs1_value),
            (OPCODE_OP_IMM, F3_OR, _) => write_rd(instruction.immediate_i_unsigned() | rs1_value),
            (OPCODE_OP_IMM, F3_XOR, _) => write_rd(instruction.immediate_i_unsigned() ^ rs1_value),
            (OPCODE_OP_IMM, F3_SLL, _) => write_rd(rs1_value << instruction.shamt),
            (OPCODE_OP_IMM, F3_SRL, F7_SRL) => write_rd(rs1_value >> instruction.shamt),
            (OPCODE_OP_IMM, F3_SRA, F7_SRA) => write_rd((rs1_value_signed >> instruction.shamt) as u64),

            (OPCODE_OP_IMM_32, F3_ADD, _) => write_rd((rs1_value as u32).wrapping_add_signed(instruction.immediate_i() as i32) as i32 as u64),
            (OPCODE_OP_IMM_32, F3_SLL, _) => write_rd(((rs1_value as u32) << instruction.shamt) as u64),
            (OPCODE_OP_IMM_32, F3_SRL, F7_SRL) => write_rd(((rs1_value as u32) >> instruction.shamtw) as u64),
            (OPCODE_OP_IMM_32, F3_SRA, F7_SRA) => write_rd(((rs1_value_signed as i32) >> instruction.shamt) as u64),

            (OPCODE_LUI, _, _) => write_rd(instruction.immediate_u_unsigned()),

            (OPCODE_AUIPC, _, _) => write_rd(pc + instruction.immediate_u_unsigned()),

            (OPCODE_OP, F3_ADD, F7_ADD) => write_rd(rs1_value.wrapping_add(rs2_value)),
            (OPCODE_OP, F3_SLT, F7_SLT) => write_rd((rs1_value_signed < rs2_value_signed) as u64),
            (OPCODE_OP, F3_SLTU, F7_SLTU) => write_rd((rs1_value < rs2_value) as u64),
            (OPCODE_OP, F3_AND, F7_AND) => write_rd(rs1_value & rs2_value),
            (OPCODE_OP, F3_OR, F7_OR) => write_rd(rs1_value | rs2_value),
            (OPCODE_OP, F3_XOR, F7_XOR) => write_rd(rs1_value ^ rs2_value),
            (OPCODE_OP, F3_SLL, F7_SLL) => write_rd(rs1_value << (rs2_value & 0x3F)),
            (OPCODE_OP, F3_SRL, F7_SRL) => write_rd(rs1_value >> (rs2_value & 0x3F)),
            (OPCODE_OP, F3_SRA, F7_SRA) => write_rd((rs1_value_signed >> (rs2_value & 0x3F)) as u64),
            (OPCODE_OP, F3_SUB, F7_SUB) => write_rd(rs1_value.wrapping_sub(rs2_value)),
            (OPCODE_OP, F3_MUL, F7_MULDIV) => write_rd(rs1_value.wrapping_mul(rs2_value)),
            (OPCODE_OP, F3_MULH, F7_MULDIV) => write_rd(((rs1_value_signed as i128).wrapping_mul(rs2_value_signed as i128) >> 64) as u64),
            (OPCODE_OP, F3_MULHU, F7_MULDIV) => write_rd(mulhu(rs1_value, rs2_value)),
            (OPCODE_OP, F3_MULHSU, F7_MULDIV) => write_rd(mulhsu(rs1_value_signed, rs2_value)),
            (OPCODE_OP, F3_DIV, F7_MULDIV) => write_rd(div_signed(rs1_value, rs2_value).0),
            (OPCODE_OP, F3_REM, F7_MULDIV) => write_rd(div_signed(rs1_value, rs2_value).1),
            (OPCODE_OP, F3_DIVU, F7_MULDIV) => write_rd(div_unsigned(rs1_value, rs2_value).0),
            (OPCODE_OP, F3_REMU, F7_MULDIV) => write_rd(div_unsigned(rs1_value, rs2_value).1),

            // TODO less casts?
            (OPCODE_OP_32, F3_ADD, _) => write_rd(rs1_value.wrapping_add(rs2_value) as i32 as u64),
            (OPCODE_OP_32, F3_SUB, _) => write_rd((rs1_value as u32).wrapping_sub(rs2_value as u32) as i32 as u64),
            (OPCODE_OP_32, F3_SLL, _) => write_rd(((rs1_value as u32) << ((rs2_value as u32) & 0x1F)) as u64),
            (OPCODE_OP_32, F3_SRL, _) => write_rd(((rs1_value as u32) >> ((rs2_value as u32) & 0x1F)) as u64),
            (OPCODE_OP_32, F3_SRA, _) => write_rd(((rs1_value_signed as i32) >> (rs2_value & 0x1F)) as u64),
            (OPCODE_OP_32, F3_MULW, F7_MULDIV) => write_rd((rs1_value as u32).wrapping_mul(rs2_value as u32) as i32 as u64),
            (OPCODE_OP_32, F3_DIVW, F7_MULDIV) => write_rd(div_signed(rs1_value as i32 as u64, rs2_value as i32 as u64).0 as u32 as u64),
            (OPCODE_OP_32, F3_REMW, F7_MULDIV) => write_rd(div_signed(rs1_value as i32 as u64, rs2_value as i32 as u64).1 as u32 as u64),
            (OPCODE_OP_32, F3_DIVUW, F7_MULDIV) => write_rd(div_unsigned(rs1_value as u32 as u64, rs2_value as u32 as u64).0 as u32 as u64),
            (OPCODE_OP_32, F3_REMUW, F7_MULDIV) => write_rd(div_unsigned(rs1_value as u32 as u64, rs2_value as u32 as u64).1 as u32 as u64),

            (OPCODE_JAL, _, _) => {
                write_rd(next_instruction_address);
                new_pc = pc.wrapping_add_signed(instruction.immediate_j())
            }

            (OPCODE_JALR, _, _) => {
                write_rd(next_instruction_address);
                new_pc = rs1_value.wrapping_add_signed(instruction.immediate_i()) & (!1);
            }

            (OPCODE_BRANCH, F3_BEQ, _) => if rs1_value == rs2_value { new_pc = pc.wrapping_add_signed(instruction.immediate_b()) },
            (OPCODE_BRANCH, F3_BNE, _) => if rs1_value != rs2_value { new_pc = pc.wrapping_add_signed(instruction.immediate_b()) },
            (OPCODE_BRANCH, F3_BLT, _) => if rs1_value_signed < rs2_value_signed { new_pc = pc.wrapping_add_signed(instruction.immediate_b()) },
            (OPCODE_BRANCH, F3_BGE, _) => if rs1_value_signed >= rs2_value_signed { new_pc = pc.wrapping_add_signed(instruction.immediate_b()) },
            (OPCODE_BRANCH, F3_BLTU, _) => if rs1_value < rs2_value { new_pc = pc.wrapping_add_signed(instruction.immediate_b()) },
            (OPCODE_BRANCH, F3_BGEU, _) => if rs1_value >= rs2_value { new_pc = pc.wrapping_add_signed(instruction.immediate_b()) },

            (OPCODE_LOAD, F3_LB, _) => write_rd(bus.load8(rs1_value.wrapping_add_signed(instruction.immediate_i())) as i8 as u64),
            (OPCODE_LOAD, F3_LH, _) => write_rd(bus.load16(rs1_value.wrapping_add_signed(instruction.immediate_i())) as i16 as u64),
            (OPCODE_LOAD, F3_LW, _) => write_rd(bus.load32(rs1_value.wrapping_add_signed(instruction.immediate_i())) as i32 as u64),
            (OPCODE_LOAD, F3_LD, _) => write_rd(bus.load64(rs1_value.wrapping_add_signed(instruction.immediate_i()))),
            (OPCODE_LOAD, F3_LBU, _) => write_rd(bus.load8(rs1_value.wrapping_add_signed(instruction.immediate_i()))),
            (OPCODE_LOAD, F3_LHU, _) => write_rd(bus.load16(rs1_value.wrapping_add_signed(instruction.immediate_i()))),
            (OPCODE_LOAD, F3_LWU, _) => write_rd(bus.load32(rs1_value.wrapping_add_signed(instruction.immediate_i()))),

            (OPCODE_STORE, F3_SB, _) => bus.store8(rs1_value.wrapping_add_signed(instruction.immediate_i()), rs2_value),
            (OPCODE_STORE, F3_SH, _) => bus.store16(rs1_value.wrapping_add_signed(instruction.immediate_i()), rs2_value),
            (OPCODE_STORE, F3_SW, _) => bus.store32(rs1_value.wrapping_add_signed(instruction.immediate_i()), rs2_value),
            (OPCODE_STORE, F3_SD, _) => bus.store64(rs1_value.wrapping_add_signed(instruction.immediate_i()), rs2_value),

            (OPCODE_MISC_MEM, _, _) => (),

            (OPCODE_SYSTEM, F3_CSRRW | F3_CSRRS | F3_CSRRC | F3_CSRRWI | F3_CSRRSI | F3_CSRRCI, _) => {
                let csr = instruction.csr();
                let result = match instruction.funct3 {
                    F3_CSRRW => self.csr_operation(csr, |_| rs1_value),
                    F3_CSRRS => self.csr_operation(csr, |old_value| old_value | rs1_value),
                    F3_CSRRC => self.csr_operation(csr, |old_value| old_value | old_value & (!rs1_value)),
                    F3_CSRRWI => self.csr_operation(csr, |_| instruction.rs1 as u64),
                    F3_CSRRSI => self.csr_operation(csr, |old_value| old_value | (instruction.rs1 as u64)),
                    F3_CSRRCI => self.csr_operation(csr, |old_value| old_value | old_value & (!(instruction.rs1 as u64))),
                    _ => None
                };
                if let Some(result) = result {
                    write_rd(result);
                } else {
                    self.undefined_instruction(instruction)
                }
            }
            (OPCODE_SYSTEM, F3_ECALL_EBREAK, 0) => if instruction.rs2 == IMM_ECALL {} else if instruction.rs2 == IMM_EBREAK {} else { self.undefined_instruction(instruction) },

            (_, _, _) => self.undefined_instruction(instruction),
        }

        if let Some(rd) = new_rd_value {
            self.write_register(instruction.rd, rd);
        }

        self.pc = new_pc;
        self.cycles += 1;
        self.instructions_retired += 1;

        ()
    }

    fn read_register(&self, index: i32) -> u64 {
        self.registers[index as usize]
    }

    fn write_register(&mut self, index: i32, value: u64) {
        if index > 0 {
            self.registers[index as usize] = value
        }
    }

    fn read_control_register(&self, id: u64) -> Option<u64> {
        match id {
            CSR_CYCLE => Some(self.cycles),
            CSR_INSTRET => Some(self.instructions_retired),
            CSR_TIME => Some(self.machine.start.elapsed().as_millis() as u64),
            _ => None
        }
    }

    fn write_control_register(&self, id: u64, value: u64) -> bool { false }

    fn csr_operation(&mut self, id: u64, operation: impl FnOnce(u64) -> u64) -> Option<u64> {
        let old_value = self.read_control_register(id)?;
        self.write_control_register(id, operation(old_value));
        Some(old_value)
    }

    fn undefined_instruction(&self, instruction: &Instruction) { unimplemented!("Unimplemented instruction: ({}, {}, {})", instruction.opcode, instruction.funct3, instruction.funct7) }
}

fn mulhsu(a: i64, b: u64) -> u64 {
    // based on https://github.com/riscv-software-src/riscv-isa-sim/blob/90aa49f85b589c91754ea224bc2f1492dd99efa3/riscv/arith.h#L40
    // let negate = a < 0;
    // let res = mulhu((if negate { -a } else { a } as u64), b);
    // if negate { !res + (a * b == 0) } else { res }
    1
}

fn mulhu(a: u64, b: u64) -> u64 {
    ((a as u128).wrapping_mul(b as u128) >> 64) as u64
}

fn div_unsigned(a: u64, b: u64) -> (u64, u64) {
    if b == 0 {
        (u64::MAX, a)
    } else {
        (a / b, a % b)
    }
}

fn div_signed(a: u64, b: u64) -> (u64, u64) {
    let a = a as i64;
    let b = b as i64;
    let result =
        if b == 0 {
            (-1, a)
        } else if a == i64::MIN && b == -1 {
            (i64::MIN, 0)
        } else {
            (a / b, a % b)
        };
    (result.0 as u64, result.1 as u64)
}
