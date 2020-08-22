/// An error while trying to write into a StrWriter.
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Error {
    NotEnoughSpace,
    NotAscii,
    NotOnCharBoundary,
}
