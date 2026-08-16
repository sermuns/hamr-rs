use core::fmt::Write;

use thiserror::Error;

use crate::{
    alphabet::{self, Alphabet, SUBALPHABETS},
    huffman::{
        HuffmanDecode, domain::DomainDecode, huffman_decode, path::PathDecode, sld::SldDecode,
        tld::TldDecode,
    },
    segment_type::SegmentType,
};

#[derive(Error, Debug)]
pub enum DecompressionError {
    #[error("StrToNumError: {0}")]
    StrToNumError(#[from] alphabet::StrToNumError),
    #[error("Fmt error: {0}")]
    Fmt(#[from] core::fmt::Error),
}

/// Decodes and decompresses the payload (writing into `output`) assuming the given alphabet and produces a full link.
pub fn decompress(
    payload: &str,
    alphabet: Alphabet,
    mut output: impl Write,
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
    let HuffmanDecode { new_number, digit } = huffman_decode::<TldDecode>(number);
    number = new_number;
    let tld = digit;

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

    let mut domain = String::new();
    let mut subdomain = String::new();
    let mut path = String::new();

    if has_known_sld {
        let HuffmanDecode { new_number, digit } = huffman_decode::<SldDecode>(number);
        number = new_number;
        domain = digit.to_string();
        if has_subdomain {
            while number > 1 {
                let HuffmanDecode { new_number, digit } = huffman_decode::<DomainDecode>(number);
                number = new_number;
                if digit == "END" {
                    break;
                }
                subdomain += digit;
            }
        }
    } else {
        while number > 1 {
            let HuffmanDecode { new_number, digit } = huffman_decode::<DomainDecode>(number);
            number = new_number;
            if digit == "END" {
                break;
            }
            domain += digit;
        }
    }

    let mut current_segment_type =
        SegmentType::from_repr(number % 3).expect("segment type index should be < 3");
    number /= 3;

    let mut query_param_index = 0;

    while number > 1 {
        match current_segment_type {
            SegmentType::Path => {
                path += "/";
            }
            SegmentType::Hash => {
                path += "#";
            }
            SegmentType::Query => {
                if query_param_index % 2 != 0 {
                    path += "=";
                } else if query_param_index == 0 {
                    path += "?";
                } else {
                    path += "&";
                }
                query_param_index += 1;
            }
        }

        let variant = number % (SUBALPHABETS.len() + 1);
        number /= SUBALPHABETS.len() + 1;

        // Variant 0 is Huffman code, rest are subalphabets
        if variant == 0 {
            while number > 1 {
                let HuffmanDecode { new_number, digit } = huffman_decode::<PathDecode>(number);
                number = new_number;
                if digit == "#" && !matches!(current_segment_type, SegmentType::Hash) {
                    break;
                }
                path += digit;
                if digit == "%" {
                    let byte = number % 256;
                    path += &format!("{:02X}", byte);
                    number /= 256;
                }
            }
        } else {
            let subalphabet = SUBALPHABETS[variant - 1];
            let subalphabet_length = subalphabet.len() + 1;
            while number > 1 {
                let index = number % subalphabet_length;
                number /= subalphabet_length;
                if index == 0 {
                    break;
                }
                path.push(subalphabet.chars().nth(index - 1).unwrap());
            }
        }

        // Handle changing between path segment types, unless we're in the
        // middle of decoding a query parameter key/value pair, in which
        // case switching to the hash value doesn't make sense.
        if query_param_index % 2 != 0 {
            continue;
        }
        if number & 1 != 0 {
            match current_segment_type {
                SegmentType::Path => {
                    number >>= 1;
                    if number & 1 != 0 {
                        // Skipping to hash?
                        current_segment_type = SegmentType::Hash;
                    } else {
                        current_segment_type = SegmentType::Query
                    }
                }
                _ => {
                    current_segment_type = SegmentType::Hash;
                }
            }
        }
        number >>= 1;
    }

    let path_split_index = path.find(['?', '#']);
    let path_before_query = path_split_index
        .map(|index| &path[..index])
        .unwrap_or(&path);
    let path_from_query = path_split_index.map(|index| &path[index..]).unwrap_or("");

    output.write_str(if is_https { "https://" } else { "http://" })?;
    if has_www {
        output.write_str("www.")?;
    }
    output.write_str(&subdomain)?;
    output.write_str(&domain)?;
    if !tld.is_empty() {
        output.write_str(".")?;
        output.write_str(tld)?;
    }
    if has_port {
        write!(output, ":{}", port)?;
    }
    output.write_str(path_before_query)?;
    output.write_str(index_suffix)?;
    output.write_str(path_from_query)?;

    Ok(())
}
