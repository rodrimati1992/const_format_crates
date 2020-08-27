use super::{Error, Formatter, FormattingFlags, StrWriterMut};

////////////////////////////////////////////////////////////////////////////////

/// A wrapper over an array used to build up a `&str` at compile-time.
///
#[derive(Debug, Copy, Clone)]
pub struct StrWriter<A: ?Sized = [u8]> {
    pub(super) len: usize,
    pub(super) buffer: A,
}

impl<A> StrWriter<A> {
    /// Constructs a `StrWriter` from a `u8` array
    pub const fn new(array: A) -> Self {
        Self {
            len: 0,
            buffer: array,
        }
    }
}

impl<A: ?Sized> StrWriter<A> {
    #[inline(always)]
    pub const fn len(&self) -> usize {
        self.len
    }

    #[inline(always)]
    pub const fn buffer(&self) -> &A {
        &self.buffer
    }

    // For borrowing this mutably in macros, without getting nested mutable references.
    #[inline(always)]
    pub const fn borrow_mutably(&mut self) -> &mut Self {
        self
    }
}

impl StrWriter {
    #[inline]
    pub const fn truncate(&mut self, length: usize) -> Result<(), Error> {
        self.as_mut().truncate(length)
    }

    /// Truncates this `StrWriterMut` to length 0.
    #[inline]
    pub const fn clear(&mut self) {
        self.len = 0;
    }

    #[inline(always)]
    pub const fn capacity(&self) -> usize {
        self.buffer.len()
    }

    #[inline(always)]
    pub fn as_str(&self) -> &str {
        // All the methods that modify the buffer must ensure utf8 validity,
        // only methods from this module need to ensure this.
        unsafe { core::str::from_utf8_unchecked(self.as_bytes()) }
    }

    #[inline(always)]
    pub fn as_bytes(&self) -> &[u8] {
        &self.buffer[..self.len]
    }

    #[inline(always)]
    pub const fn as_mut(&mut self) -> StrWriterMut<'_> {
        StrWriterMut {
            len: &mut self.len,
            buffer: &mut self.buffer,
        }
    }

    #[inline(always)]
    pub const fn make_formatter(&mut self, flags: FormattingFlags) -> Formatter<'_> {
        Formatter::from_sw_mut(
            flags,
            StrWriterMut {
                len: &mut self.len,
                buffer: &mut self.buffer,
            },
        )
    }
}
