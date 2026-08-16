use thiserror::Error;

#[derive(Clone, Default, Debug)]
#[cfg_attr(feature = "std", derive(clap::ValueEnum))]
pub enum Alphabet {
    #[default]
    Ascii,
    Qr,
    Emoji,
}

#[derive(Error, Debug, PartialEq)]
pub enum StrToNumError {
    #[error("encountered `{0}`")]
    InvalidCharacter(char),
}

impl Alphabet {
    pub fn str_to_number(&self, input: &str) -> Result<u64, StrToNumError> {
        let mut number = 0u64;

        for c in input.chars() {
            let digit = self
                .get_slice()
                .iter()
                .position(|a| *a == c)
                .ok_or_else(|| StrToNumError::InvalidCharacter(c))?;

            // BUG: no idea if it should be wrapping..
            number = number.wrapping_mul(self.alphabet_size() as u64);
            number += digit as u64 + 1;
        }

        Ok(number)
    }

    pub const fn get_slice(&self) -> &'static [char] {
        match self {
            Self::Ascii => OUTPUT_ALPHABET_ASCII,
            Self::Qr => OUTPUT_ALPHABET_QR,
            Self::Emoji => OUTPUT_ALPHABET_EMOJI,
        }
    }

    pub const fn alphabet_size(&self) -> usize {
        self.get_slice().len()
    }
}

pub const OUTPUT_ALPHABET_ASCII: &[char] = &[
    '!', '#', '$', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/', '0', '1', '2', '3', '4', '5',
    '6', '7', '8', '9', ':', ';', '=', '?', '~', '@', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I',
    'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '[', ']',
    '_', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
    's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
];
pub const OUTPUT_ALPHABET_QR: &[char] = &[
    '$', '*', '+', '-', '.', '/', ':', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B',
    'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U',
    'V', 'W', 'X', 'Y', 'Z',
];
// TODO:
pub const OUTPUT_ALPHABET_EMOJI: &[char] = &[];

// Growing subcategories of the full URL alphabet
// Each also includes the hyphen and underscore as common separators
pub const SUBALPHABETS: &[&str] = &[
    // Numbers only
    "0123456789-_",
    // Uppercase only
    "ABCDEFGHIJKLMNOPQRSTUVWXYZ-_",
    // Lowercase only
    "abcdefghijklmnopqrstuvwxyz-_",
    // Uppercase and numbers
    "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-_",
    // Lowercase and numbers
    "abcdefghijklmnopqrstuvwxyz0123456789-_",
    // Uppercase and lowercase
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz-_",
    // Upercase, lowercase and numbers (base64)
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_",
    // Full alphabet without slash character
    "!#$&'()*+,-.0123456789:;=?~@ABCDEFGHIJKLMNOPQRSTUVWXYZ[]_abcdefghijklmnopqrstuvwxyz%",
];

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(Alphabet::Ascii, "!", 1)]
    #[case(Alphabet::Ascii, "A", 30)]
    #[case(Alphabet::Ascii, "!!", 85)]
    #[case(Alphabet::Ascii, "!!!", 7141)]
    #[case(Alphabet::Qr, "$", 1)]
    #[case(Alphabet::Qr, "$$", 44)]
    fn str_to_number_works(#[case] alphabet: Alphabet, #[case] input: &str, #[case] expected: u64) {
        let result = alphabet.str_to_number(input).unwrap();
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(Alphabet::Qr, "!", StrToNumError::InvalidCharacter('!'))]
    fn str_to_number_errors(
        #[case] alphabet: Alphabet,
        #[case] input: &str,
        #[case] expected: StrToNumError,
    ) {
        let err = alphabet.str_to_number(input).unwrap_err();
        assert_eq!(err, expected);
    }
}
