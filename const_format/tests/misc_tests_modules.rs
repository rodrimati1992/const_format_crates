#![cfg_attr(feature = "fmt", feature(const_mut_refs))]
#![cfg_attr(feature = "assert", feature(const_panic))]

mod misc_tests {
    #[cfg(feature = "assert")]
    mod asserts;

    #[cfg(feature = "fmt")]
    #[cfg(not(feature = "only_new_tests"))]
    mod call_debug_fmt_macro;

    #[cfg(feature = "derive")]
    mod derive_tests;

    #[cfg(feature = "assert")]
    mod equality_tests;

    #[cfg(not(feature = "only_new_tests"))]
    mod formatc_macros;

    #[cfg(feature = "fmt")]
    #[cfg(not(feature = "only_new_tests"))]
    mod impl_fmt_macro_tests;

    #[cfg(not(feature = "only_new_tests"))]
    mod shared_cp_macro_tests;

    #[cfg(feature = "fmt")]
    #[cfg(not(feature = "only_new_tests"))]
    mod type_kind_coercion_macro_tests;

    #[cfg(feature = "fmt")]
    #[cfg(not(feature = "only_new_tests"))]
    mod writec_macro;
}
