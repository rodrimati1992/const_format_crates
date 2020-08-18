/// Concatenates constants of primitive types into a `&'static str`.
///
/// Each argument is stringified after evaluating it, so `concatcp!(1u8 + 3) == "4"`
///
/// # Example
///
/// ### Literal arguments
///
///
/// ```rust
/// use const_format::concatcp;
///
/// const TWO: u64 = 2;
///
/// const MSG: &str = concatcp!(TWO, "+", TWO, "=", 2u8 + 2);
///
/// assert_eq!(MSG, "2+2=4");
///
/// ```
///
/// ### `const` arguments
///
/// ```rust
/// use const_format::concatcp;
///
/// const PASSWORD: &str = "password";
///
/// const fn times() -> u64 { 10 }
///
/// const MSG: &str =
///     concatcp!("The password is \"", PASSWORD, "\", you can only guess ", times(), " times.");
///
/// assert_eq!(MSG, r#"The password is "password", you can only guess 10 times."#);
///
/// ```
///
/// # Limitations
///
/// This macro can only take constants of these types as inputs:
///
/// - `&str`
///
/// - `i*`/`u*` (all the primitive integer types).
///
/// - `bool`
///
/// This macro also shares
/// [the limitations described in here](./index.html#macro-limitations)
/// as well.
#[macro_export]
macro_rules! concatcp {
    ()=>{""};
    ($($arg: expr),* $(,)?)=>(
        $crate::concatcp!(
            @with_fmt
            locals()
            $(($crate::pmr::Formatting::Display, $arg))*
        )
    );
    (@with_fmt
        locals($(($local:ident, $local_init:expr))*)
        $(($fmt:expr, $arg: expr))*
    )=>({
        // The suffix is to avoid name collisions with identifiers in the passed-in expression.
        const CONCATP_NHPMWYD3NJA : (usize, &[$crate::pmr::PArgument]) = {
            let mut len = 0usize;

            $(let $local = $local_init;)*

            let array = [
                $({
                    let arg = $crate::pmr::PConvWrapper($arg).to_pargument($fmt);
                    len += arg.fmt_len;
                    arg
                }),*
            ];

            (len, &{array})
        };

        {
            const ARR_LEN: usize = CONCATP_NHPMWYD3NJA.0;

            const CONCAT_ARR: &$crate::pmr::LenAndArray<[u8; ARR_LEN]> = {
                use $crate::{
                    pmr::PVariant,
                    __write_pvariant,
                };

                let mut out = $crate::pmr::LenAndArray{
                    len: 0,
                    array: [0u8; ARR_LEN],
                };

                let input = CONCATP_NHPMWYD3NJA.1;

                $crate::__for_range!{ outer_i in 0..input.len() =>
                    let current = &input[outer_i];

                    match current.elem {
                        PVariant::Str(s) => __write_pvariant!(str, current, s => out),
                        PVariant::Int(int) => __write_pvariant!(int, current, int => out),
                    }
                }
                &{out}
            };
            const CONCAT_STR: &str = unsafe{
                // This transmute truncates the length of the array to the amound of written bytes.
                let slice =
                    $crate::pmr::Transmute::<&[u8; ARR_LEN], &[u8; CONCAT_ARR.len]>{
                        from: &CONCAT_ARR.array,
                    }.to;

                $crate::pmr::Transmute::<&[u8], &str>{from: slice}.to
            };
            CONCAT_STR
        }
    });
}

#[macro_export]
macro_rules! formatcp {
    ($format_string:expr $( $(, $expr:expr )+ )? $(,)? ) => (
        $crate::formatcp!(
            @inner
            (($crate))
            $format_string
            $(, $(($expr),)+)?
        )
    );
    (@inner (($path:path)) $($everything:tt)*  ) => (
        $crate::pmr::__formatcp_impl!(
            (($path))
            $($everything)*
        )
    );
}
