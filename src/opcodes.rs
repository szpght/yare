#![allow(dead_code)]

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
