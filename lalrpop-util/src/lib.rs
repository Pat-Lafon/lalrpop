#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use alloc::{string::String, vec::Vec};
use core::fmt;
#[cfg(feature = "std")]
use std::error::Error;

#[cfg(feature = "lexer")]
pub mod lexer;
pub mod state_machine;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ParseError<Loc, Token, LError, PError> {
    /// Errors captured by the parser from the lexer.
    LexerError { error: LError },

    /// Generated by the parser when it encounters an EOF it did not expect.
    UnrecognizedEof {
        /// The end of the final token
        location: Loc,

        /// The set of expected tokens: these names are taken from the
        /// grammar and hence may not necessarily be suitable for
        /// presenting to the user.
        expected: Vec<String>,
    },

    /// Generated by the parser when it encounters a token it did not expect.
    UnrecognizedToken {
        /// The unexpected token of type `T` with a span given by the two `L` values.
        token: (Loc, Token, Loc),

        /// The set of expected tokens: these names are taken from the
        /// grammar and hence may not necessarily be suitable for
        /// presenting to the user.
        expected: Vec<String>,
    },

    /// Generated by the parser when it encounters additional, unexpected tokens.
    ExtraToken { token: (Loc, Token, Loc) },

    /// Custom error type.
    User { error: PError },
}

impl<Loc, Token, LError, PError> ParseError<Loc, Token, LError, PError> {
    fn map_intern<LL, TT, LEE, PEE>(
        self,
        mut loc_op: impl FnMut(Loc) -> LL,
        tok_op: impl FnOnce(Token) -> TT,
        lerr_op: impl FnOnce(LError) -> LEE,
        err_op: impl FnOnce(PError) -> PEE,
    ) -> ParseError<LL, TT, LEE, PEE> {
        let maptok = |(s, t, e): (Loc, Token, Loc)| (loc_op(s), tok_op(t), loc_op(e));
        match self {
            ParseError::InvalidToken { location } => ParseError::InvalidToken {
                location: loc_op(location),
            },
            ParseError::UnrecognizedEof { location, expected } => ParseError::UnrecognizedEof {
                location: loc_op(location),
                expected,
            },
            ParseError::UnrecognizedToken { token, expected } => ParseError::UnrecognizedToken {
                token: maptok(token),
                expected,
            },
            ParseError::ExtraToken { token } => ParseError::ExtraToken {
                token: maptok(token),
            },
            ParseError::User { error } => ParseError::User {
                error: err_op(error),
            },
        }
    }

    pub fn map_location<LL>(
        self,
        op: impl FnMut(Loc) -> LL,
    ) -> ParseError<LL, Token, LError, PError> {
        self.map_intern(op, |x| x, |x| x, |x| x)
    }

    pub fn map_token<TT>(
        self,
        op: impl FnOnce(Token) -> TT,
    ) -> ParseError<Loc, TT, LError, PError> {
        self.map_intern(|x| x, op, |x| x, |x| x)
    }

    pub fn map_error<LEE>(
        self,
        op: impl FnOnce(LError) -> LEE,
    ) -> ParseError<Loc, Token, LError, PError> {
        self.map_intern(|x| x, |x| x, op, |x| x)
    }

    pub fn map_error<PEE>(
        self,
        op: impl FnOnce(PError) -> PEE,
    ) -> ParseError<Loc, Token, LError, PError> {
        self.map_intern(|x| x, |x| x, |x| x, op)
    }
}

/// Format a list of expected tokens.
fn fmt_expected(f: &mut fmt::Formatter<'_>, expected: &[String]) -> fmt::Result {
    if !expected.is_empty() {
        writeln!(f)?;
        for (i, e) in expected.iter().enumerate() {
            let sep = match i {
                0 => "Expected one of",
                _ if i < expected.len() - 1 => ",",
                // Last expected message to be written
                _ => " or",
            };
            write!(f, "{} {}", sep, e)?;
        }
    }
    Ok(())
}

impl<Loc, Token, LError, PError> fmt::Display for ParseError<Loc, Token, LError, PError>
where
    Loc: fmt::Display,
    Token: fmt::Display,
    PError: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use self::ParseError::*;
        match *self {
            User { ref error } => write!(f, "{}", error),
            LexerError { ref error } => write!(f, "{}", error),
            /*             InvalidToken { ref location } => write!(f, "Invalid token at {}", location), */
            UnrecognizedEof {
                ref location,
                ref expected,
            } => {
                write!(f, "Unrecognized EOF found at {}", location)?;
                fmt_expected(f, expected)
            }
            UnrecognizedToken {
                token: (ref start, ref token, ref end),
                ref expected,
            } => {
                write!(
                    f,
                    "Unrecognized token `{}` found at {}:{}",
                    token, start, end
                )?;
                fmt_expected(f, expected)
            }
            ExtraToken {
                token: (ref start, ref token, ref end),
            } => write!(f, "Extra token {} found at {}:{}", token, start, end),
        }
    }
}

impl<L, T, LE, PE> From<PE> for ParseError<L, T, LE, PE> {
    fn from(error: PE) -> Self {
        ParseError::User { error }
    }
}

impl<L, T, LE, PE> From<LE> for ParseError<L, T, LE, PE> {
    fn from(error: LE) -> Self {
        ParseError::LexerError { error }
    }
}

#[cfg(feature = "std")]
impl<L, T, LE, PE> Error for ParseError<L, T, LE, PE>
where
    L: fmt::Debug + fmt::Display,
    T: fmt::Debug + fmt::Display,
    LE: fmt::Debug + fmt::Display,
    PE: fmt::Debug + fmt::Display,
{
    fn description(&self) -> &str {
        "parse error"
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ErrorRecovery<L, T, LE, PE> {
    pub error: ParseError<L, T, LE, PE>,
    pub dropped_tokens: Vec<(L, T, L)>,
}

/// Define a module using the generated parse from a `.lalrpop` file.
///
/// You have to specify the name of the module and the path of the file
/// generated by LALRPOP. If the input is in the root directory, you can
/// omit it.
///
/// # Example
/// ```ignore
/// // load parser in src/parser.lalrpop
/// lalrpop_mod!(parser);
///
/// // load parser in src/lex/parser.lalrpop
/// lalrpop_mod!(parser, "/lex/parser.rs");
///
/// // define a public module
/// lalrpop_mod!(pub parser);
/// ```

#[macro_export]
macro_rules! lalrpop_mod {
    ($(#[$attr:meta])* $vis:vis $modname:ident) => {
        lalrpop_mod!($(#[$attr])* $vis $modname, concat!("/", stringify!($modname), ".rs"));
    };

    ($(#[$attr:meta])* $vis:vis $modname:ident, $source:expr) => {
        #[rustfmt::skip]
        #[allow(clippy::extra_unused_lifetimes)]
        #[allow(clippy::needless_lifetimes)]
        #[allow(clippy::let_unit_value)]
        #[allow(clippy::just_underscores_and_digits)]
        $(#[$attr])* $vis mod $modname { include!(concat!(env!("OUT_DIR"), $source)); }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let err = ParseError::UnrecognizedToken::<i32, &str, &str> {
            token: (1, "t0", 2),
            expected: vec!["t1", "t2", "t3"]
                .into_iter()
                .map(|s| s.to_string())
                .collect(),
        };
        assert_eq!(
            format!("{}", err),
            "Unrecognized token `t0` found at 1:2\n\
             Expected one of t1, t2 or t3"
        );
    }
}
