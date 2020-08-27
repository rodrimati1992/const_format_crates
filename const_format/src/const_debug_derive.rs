/// Derives the [`FormatMarker`] trait, and implements a `const_debug_fmt` method to 
/// format a type at compile-time
#[cfg(feature = "derive")]
pub use const_format_proc_macros::ConstDebug;
