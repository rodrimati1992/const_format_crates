#![cfg(feature = "fmt")]
#![cfg_attr(feature = "fmt", feature(const_mut_refs))]

const_format::__declare_rng_ext! {}

mod fmt_tests {
    #[cfg(not(feature = "only_new_tests"))]
    mod display_formatting;

    #[cfg(not(feature = "only_new_tests"))]
    mod formatted_writing;

    #[cfg(not(feature = "only_new_tests"))]
    mod std_impl_tests;

    #[cfg(not(feature = "only_new_tests"))]
    mod str_writer_methods;

    mod str_writer_mut;
}
