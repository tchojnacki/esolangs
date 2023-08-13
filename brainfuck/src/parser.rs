use crate::{
    instruction::{Instruction, Procedure},
    lexer::Token,
};

#[must_use]
#[derive(Debug)]
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
