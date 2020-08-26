#[cfg(feature = "with_fmt")]
#[cfg(not(feature = "only_new_tests"))]
mod call_debug_fmt_macro;

#[cfg(feature = "derive")]
mod derive_tests;

#[cfg(not(feature = "only_new_tests"))]
mod formatc_macros;

#[cfg(not(feature = "only_new_tests"))]
mod shared_cp_macro_tests;

#[cfg(feature = "with_fmt")]
#[cfg(not(feature = "only_new_tests"))]
mod impl_fmt_macro_tests;

#[cfg(feature = "with_fmt")]
#[cfg(not(feature = "only_new_tests"))]
mod type_kind_coercion_macro_tests;

#[cfg(feature = "with_fmt")]
#[cfg(not(feature = "only_new_tests"))]
mod writec_macro;

#[cfg(feature = "with_fmt")]
#[cfg(not(feature = "only_new_tests"))]
mod writec_macro;
