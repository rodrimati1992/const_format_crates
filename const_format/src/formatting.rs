//! Formatting items that are always enabled,
//! the `fmt` module requires the "with_fmt" feature.

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

/// The formatting mode:
///
/// For Display formatting it can only be the `Regular` variant.
///
/// For Debug-like formatting it can be any of these.
///
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum FormattingMode {
    Regular,
    Hexadecimal,
    Binary,
}

impl FormattingMode {
    #[cfg(test)]
    pub(crate) const ALL: &'static [Self; 3] = &[
        FormattingMode::Regular,
        FormattingMode::Hexadecimal,
        FormattingMode::Binary,
    ];
}

////////////////////////////////////////////////////////////////////////////////

#[must_use]
#[derive(Debug, Copy, Clone)]
pub struct FormattingFlags {
    mode: FormattingMode,
    is_alternate: bool,
    margin: u16,
}

const INITIAL_MARGIN: u16 = 0;

#[doc(hidden)]
impl FormattingFlags {
    pub const __REG: Self = Self::NEW.set_alternate(false).unset_mode();
    pub const __HEX: Self = Self::NEW.set_alternate(false).set_hexadecimal_mode();
    pub const __BIN: Self = Self::NEW.set_alternate(false).set_binary_mode();

    pub const __A_REG: Self = Self::NEW.set_alternate(true).unset_mode();
    pub const __A_HEX: Self = Self::NEW.set_alternate(true).set_hexadecimal_mode();
    pub const __A_BIN: Self = Self::NEW.set_alternate(true).set_binary_mode();
}
impl FormattingFlags {
    #[doc(hidden)]
    pub const DEFAULT: Self = Self {
        mode: FormattingMode::Regular,
        is_alternate: false,
        margin: INITIAL_MARGIN,
    };

    /// Constructs a `FormattingFlags`
    pub const NEW: Self = Self {
        mode: FormattingMode::Regular,
        is_alternate: false,
        margin: INITIAL_MARGIN,
    };

    /// Sets the formatting mode,
    ///
    /// This usually doesn't affect the outputted text in display formatting.
    #[inline]
    pub const fn set_mode(mut self, mode: FormattingMode) -> Self {
        self.mode = mode;
        self
    }

    /// Sets the formatting mode to `FormattingMode::Regular`.
    ///
    /// This  means that integers are printed as decimal.
    #[inline]
    pub const fn unset_mode(mut self) -> Self {
        self.mode = FormattingMode::Regular;
        self
    }

    /// Sets the formatting mode to `FormattingMode::Hexadecimal`.
    ///
    /// This  means that integers are printed as hexadecimal.
    #[inline]
    pub const fn set_hexadecimal_mode(mut self) -> Self {
        self.mode = FormattingMode::Hexadecimal;
        self
    }

    /// Sets the formatting mode to `FormattingMode::Binary`.
    ///
    /// This  means that integers are printed as binary.
    #[inline]
    pub const fn set_binary_mode(mut self) -> Self {
        self.mode = FormattingMode::Binary;
        self
    }

    #[inline]
    pub const fn set_alternate(mut self, is_alternate: bool) -> Self {
        self.is_alternate = is_alternate;
        self
    }

    #[inline]
    pub const fn increment_margin(mut self) -> Self {
        self.margin += 4;
        self
    }

    #[inline]
    pub const fn decrement_margin(mut self) -> Self {
        self.margin -= 4;
        self
    }

    #[inline]
    pub const fn mode(self) -> FormattingMode {
        self.mode
    }

    #[inline]
    pub const fn is_alternate(self) -> bool {
        self.is_alternate
    }

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
