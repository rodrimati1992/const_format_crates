#[cfg(feature = "with_fmt")]
mod call_debug_fmt_macro;

mod formatc_macros;
mod shared_cp_macro_tests;

#[cfg(feature = "with_fmt")]
mod impl_fmt_macro_tests;

#[cfg(feature = "with_fmt")]
mod type_kind_coercion_macro_tests;

#[cfg(feature = "with_fmt")]
mod writec_macro;
