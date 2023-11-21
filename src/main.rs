mod opcodes;
mod bus;
mod assembler;
mod instruction;

use std::fs::File;
use std::io::{self, BufReader, Read};
use lexpr::Value;
use crate::assembler::assemble;
use crate::bus::Bus;
use crate::instruction::Instruction;
use crate::opcodes::{mnemonic_to_opcode, opcode_to_mnemonic, SUB};

/// Instruction format:
/// 8b - opcode
/// 8b - 1st input register
/// 8b - 2nd input register
/// 8b - output register
/// 32b - padding
/// 64b - immediate

#[derive(Default, Debug)]
enum Equality {
    #[default]
    Equal,
    Less,
    Greater,
}

#[derive(Debug)]
struct Cpu {
    pub registers: [u64; 32],
    pub pc: u64,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu { registers: [0; 32], pc: 0 }
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
        let data = bus.load64(self.pc);
        let immediate = bus.load64(self.pc + 8);
        
        Instruction::decode(data, immediate)
    }
    
    pub fn execute(&mut self, instruction: &Instruction, bus: &mut Bus) {
        let pc = self.pc;
        let value_a = self.read_register(instruction.reg_a, instruction.immediate);
        let value_b = self.read_register(instruction.reg_b, instruction.immediate);
        let value_target = self.read_register(instruction.reg_target, instruction.immediate);
        let reg_a_signed = value_a as i64;
        let reg_b_signed = value_b as i64;
        let shift = value_b % 32;
        let next_instruction_address = self.pc + Instruction::SIZE;
        let mut new_pc = self.pc + Instruction::SIZE;

        {
            let mut write_target = |x| self.write_register(instruction.reg_target, x);
            let mut jump = |x| {
                write_target(next_instruction_address);
                new_pc = x; };
            let mut jump_relative = |x| jump(pc.wrapping_add(x));

            match instruction.opcode {
                // math
                opcodes::ADD => write_target(value_a.wrapping_add(value_b)),
                opcodes::SUB => write_target(value_a.wrapping_sub(value_b)),
                opcodes::AND => write_target(value_a & value_b),
                //opcodes::NEG => write_target((!value_a).wrapping_add(1)),
                //opcodes::NOT => write_target(!value_a),
                opcodes::OR => write_target(value_a | value_b),
                opcodes::XOR => write_target(value_a ^ value_b),
                opcodes::SHL => write_target(value_a << shift),
                opcodes::SHR => write_target(value_a >> shift),
                opcodes::SAR => write_target(((reg_a_signed) >> shift) as u64),
                
                // branches
                opcodes::JA => jump(value_a),
                opcodes::JR => jump_relative(value_a),
                opcodes::JE => if value_a == value_b { jump_relative(instruction.immediate) },
                opcodes::JNE => if value_a != value_b { jump_relative(instruction.immediate) },
                opcodes::JG => if value_a > value_b { jump_relative(instruction.immediate) },
                opcodes::JGE => if value_a >= value_b { jump_relative(instruction.immediate) },
                opcodes::JGS => if reg_a_signed > reg_b_signed { jump_relative(instruction.immediate) },
                opcodes::JGES => if reg_a_signed >= reg_b_signed { jump_relative(instruction.immediate) },
                
                // load
                opcodes::LBU => write_target(bus.load8(value_a.wrapping_add_signed(instruction.offset as i64))),
                opcodes::LWU => write_target(bus.load16(value_a.wrapping_add_signed(instruction.offset as i64))),
                opcodes::LDU => write_target(bus.load32(value_a.wrapping_add_signed(instruction.offset as i64))),
                opcodes::LQ => write_target(bus.load64(value_a.wrapping_add_signed(instruction.offset as i64))),
                opcodes::LBS => write_target(bus.load8(value_a.wrapping_add_signed(instruction.offset as i64)) as i8 as u64),
                opcodes::LWS => write_target(bus.load16(value_a.wrapping_add_signed(instruction.offset as i64)) as i16 as u64),
                opcodes::LDS => write_target(bus.load32(value_a.wrapping_add_signed(instruction.offset as i64)) as i32 as u64),
                
                // store
                opcodes::SB => bus.store8(value_target.wrapping_add_signed(instruction.offset as i64), value_a),
                opcodes::SW => bus.store16(value_target.wrapping_add_signed(instruction.offset as i64), value_a),
                opcodes::SD => bus.store32(value_target.wrapping_add_signed(instruction.offset as i64), value_a),
                opcodes::SQ => bus.store64(value_target.wrapping_add_signed(instruction.offset as i64), value_a),

                _ => self.undefined_instruction(),
            }
        }

        self.pc = new_pc;

        ()
    }
    
    fn read_register(&self, index: u8, immediate: u64) -> u64 {
        if index == 0 { immediate } else { self.registers[index as usize] }
    }

    fn write_register(&mut self, index: u8, value: u64) {
        if index > 0 {
            self.registers[index as usize] = value   
        }
    }
    
    fn undefined_instruction(&self) {}
}

fn main() -> io::Result<()> {
    println!("{}", opcode_to_mnemonic(SUB).unwrap());
    println!("{}", mnemonic_to_opcode("SUB").unwrap());
    let file = File::open("foo.scm")?;
    let mut reader = BufReader::new(file);
    let mut code = String::new();
    reader.read_to_string(&mut code)?;
    println!("Wczytany kod: \n{}", code);
    let xd = assemble(code.as_str());
    println!("{:?}", xd);
    
    let mut cpu = Cpu::new();
    let mut bus = Bus::new();
    // cpu.execute(&Instruction { opcode: opcodes::MOV, reg_a: 0, reg_b: 0, reg_target: 1, immediate: 666 }, &mut bus);
    // cpu.execute(&Instruction { opcode: opcodes::MOV, reg_a: 0, reg_b: 0, reg_target: 2, immediate: 42 }, &mut bus);
    cpu.execute(&Instruction { opcode: opcodes::ADD, reg_a: 1, reg_b: 2, reg_target: 3, immediate: 0, offset: 0 }, &mut bus);
    cpu.execute(&Instruction { opcode: opcodes::SQ, reg_a: 1, reg_b: 0, reg_target: 0, immediate: 2, offset: 0 }, &mut bus);
    cpu.execute(&Instruction { opcode: opcodes::SQ, reg_a: 2, reg_b: 0, reg_target: 0, immediate: 10, offset: 0 }, &mut bus);
    cpu.execute(&Instruction { opcode: opcodes::SQ, reg_a: 3, reg_b: 0, reg_target: 0, immediate: 18, offset: 0 }, &mut bus);
    cpu.execute(&Instruction { opcode: opcodes::LQ, reg_a: 0, reg_b: 0, reg_target: 4, immediate: 2, offset: 0 }, &mut bus);
    cpu.execute(&Instruction { opcode: opcodes::LQ, reg_a: 0, reg_b: 0, reg_target: 5, immediate: 10, offset: 0 }, &mut bus);
    cpu.execute(&Instruction { opcode: opcodes::LQ, reg_a: 0, reg_b: 0, reg_target: 6, immediate: 18, offset: 0 }, &mut bus);

    let mut cpussy = Cpu::new();
    let mut bussy = Bus::new();

    // bussy.store_instruction(0, Instruction { opcode: opcodes::MOV, reg_a: 0, reg_b: 0, reg_target: 1, immediate: 666 }.encode());
    // bussy.store_instruction(16, Instruction { opcode: opcodes::MOV, reg_a: 0, reg_b: 0, reg_target: 2, immediate: 42 }.encode());
    bussy.store_instruction(32, Instruction { opcode: opcodes::ADD, reg_a: 1, reg_b: 2, reg_target: 3, immediate: 0, offset: 0 }.encode());
    bussy.store_instruction(48, Instruction { opcode: opcodes::SQ, reg_a: 1, reg_b: 0, reg_target: 0, immediate: 2, offset: 0 }.encode());
    bussy.store_instruction(64, Instruction { opcode: opcodes::SQ, reg_a: 2, reg_b: 0, reg_target: 0, immediate: 10, offset: 0 }.encode());
    bussy.store_instruction(80, Instruction { opcode: opcodes::SQ, reg_a: 3, reg_b: 0, reg_target: 0, immediate: 18, offset: 0 }.encode());
    bussy.store_instruction(96, Instruction { opcode: opcodes::LQ, reg_a: 0, reg_b: 0, reg_target: 4, immediate: 2, offset: 0 }.encode());
    bussy.store_instruction(112, Instruction { opcode: opcodes::LQ, reg_a: 0, reg_b: 0, reg_target: 5, immediate: 10, offset: 0 }.encode());
    bussy.store_instruction(128, Instruction { opcode: opcodes::LQ, reg_a: 0, reg_b: 0, reg_target: 6, immediate: 18, offset: 0 }.encode());
    
    cpussy.step(&mut bussy);
    cpussy.step(&mut bussy);
    cpussy.step(&mut bussy);
    cpussy.step(&mut bussy);
    cpussy.step(&mut bussy);
    cpussy.step(&mut bussy);
    cpussy.step(&mut bussy);
    cpussy.step(&mut bussy);
    cpussy.step(&mut bussy);
    
    println!("{}, {}, {}", cpussy.registers[4], cpussy.registers[5], cpussy.registers[6]);

    Ok(())
}

fn handle_single_instruction(a: &Value) {
    println!("{}", a);
}