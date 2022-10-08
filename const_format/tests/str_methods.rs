mod str_methods_modules {
    #[cfg(feature = "rust_1_51")]
    mod conv_ascii_case;

    #[cfg(feature = "rust_1_51")]
    mod str_replace;

    mod str_splice;

    #[cfg(feature = "rust_1_64")]
    mod str_split_tests;
}
