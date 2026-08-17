use core::fmt::Write;
use std::borrow::Cow;

use thiserror::Error;
use url::Url;

use crate::{
    alphabet::{Alphabet, SUBALPHABETS},
    huffman::{Lookup, sld::SLD_LIST, tld::TldEncode},
    segment_type::SegmentType,
};

pub enum Segment<'a> {
    Hash(&'a str),
    Path(&'a str),
    Query {
        key: Cow<'a, str>,
        value: Cow<'a, str>,
    },
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

    let query_params = url
        .query_pairs()
        .map(|(key, value)| Segment::Query { key, value });
    path_segments.extend(query_params);

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
                (SegmentType::Hash, Segment::Query { .. }) => {
                    number += 1;
                }
                (SegmentType::Hash, Segment::Path { .. }) => {
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
        let mut subalphabet = None;
        for (i, subalphabet) in SUBALPHABETS.iter().enumerate() {
            if 
        }
    }

    Ok(())
}
