use core::fmt::Write;

use crate::alphabet::Alphabet;

/// Decodes and decompresses the payload (writing into `output`) assuming the given alphabet and produces a full link.
pub fn decompress(payload: &str, alphabet: Alphabet, output: impl Write) {
    let number = alphabet.str_to_number(payload);
}
