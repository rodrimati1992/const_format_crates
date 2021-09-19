#![cfg_attr(feature = "fmt", feature(const_mut_refs))]

extern crate const_format as cfmt_b;
extern crate self as const_format;

// Making sure that `const_format` points at this test crate.
pub const NOT_CF: usize = 13;
pub const _ASSERT_NOT_CF: [(); 13] = [(); const_format::NOT_CF];

mod misc_tests {
    #[cfg(feature = "assertc")]
    #[cfg(not(feature = "only_new_tests"))]
    mod assertc_tests;

    #[cfg(feature = "assertcp")]
    mod assertcp_tests;

    #[cfg(feature = "fmt")]
    #[cfg(not(feature = "only_new_tests"))]
    mod call_debug_fmt_macro;

    #[cfg(feature = "fmt")]
    #[cfg(not(feature = "only_new_tests"))]
    mod concatc_macro_tests;

    #[cfg(feature = "derive")]
    #[cfg(not(feature = "only_new_tests"))]
    mod derive_tests;

    #[cfg(feature = "assertc")]
    #[cfg(not(feature = "only_new_tests"))]
    mod equality_tests;

    #[cfg(not(feature = "only_new_tests"))]
    mod formatc_macros;

    #[cfg(feature = "fmt")]
    #[cfg(not(feature = "only_new_tests"))]
    mod impl_fmt_macro_tests;

    #[cfg(feature = "assertc")]
    #[cfg(not(feature = "only_new_tests"))]
    mod shared_cp_macro_tests;

    #[cfg(feature = "fmt")]
    #[cfg(not(feature = "only_new_tests"))]
    mod type_kind_coercion_macro_tests;

    #[cfg(feature = "fmt")]
    //#[cfg(not(feature = "only_new_tests"))]
    mod writec_macro;
}
