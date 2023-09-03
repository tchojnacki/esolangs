use crate::{
    ast::{Node, Tree},
    lexer::Token,
};

#[must_use]
#[derive(Debug, PartialEq)]
pub enum ParseError {
    UnexpectedLoopEnd,
    MissingLoopEnd,
}

#[must_use]
#[derive(Clone, Copy)]
enum Context {
    Root,
    InsideLoop,
}

pub fn parse(tokens: impl IntoIterator<Item = Token>) -> Result<Tree, ParseError> {
    parse_proc(&mut tokens.into_iter(), Context::Root)
}

fn parse_proc(
    tokens: &mut impl Iterator<Item = Token>,
    context: Context,
) -> Result<Tree, ParseError> {
    use {Context as C, Node as N, ParseError as E, Token as T};
    let mut result = Vec::new();
    while let Some(token) = tokens.next() {
        result.push(match token {
            T::Right => N::Right,
            T::Left => N::Left,
            T::Increment => N::Increment,
            T::Decrement => N::Decrement,
            T::Output => N::Output,
            T::Input => N::Input,
            T::StartLoop => N::Loop(parse_proc(tokens, C::InsideLoop)?),
            T::EndLoop => {
                return match context {
                    C::InsideLoop => Ok(result.into()),
                    C::Root => Err(E::UnexpectedLoopEnd),
                }
            }
        });
    }
    match context {
        C::InsideLoop => Err(E::MissingLoopEnd),
        C::Root => Ok(result.into()),
    }
}

#[cfg(test)]
mod tests {
    use super::{parse, Node as N, ParseError, Token as T};

    fn assert_parses(input: &[T], expected: &[N]) {
        assert_eq!(parse(input.iter().copied()).as_deref(), Ok(expected))
    }

    #[test]
    fn parses_empty_input() {
        assert_parses(&[], &[])
    }

    #[test]
    fn parses_cat() {
        assert_parses(
            &[T::Input, T::StartLoop, T::Output, T::Input, T::EndLoop],
            &[N::Input, N::Loop(Box::new([N::Output, N::Input]))],
        )
    }

    #[test]
    fn parses_nested_loops() {
        assert_parses(
            &[T::StartLoop, T::StartLoop, T::EndLoop, T::EndLoop],
            &[N::Loop(Box::new([N::Loop(Box::new([]))]))],
        )
    }

    #[test]
    fn errors_on_unexpected_loop_end() {
        assert_eq!(
            parse([T::Increment, T::EndLoop].into_iter()),
            Err(ParseError::UnexpectedLoopEnd)
        )
    }

    #[test]
    fn errors_on_missing_loop_end() {
        assert_eq!(
            parse([T::StartLoop, T::Input, T::Increment].into_iter()),
            Err(ParseError::MissingLoopEnd)
        )
    }
}
