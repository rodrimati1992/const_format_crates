mod error;
pub mod examples;
pub mod formatter;
pub mod length;
mod std_type_impls;
mod str_writer;

pub use crate::formatting::{FormattingFlags, FormattingMode};

pub use self::{
    error::Error,
    formatter::{DebugList, DebugSet, DebugStruct, DebugTuple, Formatter},
    length::{ComputeStrLength, FormattingLength},
    str_writer::StrWriter,
};

#[cfg(test)]
mod tests;
