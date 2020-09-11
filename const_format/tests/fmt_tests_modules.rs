#![cfg(feature = "fmt")]
#![cfg_attr(feature = "fmt", feature(const_mut_refs))]

// Prevents importing from const_format, requiring importing from cfmt_b.
extern crate const_format as cfmt_a;
extern crate self as const_format;

// Making sure that `const_format` points at this test crate.
pub const NOT_CF: usize = 13;
pub const _ASSERT_NOT_CF: [(); 13] = [(); const_format::NOT_CF];

cfmt_a::__declare_rng_ext! {}

mod fmt_tests {
    #[cfg(not(feature = "only_new_tests"))]
    mod display_formatting;

    #[cfg(not(feature = "only_new_tests"))]
    mod formatted_writing;

    #[cfg(not(feature = "only_new_tests"))]
    mod formatter_methods;

    #[cfg(not(feature = "only_new_tests"))]
    mod std_impl_tests;

    #[cfg(not(feature = "only_new_tests"))]
    mod str_writer_methods;

    #[cfg(not(feature = "only_new_tests"))]
    mod str_writer_mut;
}
