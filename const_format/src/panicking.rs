#![allow(non_fmt_panics)]

macro_rules! panic_ {
    ($($span:tt)*)=> {crate::pmr::respan_to!{($($span)*)
        pub trait Message {
            const COND: bool;
            const MSG: &'static str;
            const PANIC: usize = <Self as PanicIf>::PANIC;
        }

        pub trait PanicIf: Message {
            const PANIC: usize;
        }

        impl<T> crate::panicking::PanicIf for T
        where
            T: ?Sized + crate::panicking::Message,
        {
            const PANIC: usize = {
                use ::core::panic as do_not_show_this_code_rustc;

                let secret = T::MSG;
                if T::COND {
                    do_not_show_this_code_rustc!(secret)
                } else {
                    0
                }
            };
        }
    }};
}

panic_! {
    .
}

/// Equivalent to the panic macro, but takes a `&'static str` constant.
///
/// The arguments must be one of:
/// a literal, a non-associated constant, or an associated constant from a concrete type.
#[doc(hidden)]
#[macro_export]
macro_rules! assert_with_str {
    ($cond:expr, $message: expr) => {{
        struct __PanicWithStr;

        impl $crate::panicking::Message for __PanicWithStr {
            const COND: bool = $cond;
            const MSG: &'static str = $message;
        }

        const _: usize = <__PanicWithStr as $crate::panicking::Message>::PANIC;
    }};
}
