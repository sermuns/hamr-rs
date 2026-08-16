#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]

pub mod alphabet;

mod compress;
pub use compress::compress;

mod decompress;
use const_format::formatcp;
pub use decompress::decompress;

mod segment_type;

mod huffman;

pub const HAMR_DOMAIN: &str = "ha.mr";
pub const HAMR_URL_QR: &str = "HTTP://HA.MR/";
pub const HAMR_URL_HASH: &str = formatcp!("http://{}#", HAMR_DOMAIN);
