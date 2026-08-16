use core::fmt::Write;

use thiserror::Error;
use url::Url;

use crate::{
    alphabet::Alphabet,
    huffman::{Lookup, sld::SLD_LIST, tld::TldEncode},
};

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
    let number = 1u64;

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
    dbg!(hostname);

    let is_https = match url.scheme() {
        "https" => true,
        "http" => false,
        _ => return Err(CompressionError::InvalidScheme),
    };

    let has_www = hostname.starts_with("www.");
    if has_www {
        hostname.drain(..4);
    }

    let known_sld = SLD_LIST.iter().find(todo!());

    Ok(())
}
