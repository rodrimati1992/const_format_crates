mod error;
pub mod str_writer;

pub use crate::formatting::{FormattingFlags, FormattingMode};

pub use self::{error::Error, str_writer::StrWriter};
