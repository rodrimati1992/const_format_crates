macro_rules! with_shared_docs {(
    $(#[$before_syntax:meta])*
    ;syntax
    $(#[$after_syntax:meta])*
    ;error_message
    $(#[$after_error_message:meta])*
    ;limitations
    $item:item
) => (
    $(#[$before_syntax])*
    /// This macro requires the "assert" feature to be exported,
    /// and uses `std::panic` for panicking due to an unforseen limitation
    /// in `core::panic`, which doesn't allow passing non-literal strings at compile-time.
    ///
    /// # Syntax
    ///
    /// This macro uses the same syntax for the format string and formatting arguments as the
    /// `formatcp` and `formatc` macros.
    ///
    $(#[$after_syntax])*
    /// # Error message
    ///
    /// `const_format` uses some workarounds to avoid requiring users to enable the
    /// `#![feature(const_panic)]` feature themselves,
    /// as a result, the error message isn't as good as it could possibly be.
    ///
    /// Compile-time errors with this macro include the formatted error message,
    /// and the module path + line where this macro was invoked.
    ///
    $(#[$after_error_message])*
    /// # Limitations
    ///
    /// This macro has these limitations:
    ///
    /// - It can only use constants that involve concrete types,
    /// so while a `Type::<u8>::FOO` in an argument would be fine,
    /// `Type::<T>::FOO` would not be (`T` being a type parameter).
    ///
    /// - Integer arguments must have a type inferrable from context,
    /// [as described in the integer arguments section in the root module
    /// ](./index.html#integer-args).
    ///
    $item
)}

////////////////////////////////////////////////////////////////////////////////

with_shared_docs! {
    /// Compile-time assertions with formatting.
    ///
    ;syntax
    ;error_message
    ;limitations
    ///
    /// # Examples
    ///
    /// ### Passing assertion
    ///
    /// ```rust
    /// // requires these two nightly features to use (for now)
    /// #![feature(const_mut_refs)]
    ///
    /// use const_format::assertc;
    ///
    /// use std::mem::size_of;
    ///
    /// assertc!(
    ///     size_of::<&str>() == size_of::<&[u8]>(),
    ///     "The size of `&str`({} bytes) and `&[u8]`({} bytes) aren't the same?!?!",
    ///     size_of::<&str>(),
    ///     size_of::<&[u8]>(),
    /// );
    ///
    /// # fn main(){}
    /// ```
    ///
    /// ### Failing assertion
    ///
    /// This example demonstrates a failing assertion,
    /// and how the compiler error looks like as of 2020-09-06.
    ///
    /// ```compile_fail
    /// // requires these two nightly features to use (for now)
    /// #![feature(const_mut_refs)]
    ///
    /// use const_format::assertc;
    ///
    /// use std::mem::size_of;
    ///
    /// const L: u64 = 2;
    /// const R: u64 = 2;
    ///
    /// assertc!(L + R == 5, "{} plus {} isn't 5 buddy", L,  R);
    ///
    /// # fn main(){}
    /// ```
    ///
    /// This is the compiler output,
    /// the first compilation error is there to have an indicator of what assertion failed,
    /// and the second is the assertion failure.
    ///
    /// ```text
    /// error: any use of this value will cause an error
    ///   --> src/macros/assertions.rs:59:1
    ///    |
    /// 13 | assertc!(L + R == 5, "{} plus {} isn't 5 buddy", L,  R);
    ///    /// | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ exceeded interpreter step limit (see `#[const_eval_limit]`)
    ///    |
    ///    = note: `#[deny(const_err)]` on by default
    ///    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
    ///
    /// error[E0080]: could not evaluate constant
    ///   --> const_format/src/panicking.rs:31:1
    ///    |
    /// 31 | ignore_me!();
    ///    | ^^^^^^^^^^^^^ the evaluated program panicked at '
    /// --------------------------------------------------------------------------------
    /// module_path: rust_out
    /// line: 13
    ///
    /// assertion failed: L + R == 5
    ///
    /// 2 plus 2 isn't 5 buddy
    /// --------------------------------------------------------------------------------
    /// ', const_format/src/panicking.rs:31:1
    ///    |
    ///    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
    ///
    /// ```
    ///
    #[macro_export]
    macro_rules! assertc {
        ($cond:expr, $fmt_literal:expr $(,$fmt_arg:expr)* $(,)? ) => (
            const _: () = {
                const PANIC_IF_TRUE_NHPMWYD3NJA: bool = !($cond);

                const MSG_NHPMWYD3NJA: &str = $crate::assertc!(
                    @fmt_string
                    ($crate),
                    (concat!(
                        "{SEP_NHPMWYD3NJA}\
                        module_path: {MODULE_NHPMWYD3NJA}\n\
                        line: {LINE_NHPMWYD3NJA}\n\n\
                        assertion failed: {PANIC_IF_TRUE_NHPMWYD3NJA}\n\n",
                        $fmt_literal,
                        "{SEP_NHPMWYD3NJA}",
                    )),
                    $(($fmt_arg),)*
                    (PANIC_IF_TRUE_NHPMWYD3NJA = stringify!($cond)),
                    (MODULE_NHPMWYD3NJA = module_path!()),
                    (LINE_NHPMWYD3NJA = line!()),
                    (SEP_NHPMWYD3NJA = "\
                        \n\
                        ----------------------------------------\
                        ----------------------------------------\
                        \n\
                    "),
                );

                $crate::assert_with_str!(PANIC_IF_TRUE_NHPMWYD3NJA, MSG_NHPMWYD3NJA);
            };
        );
        (@fmt_string ($path:path), ($fmt_literal:expr) $(,$($eveything:tt)*)? ) => (
            $crate::pmr::__formatc_if_impl!(
                (($path))
                (PANIC_IF_TRUE_NHPMWYD3NJA),
                ($fmt_literal),
                $($($eveything)*)?
            )
        );
    }
}
