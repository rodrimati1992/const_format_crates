//! Formatting items that are always enabled,
//! the `fmt` module requires the "fmt" feature.

///
#[derive(Debug, Copy, Clone, PartialEq)]
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

/// How integers are formatted in debug formatters.
///
/// Hexadecimal or binary formatting in the formatting string from this crate imply
/// debug formatting.
///
///
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum NumberFormatting {
    /// Formats integers as decimal
    Decimal,
    /// Formats integers as hexadecimal
    Hexadecimal,
    /// Formats integers as binary
    Binary,
}

impl NumberFormatting {
    #[cfg(test)]
    pub(crate) const ALL: &'static [Self; 3] = &[
        NumberFormatting::Decimal,
        NumberFormatting::Hexadecimal,
        NumberFormatting::Binary,
    ];
}

////////////////////////////////////////////////////////////////////////////////

/// This type bundles configuration for how to format data into strings, including.
///
/// # Formatting mode
///
/// How integers are formatted in debug formatters,
/// each one corresponding to a [`NumberFormatting`] variant:
///
/// - `NumberFormatting::Decimal` (eg: `formatc!("{:?}", FOO)`):
/// formats integers as decimal.
///
/// - `NumberFormatting::Hexadecimal`  (eg: `formatc!("{:x}", FOO)`):
/// formats integers as hexadecimal.
///
/// - `NumberFormatting::Binary` (eg: `formatc!("{:b}", FOO)`):
/// formats integers as binary.
///
/// Hexadecimal or binary formatting in the formatting string from this crate imply
/// debug formatting,
/// and can be used to for example print an array of binary integers.
///
/// # Alternate flag
///
/// A flag that types can use to be formatted differently when it's enabled.
///
/// The default behavior when it is enabled is this:
///
/// - The Debug formater (eg: `formatc!("{:#?}", FOO)`):
/// pretty print structs and enums.
///
/// - The hexadecimal formater (eg: `formatc!("{:#x}", FOO)`):
/// prefixes numbers with `0x`.
///
/// - The binary formater (eg: `formatc!("{:#b}", FOO)`):
/// prefixes numbers with `0b`.`
///
/// # Margin
///
/// The amount of leading space when writing structs and enums into a [`Formatter`].
///
/// [`Formatter`]: ./struct.Formatter.html
///
#[must_use]
#[derive(Debug, Copy, Clone)]
pub struct FormattingFlags {
    mode: NumberFormatting,
    is_alternate: bool,
    margin: u16,
}

const INITIAL_MARGIN: u16 = 0;

#[doc(hidden)]
impl FormattingFlags {
    pub const __REG: Self = Self::NEW.set_alternate(false).set_decimal();
    pub const __HEX: Self = Self::NEW.set_alternate(false).set_hexadecimal();
    pub const __BIN: Self = Self::NEW.set_alternate(false).set_binary();

    pub const __A_REG: Self = Self::NEW.set_alternate(true).set_decimal();
    pub const __A_HEX: Self = Self::NEW.set_alternate(true).set_hexadecimal();
    pub const __A_BIN: Self = Self::NEW.set_alternate(true).set_binary();
}
impl FormattingFlags {
    #[doc(hidden)]
    pub const DEFAULT: Self = Self {
        mode: NumberFormatting::Decimal,
        is_alternate: false,
        margin: INITIAL_MARGIN,
    };

    /// Constructs a `FormattingFlags`
    pub const NEW: Self = Self {
        mode: NumberFormatting::Decimal,
        is_alternate: false,
        margin: INITIAL_MARGIN,
    };

    /// Sets the integer formatting mode,
    ///
    /// This usually doesn't affect the outputted text in display formatting.
    #[inline]
    pub const fn set_num_fmt(mut self, mode: NumberFormatting) -> Self {
        self.mode = mode;
        self
    }

    /// Sets the formatting mode to `NumberFormatting::Decimal`.
    ///
    /// This means that integers are written as decimal.
    #[inline]
    pub const fn set_decimal(mut self) -> Self {
        self.mode = NumberFormatting::Decimal;
        self
    }

    /// Sets the formatting mode to `NumberFormatting::Hexadecimal`.
    ///
    /// This means that integers are written as hexadecimal.
    #[inline]
    pub const fn set_hexadecimal(mut self) -> Self {
        self.mode = NumberFormatting::Hexadecimal;
        self
    }

    /// Sets the formatting mode to `NumberFormatting::Binary`.
    ///
    /// This means that integers are written as binary.
    #[inline]
    pub const fn set_binary(mut self) -> Self {
        self.mode = NumberFormatting::Binary;
        self
    }

    /// Sets whether the formatting flag is enabled.
    #[inline]
    pub const fn set_alternate(mut self, is_alternate: bool) -> Self {
        self.is_alternate = is_alternate;
        self
    }

    /// Increments the margin.
    ///
    /// The margin is used for pretty printing data structures.
    #[inline]
    pub const fn increment_margin(mut self) -> Self {
        self.margin += 4;
        self
    }

    /// Decrements the margin.
    ///
    /// The margin is used for pretty printing data structures.
    #[inline]
    pub const fn decrement_margin(mut self) -> Self {
        self.margin -= 4;
        self
    }

    pub(crate) const fn copy_margin_of(mut self, other: FormattingFlags) -> Self {
        self.margin = other.margin;
        self
    }

    /// Gets the current `NumberFormatting`.
    #[inline]
    pub const fn mode(self) -> NumberFormatting {
        self.mode
    }

    /// Gets whether the alternate flag is enabled
    #[inline]
    pub const fn is_alternate(self) -> bool {
        self.is_alternate
    }

    /// Gets the current margin.
    ///
    /// The margin is used for pretty printing data structures.
    #[inline]
    pub const fn margin(self) -> usize {
        self.margin as usize
    }
}

////////////////////////////////////////////////////////////////////////////////

/// For writing into an array from the start
pub struct LenAndArray<T: ?Sized> {
    /// The amount of elements written in `array`
    pub len: usize,
    pub array: T,
}

/// For writing into an array from the end
pub struct StartAndArray<T: ?Sized> {
    /// The first element in `array`
    pub start: usize,
    pub array: T,
}

////////////////////////////////////////////////////////////////////////////////

pub struct ForEscaping {
    pub is_escaped: u128,
    pub is_backslash_escaped: u128,
    pub escape_char: [u8; 16],
}

impl ForEscaping {
    /// Gets the backslash escape for a character that is kwown to be escaped with a backslash.
    #[inline(always)]
    pub const fn get_backslash_escape(b: u8) -> u8 {
        FOR_ESCAPING.escape_char[(b & 0b1111) as usize]
    }
}

/// Converts 0..=0xF to its ascii representation of '0'..='9' and 'A'..='F'
#[inline(always)]
pub const fn hex_as_ascii(n: u8) -> u8 {
    if n < 10 {
        n + b'0'
    } else {
        n - 10 + b'A'
    }
}

pub const FOR_ESCAPING: &ForEscaping = {
    let mut is_backslash_escaped = 0;

    let escaped = [
        (b'\t', b't'),
        (b'\n', b'n'),
        (b'\r', b'r'),
        (b'\'', b'\''),
        (b'"', b'"'),
        (b'\\', b'\\'),
    ];

    // Using the fact that the characters above all have different bit patterns for
    // the lowest 4 bits.
    let mut escape_char = [0u8; 16];

    __for_range! {i in 0..escaped.len() =>
        let (code, escape) = escaped[i];
        is_backslash_escaped |= 1 << code;

        let ei = (code & 0b1111) as usize;
        let prev_escape = escape_char[ei] as usize;
        ["Oh no, some escaped character uses the same 4 lower bits as another"][prev_escape];
        escape_char[ei] = escape;
    }

    // Setting all the control characters as being escaped.
    let is_escaped = is_backslash_escaped | 0xFFFF_FFFF;

    &ForEscaping {
        escape_char,
        is_backslash_escaped,
        is_escaped,
    }
};
