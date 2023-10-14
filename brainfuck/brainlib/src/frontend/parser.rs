use crate::frontend::{
    ast::{Node, Tree},
    token::{Token, TokenKind},
};

#[must_use]
#[derive(Debug, PartialEq)]
pub enum ParseError {
    UnexpectedLoopEnd(usize),
    MissingLoopEnd,
}

#[must_use]
#[derive(Clone, Copy)]
enum Context {
    Root,
    InsideLoop,
}

pub(crate) fn parse(tokens: impl IntoIterator<Item = Token>) -> Result<Tree, ParseError> {
    parse_proc(&mut tokens.into_iter(), Context::Root)
}

fn parse_proc(
    tokens: &mut impl Iterator<Item = Token>,
    context: Context,
) -> Result<Tree, ParseError> {
    use self::{Context as C, Node as N, ParseError as E, TokenKind as TK};
    let mut result = Vec::new();
    while let Some(token) = tokens.next() {
        result.push(match token.kind {
            TK::Right => N::Right,
            TK::Left => N::Left,
            TK::Increment => N::Increment,
            TK::Decrement => N::Decrement,
            TK::Output => N::Output,
            TK::Input => N::Input,
            TK::StartLoop => N::Loop(parse_proc(tokens, C::InsideLoop)?),
            TK::EndLoop =>
                return match context {
                    C::InsideLoop => Ok(result.into()),
                    C::Root => Err(E::UnexpectedLoopEnd(token.pos)),
                },
            TK::Debug => N::Breakpoint(token.pos),
        });
    }
    match context {
        C::InsideLoop => Err(E::MissingLoopEnd),
        C::Root => Ok(result.into()),
    }
}

#[cfg(test)]
mod tests {
    use super::{parse, Node as N, ParseError, Token, TokenKind as TK};

    fn assert_parses(input: &[TK], expected: &[N]) {
        assert_eq!(
            parse(
                input
                    .iter()
                    .enumerate()
                    .map(|(pos, &kind)| Token { kind, pos })
            )
            .as_deref(),
            Ok(expected)
        )
    }

    #[test]
    fn parses_empty_input() {
        assert_parses(&[], &[])
    }

    #[test]
    fn parses_cat() {
        assert_parses(
            &[TK::Input, TK::StartLoop, TK::Output, TK::Input, TK::EndLoop],
            &[N::Input, N::Loop(Box::new([N::Output, N::Input]))],
        )
    }

    #[test]
    fn parses_nested_loops() {
        assert_parses(
            &[TK::StartLoop, TK::StartLoop, TK::EndLoop, TK::EndLoop],
            &[N::Loop(Box::new([N::Loop(Box::new([]))]))],
        )
    }

    #[test]
    fn errors_on_unexpected_loop_end() {
        assert_eq!(
            parse(
                [
                    Token {
                        kind: TK::Increment,
                        pos: 0
                    },
                    Token {
                        kind: TK::EndLoop,
                        pos: 1
                    }
                ]
                .into_iter()
            ),
            Err(ParseError::UnexpectedLoopEnd(1))
        )
    }

    #[test]
    fn errors_on_missing_loop_end() {
        assert_eq!(
            parse(
                [
                    Token {
                        kind: TK::StartLoop,
                        pos: 0
                    },
                    Token {
                        kind: TK::Input,
                        pos: 2
                    },
                    Token {
                        kind: TK::Increment,
                        pos: 3
                    }
                ]
                .into_iter()
            ),
            Err(ParseError::MissingLoopEnd)
        )
    }
}
