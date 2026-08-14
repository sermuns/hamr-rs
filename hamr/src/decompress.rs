use core::fmt::Write;

use thiserror::Error;

use crate::alphabet::{self, Alphabet};

#[derive(Error, Debug)]
pub enum DecompressionError {
    #[error("{0}")]
    StrToNumError(#[from] alphabet::StrToNumError),
}

/// Decodes and decompresses the payload (writing into `output`) assuming the given alphabet and produces a full link.
pub fn decompress(
    payload: &str,
    alphabet: Alphabet,
    output: impl Write,
) -> Result<(), DecompressionError> {
    let mut number = alphabet.str_to_number(payload)?;

    // version number - currently unused
    let mut version = 0;
    while number & 1 != 0 {
        version += 1;
        number >>= 1;
    }
    number >>= 1;

    // port number
    let has_port = number & 1 != 0;
    number >>= 1;
    let mut port = 0;
    if has_port {
        const U16_MAX_PLUS_ONE: usize = u16::MAX as usize + 1;
        port = number % U16_MAX_PLUS_ONE;
        number /= U16_MAX_PLUS_ONE;
    }

    // TLD
    let tld_decode_result = huffman_decode(number, tld_decode);
    number = tld_decode_result.new_number;
    let tld = tld_decode_result.digit;

    // "www." prefix
    let has_www = number & 1 != 0;
    number >>= 1;

    // protocol / schema
    let is_https = number & 1 != 0;
    number >>= 1;

    // possible "index.html" or "index.php" suffix
    let mut index_suffix = "";
    if number & 1 != 0 {
        number >>= 1;
        if number & 1 != 0 {
            index_suffix = "/index.php";
        } else {
            index_suffix = "/index.html";
        }
    }
    number >>= 1;

    // domain format
    let has_known_sld = number & 1 != 0;
    number >>= 1;
    let mut has_subdomain = false;
    if has_known_sld {
        has_subdomain = number & 1 != 0;
        number >>= 1;
    }

    todo!();

    Ok(())
}
