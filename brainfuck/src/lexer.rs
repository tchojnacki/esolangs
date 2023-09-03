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
    use Token as T;
    code.into_iter().filter_map(|c| match c {
        '>' => Some(T::Right),
        '<' => Some(T::Left),
        '+' => Some(T::Increment),
        '-' => Some(T::Decrement),
        '.' => Some(T::Output),
        ',' => Some(T::Input),
        '[' => Some(T::StartLoop),
        ']' => Some(T::EndLoop),
        _ => None,
    })
}

#[cfg(test)]
mod tests {
    use super::{tokenize, Token as T};

    fn assert_tokenizes(input: &'static str, expected: &[T]) {
        assert_eq!(tokenize(input.chars()).collect::<Vec<_>>(), expected);
    }

    #[test]
    fn tokenizes_all_characters() {
        assert_tokenizes(
            "><+-.,[]",
            &[
                T::Right,
                T::Left,
                T::Increment,
                T::Decrement,
                T::Output,
                T::Input,
                T::StartLoop,
                T::EndLoop,
            ],
        )
    }

    #[test]
    fn tokenizes_invalid_programs() {
        assert_tokenizes(
            "]][[[",
            &[
                T::EndLoop,
                T::EndLoop,
                T::StartLoop,
                T::StartLoop,
                T::StartLoop,
            ],
        )
    }

    #[test]
    fn tokenizes_move() {
        assert_tokenizes(
            ">>[-]<<[->>+<<]",
            &[
                T::Right,
                T::Right,
                T::StartLoop,
                T::Decrement,
                T::EndLoop,
                T::Left,
                T::Left,
                T::StartLoop,
                T::Decrement,
                T::Right,
                T::Right,
                T::Increment,
                T::Left,
                T::Left,
                T::EndLoop,
            ],
        )
    }

    #[test]
    fn tokenizes_cat() {
        assert_tokenizes(
            ",[.,]",
            &[T::Input, T::StartLoop, T::Output, T::Input, T::EndLoop],
        )
    }

    #[test]
    fn ignores_other_chars() {
        assert_tokenizes(
            "ab, a Z[ 12*3 . 1a :; , ''`]&",
            &[T::Input, T::StartLoop, T::Output, T::Input, T::EndLoop],
        )
    }

    #[test]
    fn tokenizes_empty_input() {
        assert_tokenizes("", &[])
    }
}
