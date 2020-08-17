/// The formatting style when writing a value into a buffer.
#[derive(Copy, Clone)]
pub enum Formatting {
    Debug,
    Display,
}

impl Formatting {
    /// Whether the current variant is `Display`
    #[inline(always)]
    pub const fn is_display(self) -> bool {
        matches!(self, Formatting::Display)
    }
}

/// For writing into an array from the start
pub struct LenAndArray<T: ?Sized> {
    /// The amount of elements written in `array`
    pub len: usize,
    pub array: T,
}

/// For writing into an array from the end
pub struct StartAndArray<T> {
    /// The first element in `array`
    pub start: usize,
    pub array: T,
}

/// Checks whether an ascii character needs to be escaped by prepending it with `\`.
#[inline(always)]
pub const fn is_escaped_simple(b: u8) -> bool {
    matches!(b, b'\\' | b'"')
}
