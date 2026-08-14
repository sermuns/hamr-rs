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
    // TODO: maybe don't use `usize`?
    pub fn str_to_number(&self, input: &str) -> Result<usize, StrToNumError> {
        let mut number = 0;

        for c in input.chars() {
            let digit = self
                .get_slice()
                .iter()
                .position(|a| *a == c)
                .ok_or_else(|| StrToNumError::InvalidCharacter(c))?;

            number *= self.alphabet_size();
            number += digit + 1;
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
    fn str_to_number_works(
        #[case] alphabet: Alphabet,
        #[case] input: &str,
        #[case] expected: usize,
    ) {
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
