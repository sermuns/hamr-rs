use std::io::stdout;

use clap::{Parser, Subcommand};
use color_eyre::eyre::OptionExt;
use hamr::alphabet::{Alphabet, OUTPUT_ALPHABET_ASCII};

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
    Compress { input: String, alphabet: Alphabet },
}

fn main() -> color_eyre::Result<()> {
    let Cli { command } = Cli::parse();

    match command {
        Command::Decompress { link } => {
            let lowercase_link = link.to_lowercase();

            const HAMR_DOMAIN: &str = "ha.mr";
            let payload_start_byte_index = lowercase_link
                .find(HAMR_DOMAIN)
                .map(|i| i + HAMR_DOMAIN.len())
                .ok_or_eyre("Not a valid compressed link!")?;
            let mut payload = &link[payload_start_byte_index..];

            let is_qr_code = link.starts_with('/');
            payload = &payload[1..];

            let use_emoji = payload.chars().any(|c| !OUTPUT_ALPHABET_ASCII.contains(&c));

            let mut decompressed = String::new();
            if is_qr_code {
                hamr::decompress(payload, Alphabet::Qr, &mut decompressed);
            }

            Ok(())
        }
        Command::Compress { .. } => todo!(),
    }
}
