
#[derive(Copy, Clone)]
pub enum Formatting {
    Debug,
    Display
}

impl Formatting{
    #[inline(always)]
    pub const fn is_display(self) -> bool {
        matches!(self, Formatting::Display)
    }
}

pub struct LenAndArray<T: ?Sized> {
    pub len: usize,
    pub array: T,
}

pub struct StartAndArray<T> {
    pub start: usize,
    pub array: T,
}

#[inline(always)]
pub const fn is_escaped_simple(b: u8) -> bool{
    matches!(b, b'\\'|b'"')
}