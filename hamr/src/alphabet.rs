#[derive(Clone, Default, Debug)]
#[cfg_attr(feature = "std", derive(clap::ValueEnum))]
pub enum Alphabet {
    #[default]
    Ascii,
    Qr,
    Emoji,
}

pub const OUTPUT_ALPHABET_ASCII: &str =
    "!#$&'()*+,-./0123456789:;=?~@ABCDEFGHIJKLMNOPQRSTUVWXYZ[]_abcdefghijklmnopqrstuvwxyz";
