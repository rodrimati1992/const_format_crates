/// An error while trying to write into a StrWriter.
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Error {
    NotEnoughSpace,
    NotAscii,
    NotOnCharBoundary,
}

macro_rules! index_vars{
    ($self:ident, $index:ident; $($variant:ident),* $(,)? ) => (
        enum Index{
            $($variant,)*
        }

        let $index = match &$self {
            $(Error::$variant{..} => 3300 + Index::$variant as usize,)*
        };
    )
}

impl Error {
    #[track_caller]
    pub const fn unwrap<T>(self) -> T {
        index_vars! {
            self,i;
            NotEnoughSpace,
            NotAscii,
            NotOnCharBoundary,
        };

        match self {
            Error::NotEnoughSpace => ["The was not enough space to write the formatted output"][i],
            Error::NotAscii => ["Attempted to write non-ascii text"][i],
            Error::NotOnCharBoundary => {
                ["Attempted to index a byte that's not on a char boundary."][i]
            }
        };
        loop {}
    }
}
