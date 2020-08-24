/// Concatenates constants of primitive types into a `&'static str`.
///
/// Each argument is stringified after evaluating it, so `concatcp!(1u8 + 3) == "4"`
///
/// [For **examples** look here](#examples)
///
/// `concatcp` stands for "concatenate constants (of) primitives"
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
///
/// # Examples
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
#[macro_export]
macro_rules! concatcp {
    ()=>{""};
    ($($arg: expr),* $(,)?)=>(
        $crate::concatcp!(
            @with_fmt
            locals()
            $((to_pargument_display, $crate::pmr::FormattingFlags::DEFAULT, $arg))*
        )
    );
    (@with_fmt
        locals($(($local:ident, $local_init:expr))*)
        $(($to_pargument_fn:ident, $fmt:expr, $arg: expr))*
    )=>({
        // The suffix is to avoid name collisions with identifiers in the passed-in expression.
        #[allow(unused_mut, non_snake_case)]
        const CONCATP_NHPMWYD3NJA : (usize, &[$crate::pmr::PArgument]) = {
            let mut len = 0usize;

            $(let $local = $local_init;)*

            let array = [
                $({
                    let arg = $crate::pmr::PConvWrapper($arg).$to_pargument_fn($fmt);
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

/// Formats constants of primitive types into a `&'static str`
///
/// [For **examples** look here](#examples)
///
/// `formatcp` stands for "format constants (of) primitives"
///
/// # Syntax
///
/// This macro uses a limited version of the syntax from the standard library [`format`] macro,
/// it can do these things:
///
/// - Take positional arguments: `formatcp!("{}{0}", "hello" )`
///
/// - Take named arguments: `formatcp!("{a}{a}", a = "hello" )`
///
/// - Use constants from scope as arguments: `formatcp!("{FOO}")`,
/// equivalent to the [`format_args_implicits` RFC]
///
/// - Use Debug-like formatting: `formatcp!("{:?}", "hello" )`
///
/// - Use Hexsadecimal formatting:
/// `formatcp!("{:x}", "hello" )` or `formatcp!("{:x?}", "hello" )`.
///
/// - Use Binary formatting:
/// `formatcp!("{:b}", "hello" )` or `formatcp!("{:b?}", "hello" )`.
///
/// - Use Display formatting: `formatcp!("{}", "hello" )`
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
///
/// # Format specifiers
///
/// ### Debug-like
///
/// The `{:?}` formatter formats things similarly to how Debug does it.
///
/// For `&'static str` it only does this:
/// - Prepend and append the double quote character (`"`).
/// - Prepend the `\`, and `"` characters with a backslash (`\`).
///
/// Example:
/// ```
/// use const_format::formatcp;
///
/// assert_eq!(formatcp!("{:?}", r#" \ " ó "#), r#"" \\ \" ó ""#);
/// ```
///
/// ### Display
///
/// The `{}`/`{:}` formatter works the same as in [`format`].
///
///
/// # Examples
///
/// ### Implicit argument
///
/// ```rust
/// use const_format::formatcp;
///
/// const NAME: &str = "John";
///
/// const MSG: &str = formatcp!("Hello {NAME}, your name is {} bytes long", NAME.len());
///
/// assert_eq!(MSG, "Hello John, your name is 4 bytes long");
///
/// ```
///
/// ### Repeating arguments
///
/// ```rust
/// use const_format::formatcp;
///
/// const MSG: &str = formatcp!("{0}{S}{0}{S}{0}", "SPAM", S = "   ");
///
/// assert_eq!(MSG, "SPAM   SPAM   SPAM");
///
/// ```
///
/// ### Debug-like and Display formatting
///
/// ```rust
/// use const_format::formatcp;
///
/// const TEXT: &str = r#"hello " \ world"#;
/// const MSG: &str = formatcp!("{TEXT}____{TEXT:?}");
///
/// assert_eq!(MSG, r#"hello " \ world____"hello \" \\ world""#);
///
/// ```
///
/// [`format`]: https://doc.rust-lang.org/std/macro.format.html
///
/// [`format_args_implicits` RFC]:
/// https://github.com/rust-lang/rfcs/blob/master/text/2795-format-args-implicit-identifiers.md
///
///
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

#[macro_export]
#[cfg(feature = "with_fmt")]
macro_rules! formatc {
    ($format_string:expr $( $(, $expr:expr )+ )? $(,)? ) => (
        $crate::formatc!(
            @inner
            (($crate))
            $format_string
            $(, $(($expr),)+)?
        )
    );
    (@inner (($path:path)) $($everything:tt)*  ) => ({
        $crate::pmr::__formatc_impl!{
            (($path))
            $($everything)*
        }
    });
}
