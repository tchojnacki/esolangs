use crate::frontend::token::{Token, TokenKind};

pub(crate) fn tokenize(source: &str) -> impl Iterator<Item = Token> + '_ {
    use TokenKind as TK;
    source.chars().enumerate().filter_map(|(pos, c)| {
        match c {
            '>' => Some(TK::Right),
            '<' => Some(TK::Left),
            '+' => Some(TK::Increment),
            '-' => Some(TK::Decrement),
            '.' => Some(TK::Output),
            ',' => Some(TK::Input),
            '[' => Some(TK::StartLoop),
            ']' => Some(TK::EndLoop),
            '#' => Some(TK::Debug),
            _ => None,
        }
        .map(|kind| Token { kind, pos })
    })
}

#[cfg(test)]
mod tests {
    use super::{tokenize, TokenKind as TK};

    fn assert_tokenizes(input: &'static str, expected: &[TK]) {
        assert_eq!(
            tokenize(input).map(|t| t.kind).collect::<Vec<_>>(),
            expected
        )
    }

    #[test]
    fn tokenizes_all_characters() {
        assert_tokenizes(
            "><+-.,[]#",
            &[
                TK::Right,
                TK::Left,
                TK::Increment,
                TK::Decrement,
                TK::Output,
                TK::Input,
                TK::StartLoop,
                TK::EndLoop,
                TK::Debug,
            ],
        )
    }

    #[test]
    fn tokenizes_invalid_programs() {
        assert_tokenizes(
            "]][[[",
            &[
                TK::EndLoop,
                TK::EndLoop,
                TK::StartLoop,
                TK::StartLoop,
                TK::StartLoop,
            ],
        )
    }

    #[test]
    fn tokenizes_move() {
        assert_tokenizes(
            ">>[-]<<[->>+<<]",
            &[
                TK::Right,
                TK::Right,
                TK::StartLoop,
                TK::Decrement,
                TK::EndLoop,
                TK::Left,
                TK::Left,
                TK::StartLoop,
                TK::Decrement,
                TK::Right,
                TK::Right,
                TK::Increment,
                TK::Left,
                TK::Left,
                TK::EndLoop,
            ],
        )
    }

    #[test]
    fn tokenizes_cat() {
        assert_tokenizes(
            ",[.,]",
            &[TK::Input, TK::StartLoop, TK::Output, TK::Input, TK::EndLoop],
        )
    }

    #[test]
    fn ignores_other_chars() {
        assert_tokenizes(
            "ab, a Z[ 12*3 . 1a :; , ''`]&",
            &[TK::Input, TK::StartLoop, TK::Output, TK::Input, TK::EndLoop],
        )
    }

    #[test]
    fn tokenizes_empty_input() {
        assert_tokenizes("", &[])
    }

    #[test]
    fn returns_correct_positions() {
        assert_eq!(
            tokenize("  [abc+]-").map(|t| t.pos).collect::<Vec<_>>(),
            &[2, 6, 7, 8]
        )
    }
}
