use crate::{
    instruction::{Instruction, Procedure},
    lexer::Token,
};

#[must_use]
#[derive(Debug, PartialEq)]
pub enum ParseError {
    UnexpectedLoopEnd,
    MissingLoopEnd,
}

pub fn parse(tokens: impl IntoIterator<Item = Token>) -> Result<Procedure, ParseError> {
    parse_proc(&mut tokens.into_iter(), Context::Root)
}

#[must_use]
#[derive(Clone, Copy)]
enum Context {
    Root,
    InsideLoop,
}

fn parse_proc(
    tokens: &mut impl Iterator<Item = Token>,
    context: Context,
) -> Result<Procedure, ParseError> {
    use {Context::*, Instruction as I, ParseError::*, Token as T};
    let mut result = Vec::new();
    while let Some(token) = tokens.next() {
        result.push(match token {
            T::Right => I::Right,
            T::Left => I::Left,
            T::Increment => I::Increment,
            T::Decrement => I::Decrement,
            T::Output => I::Output,
            T::Input => I::Input,
            T::StartLoop => I::Loop(parse_proc(tokens, InsideLoop)?),
            T::EndLoop => {
                return match context {
                    InsideLoop => Ok(result.into()),
                    Root => Err(UnexpectedLoopEnd),
                }
            }
        });
    }
    match context {
        InsideLoop => Err(MissingLoopEnd),
        Root => Ok(result.into()),
    }
}

#[cfg(test)]
mod tests {
    use super::{parse, Instruction as I, ParseError, Token as T};

    fn assert_parses(input: &[T], expected: &[I]) {
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
            &[I::Input, I::Loop(Box::new([I::Output, I::Input]))],
        )
    }

    #[test]
    fn parses_nested_loops() {
        assert_parses(
            &[T::StartLoop, T::StartLoop, T::EndLoop, T::EndLoop],
            &[I::Loop(Box::new([I::Loop(Box::new([]))]))],
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
