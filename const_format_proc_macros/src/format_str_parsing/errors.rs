#[derive(Debug, PartialEq)]
pub(crate) struct ParseError {
    pub(crate) pos: usize,
    pub(crate) kind: ParseErrorKind,
}

#[derive(Debug, PartialEq)]
pub(crate) enum ParseErrorKind {
    /// A `{` that wasn't closed.
    UnclosedArg,
    /// A `}` that doesn't close an argument.
    InvalidClosedArg,
    /// When parsing the number of a positional arguments
    NotANumber {
        what: String,
    },
    /// When parsing the identifier of a named argument
    NotAnIdent {
        what: String,
    },
    UnknownFormatting {
        what: String,
    },
}

impl ParseErrorKind {
    pub fn not_a_number(what: &str) -> Self {
        Self::NotANumber {
            what: what.to_string(),
        }
    }
    pub fn not_an_ident(what: &str) -> Self {
        Self::NotAnIdent {
            what: what.to_string(),
        }
    }
    pub fn unknown_formatting(what: &str) -> Self {
        Self::UnknownFormatting {
            what: what.to_string(),
        }
    }
}
