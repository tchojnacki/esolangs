#[must_use]
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Token {
    Right,
    Left,
    Increment,
    Decrement,
    Output,
    Input,
    StartLoop,
    EndLoop,
}

pub fn tokenize(code: impl IntoIterator<Item = char>) -> impl Iterator<Item = Token> {
    use Token::*;
    code.into_iter().filter_map(|c| match c {
        '>' => Some(Right),
        '<' => Some(Left),
        '+' => Some(Increment),
        '-' => Some(Decrement),
        '.' => Some(Output),
        ',' => Some(Input),
        '[' => Some(StartLoop),
        ']' => Some(EndLoop),
        _ => None,
    })
}

#[cfg(test)]
mod tests {
    use super::{
        tokenize,
        Token::{self, *},
    };

    fn assert_tokenizes(input: &'static str, expected: &[Token]) {
        assert_eq!(tokenize(input.chars()).collect::<Vec<_>>(), expected);
    }

    #[test]
    fn tokenizes_all_characters() {
        assert_tokenizes(
            "><+-.,[]",
            &[
                Right, Left, Increment, Decrement, Output, Input, StartLoop, EndLoop,
            ],
        )
    }

    #[test]
    fn tokenizes_invalid_programs() {
        assert_tokenizes(
            "]][[[",
            &[EndLoop, EndLoop, StartLoop, StartLoop, StartLoop],
        )
    }

    #[test]
    fn tokenizes_move() {
        assert_tokenizes(
            ">>[-]<<[->>+<<]",
            &[
                Right, Right, StartLoop, Decrement, EndLoop, Left, Left, StartLoop, Decrement,
                Right, Right, Increment, Left, Left, EndLoop,
            ],
        )
    }

    #[test]
    fn tokenizes_cat() {
        assert_tokenizes(",[.,]", &[Input, StartLoop, Output, Input, EndLoop])
    }

    #[test]
    fn ignores_other_chars() {
        assert_tokenizes(
            "ab, a Z[ 12*3 . 1a :; , ''`]&",
            &[Input, StartLoop, Output, Input, EndLoop],
        )
    }

    #[test]
    fn tokenizes_empty_input() {
        assert_tokenizes("", &[])
    }
}
