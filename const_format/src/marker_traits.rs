//! Marker traits for types that can be formatted and/or be written to.

mod format_marker;
mod write_marker;

pub use self::{
    format_marker::{FormatMarker, IsAFormatMarker, IsArrayKind, IsNotStdKind, IsStdKind},
    write_marker::{IsAWriteMarker, IsNotAStrWriter, WriteMarker},
};
