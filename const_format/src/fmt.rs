mod error;
mod formatter;
mod std_type_impls;
mod str_writer;
mod str_writer_mut;

pub use crate::formatting::{FormattingFlags, FormattingMode};

pub use self::{
    error::Error,
    formatter::{ComputeStrLength, DebugList, DebugSet, DebugStruct, DebugTuple, Formatter},
    str_writer::StrWriter,
    str_writer_mut::StrWriterMut,
};

#[cfg(all(test, not(feature = "only_new_tests")))]
mod tests;
