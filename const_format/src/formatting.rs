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

/// How numbers are formatted in debug formatters.
///
/// Hexadecimal or binary formatting in the formatting string from this crate imply
/// debug formatting.
///
///
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum NumberFormatting {
    /// Formats numbers as decimal
    Decimal,
    /// Formats numbers as hexadecimal
    Hexadecimal,
    /// Formats numbers as binary
    Binary,
}

impl NumberFormatting {
    #[cfg(test)]
    #[cfg(feature = "fmt")]
    pub(crate) const ALL: &'static [Self; 3] = &[
        NumberFormatting::Decimal,
        NumberFormatting::Hexadecimal,
        NumberFormatting::Binary,
    ];
}

////////////////////////////////////////////////////////////////////////////////

/// This type bundles configuration for how to format data into strings, including.
///
/// # Number formatting
///
/// How numbers are formatted in debug formatters,
/// It can be accessed with the `num_fmt` method, and set with the `set_num_fmt` method.
///
/// Each type of number formatting corresponds to a [`NumberFormatting`] variant:
///
/// - `NumberFormatting::Decimal` (eg: `formatc!("{:?}", FOO)`):
/// formats numbers as decimal.
///
/// - `NumberFormatting::Hexadecimal`  (eg: `formatc!("{:x}", FOO)`):
/// formats numbers as hexadecimal.
///
/// - `NumberFormatting::Binary` (eg: `formatc!("{:b}", FOO)`):
/// formats numbers as binary.
///
/// Hexadecimal or binary formatting in the formatting string from this crate imply
/// debug formatting,
/// and can be used to for example print an array of binary numbers.
///
/// # Alternate flag
///
/// A flag that types can use to be formatted differently when it's enabled,
/// checked with the `.is_alternate()` method.
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
    num_fmt: NumberFormatting,
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
        num_fmt: NumberFormatting::Decimal,
        is_alternate: false,
        margin: INITIAL_MARGIN,
    };

    /// Constructs a `FormattingFlags` with these values:
    ///
    /// - number formatting: NumberFormatting::Decimal
    ///
    /// - is alternate: false
    ///
    /// - margin: 0
    ///
    pub const NEW: Self = Self {
        num_fmt: NumberFormatting::Decimal,
        is_alternate: false,
        margin: INITIAL_MARGIN,
    };

    /// Constructs a `FormattingFlags` with these values:
    ///
    /// - number formatting: NumberFormatting::Decimal
    ///
    /// - is alternate: false
    ///
    /// - margin: 0
    ///
    #[inline]
    pub const fn new() -> Self {
        Self::NEW
    }

    /// Sets the integer formatting num_fmt,
    ///
    /// This usually doesn't affect the outputted text in display formatting.
    #[inline]
    pub const fn set_num_fmt(mut self, num_fmt: NumberFormatting) -> Self {
        self.num_fmt = num_fmt;
        self
    }

    /// Sets the formatting num_fmt to `NumberFormatting::Decimal`.
    ///
    /// This means that numbers are written as decimal.
    #[inline]
    pub const fn set_decimal(mut self) -> Self {
        self.num_fmt = NumberFormatting::Decimal;
        self
    }

    /// Sets the formatting num_fmt to `NumberFormatting::Hexadecimal`.
    ///
    /// This means that numbers are written as hexadecimal.
    #[inline]
    pub const fn set_hexadecimal(mut self) -> Self {
        self.num_fmt = NumberFormatting::Hexadecimal;
        self
    }

    /// Sets the formatting num_fmt to `NumberFormatting::Binary`.
    ///
    /// This means that numbers are written as binary.
    #[inline]
    pub const fn set_binary(mut self) -> Self {
        self.num_fmt = NumberFormatting::Binary;
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

    #[inline]
    #[cfg(feature = "fmt")]
    pub(crate) const fn copy_margin_of(mut self, other: FormattingFlags) -> Self {
        self.margin = other.margin;
        self
    }

    /// Gets the current `NumberFormatting`.
    #[inline]
    pub const fn num_fmt(self) -> NumberFormatting {
        self.num_fmt
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
