#![doc = include_str!("../README.md")]

#![cfg_attr(not(feature = "std"), no_std)]

pub mod alphabet;

mod decompress;
pub use decompress::decompress;

mod segment_type;

mod huffman;
