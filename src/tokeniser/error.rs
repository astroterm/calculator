use miette::{Diagnostic, NamedSource, SourceSpan};
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
pub enum LexerError {
    #[error("The character {chr} is invalid")]
    #[diagnostic(code("lexer::invalid_character"), severity(Error))]
    InvalidCharacter { chr: char, span: SourceSpan },

    #[error("The identifier {iden} is invalid")]
    #[diagnostic(code("lexer::invalid_identifier"), severity(Error))]
    InvalidIdentifier { iden: String, span: SourceSpan },

    #[error("The number {num} is too large")]
    #[diagnostic(code("lexer::numerical_overflow"), severity(Error))]
    NumericalOverflow { num: String, span: SourceSpan },

    #[error("The number {num} is too small")]
    #[diagnostic(code("lexer::numerical_underflow"), severity(Error))]
    NumericalUnderflow { num: String, span: SourceSpan },
}
