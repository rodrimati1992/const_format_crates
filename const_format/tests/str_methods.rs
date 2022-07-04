mod str_methods_modules {
    #[cfg(feature = "const_generics")]
    mod conv_ascii_case;

    #[cfg(feature = "const_generics")]
    mod str_replace;

    mod str_splice;

    #[cfg(feature = "more_str_macros")]
    mod str_split_tests;
}
