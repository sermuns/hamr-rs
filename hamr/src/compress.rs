use core::fmt::Write;

use thiserror::Error;
use url::Url;

use crate::{
    alphabet::{Alphabet, SUBALPHABETS},
    huffman::{Lookup, huffman_encode, path::PathEncode, sld::SLD_LIST, tld::TldEncode},
    segment_type::SegmentType,
};

pub enum Segment<'a> {
    Hash(&'a str),
    Path(&'a str),
    Query(&'a str),
}

impl Segment<'_> {
    pub fn value(&self) -> &'_ str {
        match self {
            Self::Hash(v) => v,
            Self::Path(v) => v,
            Self::Query(v) => v,
        }
    }
}

#[derive(Error, Debug)]
pub enum CompressionError {
    #[error("Invalid URL provided")]
    InvalidUrl,
    #[error("Only HTTP/HTTPS links are supported")]
    InvalidScheme,
}

pub fn compress(
    input: &str,
    alphabet: Alphabet,
    mut output: impl Write,
) -> Result<(), CompressionError> {
    let mut number = 1u64;

    // TODO: auto-add 'http://' if needed
    let url = Url::parse(input).map_err(|_| CompressionError::InvalidUrl)?;

    let mut hostname = url.host_str().expect("should have host").to_lowercase();
    let port = url.port_or_known_default().expect("should have port");
    let tld = hostname.rsplit('.').next().expect("should have tld");

    if TldEncode::lookup(tld).is_some() {
        let last_dot_index = hostname.rfind('.').unwrap();
        // BUG: untested
        hostname.truncate(last_dot_index);
    }
    dbg!(&hostname);

    let is_https = match url.scheme() {
        "https" => true,
        "http" => false,
        _ => return Err(CompressionError::InvalidScheme),
    };

    let has_www = hostname.starts_with("www.");
    if has_www {
        hostname.drain(..4);
    }

    let known_sld: &str = SLD_LIST
        .iter()
        .copied()
        .find(|sld| hostname.ends_with(sld))
        .unwrap_or("");
    let subdomain = &hostname[..known_sld.len()];

    // The seperable parts of a path/query are split into segments with
    // their position/role in the link noted. This lets us pick optimal
    // character sets for individual segments and enables us to encode
    // only the transitions between segments. As-is, the system's a bit
    // clumsy, but it works.
    let mut path_segments: Vec<_> = url
        .path_segments()
        .unwrap()
        .filter(|s| !s.is_empty()) // FIXME: necessary?
        .map(|s| Segment::Path(s))
        .collect();

    let (has_index_html, has_index_php) = match *path_segments.last().unwrap() {
        Segment::Path("index.html") => {
            path_segments.pop();
            (true, false)
        }
        Segment::Path("index.php") => {
            path_segments.pop();
            (false, true)
        }
        _ => (false, false),
    };

    if let Some(query) = url.query() {
        // BUG: not sure if correctly splitting '&'
        let query_params = query.split('&').map(|q| Segment::Query(q));
        path_segments.extend(query_params);
    }

    if let Some(hash) = url.fragment() {
        path_segments.push(Segment::Hash(hash));
    }

    // TODO:
    // Normalize path segment encoding

    // Encode path following domain segment-by-segment, using best algorithm for each
    let mut last_segment_type = SegmentType::from(path_segments.last().unwrap());
    let mut query_param_index = 0;

    for (j, segment) in path_segments.iter().enumerate().rev() {
        let first_iteration = j == path_segments.len() - 1;
        if !first_iteration && query_param_index % 2 == 0 {
            // Indicate change of segment type (path -> param -> hash)
            //   First bit indicates that a change is happening,
            //   second bit indicates whether we're skipping straight to the hash.
            number <<= 1;
            match (&last_segment_type, segment) {
                (SegmentType::Hash, Segment::Query(..)) => {
                    number += 1;
                }
                (SegmentType::Hash, Segment::Path(..)) => {
                    number += 1; // Second bit is 1
                    number <<= 1;
                    number += 1;
                }
                _ if last_segment_type != segment.into() => {
                    // Second bit is 0
                    number <<= 1;
                    number += 1;
                }
                _ => (),
            }
            last_segment_type = segment.into();
        }
        if matches!(segment, Segment::Query { .. }) {
            query_param_index += 1;
        }

        // Look for smallest subalphabet that fits this path segment
        let mut subalphabet_index = None;
        for (i, subalphabet) in SUBALPHABETS.iter().enumerate() {
            if segment.value().chars().all(|c| subalphabet.contains(c)) {
                subalphabet_index = Some(i);
            }
        }

        let path_encode_hash = PathEncode::lookup("#").expect("# should be in PathEncode");

        // Compute number after Huffman coding
        let mut huffman_number = if first_iteration {
            number
        } else {
            huffman_encode(number, path_encode_hash)
        };

        for i in (0..segment.value().len()).rev() {
            if &segment.value()[i - 2..i - 1] == "%" {
                let byte = u8::from_str_radix(&segment.value()[i - 1..i + 1], 16).unwrap();
                huffman_number *= u8::MAX as u64;
                huffman_number += byte as u64;
                huffman_number = huffman_encode(huffman_number, path_encode_hash);
                continue;
            }
            let c = &segment.value()[i..i + 1];
            if c != "~" {
                huffman_number = huffman_encode(huffman_number, PathEncode::lookup(c).unwrap());
                continue;
            }
            // HACK:
            // Our Huffman tree is missing the tilde character (whoops!)
            // It's too late to change it now without bumping the version
            // number, and that currently costs 1 bit. Tildes are so rare
            // that it makes more sense to %-encode them instead.
            huffman_number *= u8::MAX as u64;
            huffman_number += 126;
            huffman_number = huffman_encode(huffman_number, path_encode_hash);
        }

        // Encode segment variant as 0
        // (We're adding +1 here to introduce 0 as a special value indicating Huffman)
        huffman_number *= SUBALPHABETS.len() as u64 + 1;

        // If no subalphabet fits this segment, Huffman is the only option.
        // Encoding a character missing from the subalphabet would produce the
        // value 0, which the decoder treats as the end of the segment.
        let Some(subalphabet_index) = subalphabet_index else {
            number = huffman_number;
            continue;
        };

        let subalphabet = SUBALPHABETS[subalphabet_index];
    // Compute number after encoding with chosen subalphabet
    let subalphabetLength = subalphabet.len() + 1;
    let subalphabetNumber = if first_iteration{ number } else { number * subalphabetLength};
    for i = segment.value.length - 1; i >= 0; i--) {
      subalphabetNumber *= subalphabetLength;
      subalphabetNumber += BigInt(subalphabet.indexOf(segment.value[i]) + 1);
    }
    // Encode segment variant as subalphabet index + 1
    subalphabetNumber *= BigInt(subalphabets.length + 1);
    subalphabetNumber += BigInt(subalphabetIndex + 1);
    // Compare candidate numbers, pick smallest one
    if (huffmanNumber < subalphabetNumber) {
      number = huffmanNumber;
    } else {
      number = subalphabetNumber;
    }
    }

    Ok(())
}
