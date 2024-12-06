#![cfg_attr(feature = "__inline_const_pat_tests", feature(inline_const_pat))]

extern crate const_format as cfmt_b;
extern crate self as const_format;

// Making sure that `const_format` points at this test crate.
pub const NOT_CF: usize = 13;
pub const _ASSERT_NOT_CF: [(); 13] = [(); const_format::NOT_CF];

mod misc_tests {
    #[cfg(feature = "assertc")]
    #[cfg(not(feature = "__only_new_tests"))]
    mod assertc_tests;

    mod clippy_warnings;

    #[cfg(feature = "assertcp")]
    mod assertcp_tests;

    #[cfg(feature = "fmt")]
    #[cfg(not(feature = "__only_new_tests"))]
    mod call_debug_fmt_macro;

    #[cfg(feature = "fmt")]
    #[cfg(not(feature = "__only_new_tests"))]
    mod concatc_macro_tests;

    #[cfg(feature = "derive")]
    #[cfg(not(feature = "__only_new_tests"))]
    mod derive_tests;

    #[cfg(feature = "assertc")]
    #[cfg(not(feature = "__only_new_tests"))]
    mod equality_tests;

    #[cfg(not(feature = "__only_new_tests"))]
    mod formatc_macros;

    #[cfg(feature = "fmt")]
    #[cfg(not(feature = "__only_new_tests"))]
    mod impl_fmt_macro_tests;

    #[cfg(not(feature = "__only_new_tests"))]
    mod shared_cp_macro_tests;

    #[cfg(feature = "fmt")]
    #[cfg(not(feature = "__only_new_tests"))]
    mod type_kind_coercion_macro_tests;

    #[cfg(feature = "fmt")]
    //#[cfg(not(feature = "__only_new_tests"))]
    mod writec_macro;

    #[cfg(feature = "__inline_const_pat_tests")]
    mod inline_const_pattern_tests;
}
