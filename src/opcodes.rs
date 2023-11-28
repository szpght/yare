#![allow(dead_code)]

pub const OPCODE_OP_IMM: i32 = 0b0010011;
pub const OPCODE_OP_IMM_32: i32 = 0b0011011;
pub const OPCODE_OP: i32 = 0b0110011;
pub const OPCODE_OP_32: i32 = 0b0111011;
pub const OPCODE_LUI: i32 = 0b0110111;
pub const OPCODE_AUIPC: i32 = 0b0010111;
pub const OPCODE_JAL: i32 = 0b1101111;
pub const OPCODE_JALR: i32 = 0b1100111;
pub const OPCODE_BRANCH: i32 = 0b1100011;
pub const OPCODE_LOAD: i32 = 0b0000011;
pub const OPCODE_STORE: i32 = 0b0100011;
pub const OPCODE_MISC_MEM: i32 = 0b0001111;
pub const OPCODE_SYSTEM: i32 = 0b1110011;

pub const F3_ADD: i32 = 0;
pub const F3_SUB: i32 = 0;
pub const F3_SLL: i32 = 1;
pub const F3_SLT: i32 = 2;
pub const F3_SLTU: i32 = 3;
pub const F3_XOR: i32 = 4;
pub const F3_SRL: i32 = 5;
pub const F3_SRA: i32 = 5;
pub const F3_OR: i32 = 6;
pub const F3_AND: i32 = 7;

pub const F3_MUL: i32 = 0;
pub const F3_MULH: i32 = 1;
pub const F3_MULHSU: i32 = 2;
pub const F3_MULHU: i32 = 3;
pub const F3_DIV: i32 = 4;
pub const F3_DIVU: i32 = 5;
pub const F3_REM: i32 = 6;
pub const F3_REMU: i32 = 7;

pub const F3_MULW: i32 = F3_MUL;
pub const F3_DIVW: i32 = F3_DIV;
pub const F3_DIVUW: i32 = F3_DIVU;
pub const F3_REMW: i32 = F3_REM;
pub const F3_REMUW: i32 = F3_REMU;

pub const F3_BEQ: i32 = 0;
pub const F3_BNE: i32 = 1;
pub const F3_BLT: i32 = 4;
pub const F3_BGE: i32 = 5;
pub const F3_BLTU: i32 = 6;
pub const F3_BGEU: i32 = 7;

pub const F3_LB: i32 = 0;
pub const F3_LH: i32 = 1;
pub const F3_LW: i32 = 2;
pub const F3_LD: i32 = 3;
pub const F3_LBU: i32 = 4;
pub const F3_LHU: i32 = 5;
pub const F3_LWU: i32 = 6;

pub const F3_SB: i32 = 0;
pub const F3_SH: i32 = 1;
pub const F3_SW: i32 = 2;
pub const F3_SD: i32 = 3;

pub const F3_ECALL_EBREAK: i32 = 0;
pub const F3_CSRRW: i32 = 1;
pub const F3_CSRRS: i32 = 2;
pub const F3_CSRRC: i32 = 3;
pub const F3_CSRRWI: i32 = 5;
pub const F3_CSRRSI: i32 = 6;
pub const F3_CSRRCI: i32 = 7;

pub const F7_ADD: i32 = 0;
pub const F7_SLT: i32 = 0;
pub const F7_SLTU: i32 = 0;
pub const F7_AND: i32 = 0;
pub const F7_OR: i32 = 0;
pub const F7_XOR: i32 = 0;
pub const F7_SLL: i32 = 0;
pub const F7_SRL: i32 = 0;
pub const F7_SUB: i32 = 0b0100000;
pub const F7_SRA: i32 = 0b0100000;
pub const F7_MULDIV: i32 = 1;

pub const IMM_ECALL: i32 = 0;
pub const IMM_EBREAK: i32 = 1;

pub const CSR_CYCLE: u64 = 0xC00;
pub const CSR_TIME: u64 = 0xC01;
pub const CSR_INSTRET: u64 = 0xC02;


















macro_rules! declare_instructions {
    ($(($name:ident $opcode:literal))+) => {
        paste::paste! {
            $(
                pub const [<$name:upper>]: u8 = $opcode;
            )*
        }
        
        pub fn mnemonic_to_opcode(mnemonic: &str) -> Option<u8> {
            let mnemonic = mnemonic.to_lowercase();
            match mnemonic.as_str() {
                $(
                    stringify!($name) => Option::Some($opcode),
                )*
                _ => Option::None 
            }
        }
        
        pub fn opcode_to_mnemonic(opcode: u8) -> Option<&'static str> {
            match opcode {
                $(
                    $opcode => Option::Some(stringify!($name)),
                )*
                _ => Option::None 
            }
        }
    }
}

declare_instructions!(
    // math
    (add 0)
    (sub 1)
    (and 2)
    (or 3)
    (xor 4)
    (shl 5)
    (shr 6)
    (sar 7)
    
    // branches
    (ja 8)
    (jr 9)
    (je 10)
    (jne 11)
    (jg 12)
    (jge 13)
    (jgs 14)
    (jges 15)
    
    // loads
    (lbu 16)
    (lwu 17)
    (ldu 18)
    (lbs 19)
    (lws 20)
    (lds 21)
    (lq 22)
    
    // stores
    (sb 23) 
    (sw 24) 
    (sd 25) 
    (sq 26)
    // 
    // // virtual
    (neg 255)
    (not 254)
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mnemonic_to_opcode() {
        assert_eq!(mnemonic_to_opcode("sUb").unwrap(), SUB);
    }

    #[test]
    fn test_opcode_to_mnemonic() {
        assert_eq!(opcode_to_mnemonic(SUB).unwrap(), "sub");
    }
}
