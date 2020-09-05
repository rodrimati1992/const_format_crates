/// Compile-time assertions with formatting.
///
/// This macro requires the "assert" feature to be exported.
///
/// # Syntax
///
/// This macro uses the same syntax for the format string and formatting arguments as the
/// `formatcp` and `formatc` macros.
///
/// # Examples
///
/// ### Passing assertion
///
/// ```rust
/// // requires these two nightly features to use (for now)
/// #![feature(const_mut_refs)]
/// #![feature(const_panic)]
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
/// and how the compiler error looks like as of 2020-09-04.
///
/// ```compile_fail
/// // requires these two nightly features to use (for now)
/// #![feature(const_mut_refs)]
/// #![feature(const_panic)]
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
/// ```text
/// error: any use of this value will cause an error
///   --> src/macros/assertions.rs:47:1
///    |
/// 14 | assertc!(L + R == 5, "{} plus {} isn't 5 buddy", L,  R);
///    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at '
///
/// assertion failed: L + R == 5
///
/// 2 plus 2 isn't 5 buddy
///
/// ', src/macros/assertions.rs:14:1
///    |
///    = note: `#[deny(const_err)]` on by default
///    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
///
/// ```
///
#[macro_export]
macro_rules! assertc {
    ($cond:expr, $fmt_literal:expr $(,$fmt_arg:expr)* $(,)? ) => (
        const _: () = {
            const PANIC_IF_TRUE_NHPMWYD3NJA: bool = !($cond);

            if PANIC_IF_TRUE_NHPMWYD3NJA {
                ::std::panic!($crate::assertc!(
                    @fmt_string
                    ($crate),
                    (concat!(
                        "\n\nassertion failed: {PANIC_IF_TRUE_NHPMWYD3NJA}\n\n",
                        $fmt_literal,
                        "\n\n",
                    )),
                    $(($fmt_arg),)*
                    (PANIC_IF_TRUE_NHPMWYD3NJA = stringify!($cond)),
                ));
            }
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
