pub mod domain;
pub mod path;
pub mod sld;
pub mod tld;

pub trait Lookup {
    fn lookup(key: &str) -> Option<&'static str>;
}

pub struct HuffmanDecode {
    pub new_number: u64,
    pub digit: &'static str,
}

pub fn huffman_decode<L: Lookup>(mut number: u64) -> HuffmanDecode {
    let mut sequence = String::new();

    let digit = loop {
        sequence += if number & 1 != 0 { "1" } else { "0" };
        number >>= 1;
        if sequence.len() > 20 {
            panic!("Huffman sequence too long: '{sequence}'");
        }
        if let Some(digit) = L::lookup(&sequence) {
            break digit;
        }
    };

    HuffmanDecode {
        new_number: number,
        digit,
    }
}

pub fn huffman_encode(mut number: u64, sequence: &str) -> u64 {
    for c in sequence.chars().rev() {
        number <<= 1;
        if c == '1' {
            number += 1;
        }
    }
    number
}
