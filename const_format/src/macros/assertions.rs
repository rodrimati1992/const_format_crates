///
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
