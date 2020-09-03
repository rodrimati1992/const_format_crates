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
/// const MSG: &str = concatcp!(2u8, "+", 2u8, "=", 2u8 + 2);
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
    ($($arg: expr),* $(,)?)=>({
        // The suffix is to avoid name collisions with identifiers in the passed-in expression.
        #[allow(unused_mut, non_snake_case)]
        const CONCATP_NHPMWYD3NJA : (usize, &[$crate::pmr::PArgument]) = {
            let mut len = 0usize;

            let fmt = $crate::pmr::FormattingFlags::NEW;

            let array = [
                $({
                    let arg = $crate::pmr::PConvWrapper($arg).to_pargument_display(fmt);
                    len += arg.fmt_len;
                    arg
                }),*
            ];

            (len, &{array})
        };

        $crate::__concatcp_inner!(CONCATP_NHPMWYD3NJA)
    });
}

#[doc(hidden)]
#[macro_export]
macro_rules! __concatcp_inner {
    ($variables:ident) => {{
        const ARR_LEN: usize = $variables.0;

        const CONCAT_ARR: &$crate::pmr::LenAndArray<[u8; ARR_LEN]> = {
            use $crate::{__write_pvariant, pmr::PVariant};

            let mut out = $crate::pmr::LenAndArray {
                len: 0,
                array: [0u8; ARR_LEN],
            };

            let input = $variables.1;

            $crate::__for_range! { outer_i in 0..input.len() =>
                let current = &input[outer_i];

                match current.elem {
                    PVariant::Str(s) => __write_pvariant!(str, current, s => out),
                    PVariant::Int(int) => __write_pvariant!(int, current, int => out),
                }
            }
            &{ out }
        };
        const CONCAT_STR: &str = unsafe {
            // This transmute truncates the length of the array to the amound of written bytes.
            let slice =
                $crate::pmr::transmute::<&[u8; ARR_LEN], &[u8; CONCAT_ARR.len]>(&CONCAT_ARR.array);

            $crate::pmr::transmute::<&[u8], &str>(slice)
        };
        CONCAT_STR
    }};
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
/// - Use Debug-like formatting (eg: `formatcp!("{:?}", "hello" ):
/// Similar to how Debug formatting in the standard library works,
/// except that it does not escape unicode characters.`
///
/// - Use Hexsadecimal formatting (eg: `formatcp!("{:x}", "hello" )`):
/// Formats numbers as capitalized hexadecimal,
/// The alternate version (written as `"{:#x}"`), prefixes the number with `0x`
///
/// - Use Binary formatting (eg: `formatcp!("{:b}", "hello" )`).
/// The alternate version (written as `"{:#b}"`), prefixes the number with `0b`
///
/// - Use Display formatting: `formatcp!("{}", "hello" )`
///
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
/// For `&'static str` it does these things:
/// - Prepend and append the double quote character (`"`).
/// - Escape the `'\t'`,`'\n'`,`'\r'`,`'\\'`, `'\''`, and`'\"'` characters.
/// - Escape control characters with `\xYY`,
/// where `YY` is the hexadecimal value of the control character.
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

/// Formats constants into a `&'static str`.
///
/// # Syntax
///
/// This macro uses the syntax described in
/// [the const_format::fmt module](./fmt/index.html#fmtsyntax)
///
/// # Limitations
///
/// This macro has [the limitations described in here](./index.html#macro-limitations).
///
/// # Example
///
/// ```rust
/// #![feature(const_mut_refs)]
///
/// use const_format::for_examples::Point3;
/// use const_format::formatc;
///
/// // Formatting a non-std struct.
/// const POINT: &str = formatc!("{:?}", Point3{x: 8, y: 13, z: 21});
///
/// // Formatting a number as decimal, hexadecimal, and binary
/// const NUMBER: &str = formatc!("{0},{0:x},{0:b}", 10u8);
///
/// // Formatting the numbers in an array as decimal, hexadecimal, and binary.
/// // You can use the name of cnstants from scope, as well as named arguments.
/// const ARR: &[u32] = &[9, 25];
/// const ARRAY: &str = formatc!("{ARR:?},{ARR:x},{ARR:b}");
///
///
/// assert_eq!(POINT, "Point3 { x: 8, y: 13, z: 21 }");
/// assert_eq!(NUMBER, "10,A,1010");
/// assert_eq!(ARRAY, "[9, 25],[9, 19],[1001, 11001]");
///
/// ```
#[macro_export]
#[cfg(feature = "fmt")]
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

/// Writes some formatted text into a buffer.
///
/// This macro evaluates to a `Result<(), const_format::Error>` which must be handled.
///
/// # Syntax
///
/// The syntax is similar to that of other formatting macros in this crate:
///
/// ```ìgnore
/// ẁritec!(
///     writer_expression,
///     "formatting literal",
///     positional_arg_0_expression,
///     positional_arg_1_expression,
///     named_arg_foo = expression,
///     named_arg_bar = expression,
/// )
/// ```
///
/// The syntax is otherwise the same as described in
/// [the const_format::fmt module](./fmt/index.html#fmtsyntax).
///
/// # Writers
///
/// The first argument must be a type that implements the [`WriteMarker`] trait,
/// and has these inherent methods:
/// ```ignore
/// const fn borrow_mutably(&mut self) -> &mut Self
/// const fn make_formatter(&mut self, flags: FormattingFlags) -> Formatter<'_>
/// ```
///
/// [This example](#custom-writable-example) below shows how to use this macro
/// with a custom type.
///
/// # Limitations
///
/// Integer arguments must have a type inferrable from context,
/// [more details in the Integer arguments section](./index.html#integer-args).
///
/// # Examples
///
/// ### Ẁriting a Display impl.
///
/// ```
/// #![feature(const_mut_refs)]
///
/// use const_format::{Error, Formatter, StrWriter};
/// use const_format::{impl_fmt, try_, writec};
///
/// pub struct Foo(u32, &'static str);
///
/// impl_fmt!{
///     impl Foo;
///     pub const fn const_display_fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
///         try_!(writec!(f, "{},", self.0));
///         try_!(writec!(f, "{:?};", self.1));
///         Ok(())
///     }
/// }
///
/// // Coerces the `&mut StrWriter<[u8; 128]>` to `&mut StrWriter<[u8]>`.
/// // This is necessary because the `as_str` method is defined for `StrWriter<[u8]>`.
/// let writer: &mut StrWriter = &mut StrWriter::new([0; 128]);
/// writec!(writer, "{}", Foo(100, "bar"))?;
///
/// assert_eq!(writer.as_str(), r#"100,"bar";"#);
///
/// # Ok::<(), const_format::Error>(())
/// ```
///
/// <span id="custom-writable-example"></span>
/// ### Writing to a custom type
///
/// This example demonstrates how you can use the `ẁritec` macro with a custom type,
/// in this case it's a buffer that is cleared every time it's written.
///
/// ```rust
/// #![feature(const_mut_refs)]
///
/// use const_format::marker_traits::{IsNotAStrWriter, WriteMarker};
/// use const_format::{Formatter, FormattingFlags};
/// use const_format::writec;
///
/// const ARRAY_CAP: usize = 20;
/// struct Array {
///     len: usize,
///     arr: [u8; ARRAY_CAP],
/// }
///
/// impl WriteMarker for Array{
///     type Kind = IsNotAStrWriter;
///     type This = Self;
/// }
///
/// impl Array {
///     // Gets the part of the array that has been written to.
///     pub const fn as_bytes(&self) -> &[u8] {
///         const_format::utils::slice_up_to_len_alt(&self.arr, self.len)
///     }
///
///     pub const fn borrow_mutably(&mut self) -> &mut Self {
///         self
///     }
///
///     pub const fn make_formatter(&mut self, flags: FormattingFlags) -> Formatter<'_> {
///         Formatter::from_custom_cleared(&mut self.arr, &mut self.len, flags)
///     }
/// }
///
///
/// let mut buffer = Array{ arr: [0; ARRAY_CAP], len: 0 };
///
/// writec!(buffer, "{:?}", [3u8, 5, 8, 13, 21])?;
/// assert_eq!(buffer.as_bytes(), b"[3, 5, 8, 13, 21]");
///
/// writec!(buffer, "{}{}", "Hello, world!", 100u16)?;
/// assert_eq!(buffer.as_bytes(), b"Hello, world!100");
///
/// # Ok::<(), const_format::Error>(())
/// ```
///
///
///
/// [`WriteMarker`]: ./marker_traits/trait.WriteMarker.html
///
///
///
///
#[macro_export]
#[cfg(feature = "fmt")]
macro_rules! writec {
    ( $writer:expr, $format_string:expr $( $(, $expr:expr )+ )? $(,)? ) => (
        $crate::writec!(
            @inner
            (($crate))
            ($writer)
            $format_string
            $(, $(($expr),)+)?
        )
    );
    (@inner (($path:path)) $($everything:tt)*  ) => ({
        $crate::pmr::__writec_impl!{
            (($path))
            $($everything)*
        }
    });
}
