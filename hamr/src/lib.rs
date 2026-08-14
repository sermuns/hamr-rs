#![cfg_attr(not(feature = "std"), no_std)]

pub mod alphabet;

mod decompress;
pub use decompress::decompress;
