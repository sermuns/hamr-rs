use std::io::stdout;

use clap::{Parser, Subcommand};
use color_eyre::eyre::OptionExt;
use hamr::{
    HAMR_DOMAIN, HAMR_URL_HASH, HAMR_URL_QR,
    alphabet::{Alphabet, OUTPUT_ALPHABET_ASCII},
    compress,
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    #[command(arg_required_else_help = true)]
    Decompress { link: String },
    #[command(arg_required_else_help = true)]
    Compress { alphabet: Alphabet, link: String },
}

fn main() -> color_eyre::Result<()> {
    let Cli { command } = Cli::parse();

    match command {
        Command::Decompress { link } => {
            let lowercase_link = link.to_lowercase();

            let payload_start_byte_index = lowercase_link
                .find(HAMR_DOMAIN)
                .map(|i| i + HAMR_DOMAIN.len())
                .ok_or_eyre("Not a valid compressed link!")?;
            let mut payload = &link[payload_start_byte_index..];

            let is_qr_code = link.starts_with('/');
            payload = &payload[1..];

            let use_emoji = payload.chars().any(|c| !OUTPUT_ALPHABET_ASCII.contains(&c));

            let mut decompressed = String::new();
            hamr::decompress(
                payload,
                if is_qr_code {
                    Alphabet::Qr
                } else if use_emoji {
                    Alphabet::Emoji
                } else {
                    Alphabet::Ascii
                },
                &mut decompressed,
            )?;
            println!("{}", decompressed);

            Ok(())
        }
        Command::Compress { alphabet, link } => {
            let mut compressed = String::new();

            match alphabet {
                Alphabet::Qr => {
                    compressed += HAMR_URL_QR;
                    todo!();
                }
                Alphabet::Ascii | Alphabet::Emoji => {
                    compressed += HAMR_URL_HASH;
                    compress(&link, alphabet, &mut compressed)?;
                }
            }

            println!("{}", compressed);

            Ok(())
        }
    }
}
