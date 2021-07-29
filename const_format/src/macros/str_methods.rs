/// A const subset of [`str::replace`],
/// which takes constants as arguments and returns a `&'static str`.
///
/// # Signature
///
/// This macro acts like a function of this signature:
/// ```rust
/// # trait Pattern {}
///
/// fn str_replace(
///     string: &'static str,
///     pattern: impl Pattern,
///     replace_with: &'static str,
/// ) -> &'static str
/// # {""}
/// ```
/// Where `pattern` can be any of these types:
///
/// - `&'static str`
///
/// - `u8`: required to be ascii (`0` up to `127` inclusive).
///
/// # Example
///
///
/// ```rust
/// use const_format::str_replace;
///
/// // Passing a string pattern
/// assert_eq!(
///     str_replace!("The incredible shrinking man.", "i", "eee"),
///     "The eeencredeeeble shreeenkeeeng man.",
/// );
///
/// // Passing an ascii u8 pattern.
/// assert_eq!(
///     str_replace!("The incredible shrinking man.", b'i', "eee"),
///     "The eeencredeeeble shreeenkeeeng man.",
/// );
///
/// // Removing all instances of the pattern
/// assert_eq!(
///     str_replace!("remove haire", "re", ""),
///     "move hai",
/// );
///
/// // This shows that all the arguments can be `const`s, they don't have to be literals.
/// {
///     const IN: &str = "Foo Boo Patoo";
///     const REPLACING: &str = "oo";
///     const REPLACE_WITH: &str = "uh";
///     assert_eq!(str_replace!(IN, REPLACING, REPLACE_WITH), "Fuh Buh Patuh");
/// }
/// ```
///
/// [`str::replace`]: https://doc.rust-lang.org/std/primitive.str.html#method.replace
#[macro_export]
#[cfg(feature = "const_generics")]
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "const_generics")))]
macro_rules! str_replace {
    ($string:expr, $replace:expr, $with:expr) => {{
        const STR_OSRCTFL4A: &$crate::pmr::str = $string;

        const REPLACE_OSRCTFL4A: $crate::__str_methods::ReplaceInput =
            $crate::__str_methods::ReplaceInputConv($replace).conv();

        const WITH_OSRCTFL4A: &$crate::pmr::str = $with;

        {
            use $crate::__str_methods::{str_replace, str_replace_length};

            const L: $crate::pmr::usize =
                str_replace_length(STR_OSRCTFL4A, REPLACE_OSRCTFL4A, WITH_OSRCTFL4A);

            const OB: &[$crate::pmr::u8; L] =
                &str_replace::<L>(STR_OSRCTFL4A, REPLACE_OSRCTFL4A, WITH_OSRCTFL4A);

            const OS: &$crate::pmr::str =
                unsafe { $crate::pmr::transmute::<&[$crate::pmr::u8], &$crate::pmr::str>(OB) };

            OS
        }
    }};
}

/// Creates a `&'static str` by repeating a `&'static str` some amount of times times.
///
/// # Example
///
/// ```rust
/// use const_format::str_repeat;
///
/// {
///     const OUT: &str = str_repeat!("hi ", 4);
///     assert_eq!(OUT, "hi hi hi hi ")
/// }
/// {
///     const IN: &str = "bye ";
///     const REPEAT: usize = 5;
///     const OUT: &str = str_repeat!(IN, REPEAT);
///     assert_eq!(OUT, "bye bye bye bye bye ")
/// }
///
/// ```
///
/// ### Failing
///
/// If this macro would produce too large a string,
/// it causes a compile-time error.
///
/// ```compile_fail
/// const_format::str_repeat!("hello", usize::MAX / 4);
/// ```
///
#[cfg_attr(
    feature = "testing",
    doc = r##"
```rust
const_format::str_repeat!("hello", usize::MAX.wrapping_add(4));
```
"##
)]
#[macro_export]
macro_rules! str_repeat {
    ($string:expr, $times:expr) => {{
        const STR_OSRCTFL4A: &$crate::pmr::str = $string;

        const TIMES_OSRCTFL4A: $crate::pmr::usize = $times;

        {
            use $crate::pmr::transmute;
            use $crate::pmr::{str, u8, usize};

            const STR_LEN: usize = STR_OSRCTFL4A.len();
            const OUT_LEN: usize = STR_LEN * TIMES_OSRCTFL4A;
            const OUT_B: &[u8; OUT_LEN] = &unsafe {
                let ptr = STR_OSRCTFL4A.as_ptr();
                transmute::<[[u8; STR_LEN]; TIMES_OSRCTFL4A], [u8; OUT_LEN]>(
                    [*transmute::<*const u8, &[u8; STR_LEN]>(ptr); TIMES_OSRCTFL4A],
                )
            };
            const OUT_S: &str = unsafe { transmute::<&'static [u8], &'static str>(OUT_B) };
            OUT_S
        }
    }};
}
