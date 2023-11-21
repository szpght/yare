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
    (ja 16)
    (jr 17)
    (je 18)
    (jne 19)
    (jg 20)
    (jge 21)
    (jgs 22)
    (jges 23)
    
    // loads
    (lbu 48)
    (lwu 49)
    (ldu 50)
    (lbs 51)
    (lws 52)
    (lds 53)
    (lq 54)
    
    // stores
    (sb 55) 
    (sw 56) 
    (sd 57) 
    (sq 58)
    // 
    // // virtual
    // (neg 255)
    // (not 254)
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
