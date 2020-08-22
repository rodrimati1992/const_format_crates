mod error;
mod formatter;
mod std_type_impls;
pub mod str_writer;

pub use crate::formatting::{FormattingFlags, FormattingMode};

pub use self::{
    error::Error,
    formatter::{DebugList, DebugSet, DebugStruct, DebugTuple, Formatter},
    str_writer::StrWriter,
};

#[cfg(test)]
mod tests;
