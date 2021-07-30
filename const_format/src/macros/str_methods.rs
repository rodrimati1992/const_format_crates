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

            const OS: &$crate::pmr::str = unsafe { $crate::__priv_transmute_bytes_to_str!(OB) };

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
            const OUT_S: &str = unsafe { $crate::__priv_transmute_bytes_to_str!(OUT_B) };
            OUT_S
        }
    }};
}

/// Replaces a substring in a `&'static str`.
/// Returns both the new resulting `&'static str`, and the replaced substring.
///
/// # Signature
///
/// This macro acts like a function of this signature:
/// ```rust
/// # trait SomeIndex {}
///
/// fn str_splice(
///     input: &'static str,
///     range: impl SomeIndex,
///     replace_with: &'static str,
/// ) -> const_format::SplicedStr
/// # {unimplemented!()}
/// ```
/// Where `range` determines what part of `input` is replaced,
/// and can be any of these types:
///
/// - `usize`
/// - `Range<usize>`
/// - `RangeTo<usize>`
/// - `RangeFrom<usize>`
/// - `RangeInclusive<usize>`
/// - `RangeToInclusive<usize>`
/// - `RangeFull`
///
/// [`SplicedStr`] contains:
/// - `output`: a `&'static str` with the substring at `range` in `input` replaced with
/// `replace_with`.
/// - `removed`: the substring at `range` in `input`.
///
/// # Example
///
/// ```rust
/// use const_format::{str_splice, SplicedStr};
///
/// const OUT: SplicedStr = str_splice!("foo bar baz", 4..=6, "is");
/// assert_eq!(OUT , SplicedStr{output: "foo is baz", removed: "bar"});
///
/// // You can pass `const`ants to this macro, not just literals
/// {
///     const IN: &str = "this is bad";
///     const INDEX: std::ops::RangeFrom<usize> = 8..;
///     const REPLACE_WITH: &str = "... fine";
///     const OUT: SplicedStr = str_splice!(IN, INDEX, REPLACE_WITH);
///     assert_eq!(OUT , SplicedStr{output: "this is ... fine", removed: "bad"});
/// }
/// ```
///
/// ### Invalid index
///
/// Invalid indices cause compilation errors.
///
/// ```compile_fail
/// const_format::str_splice!("foo", 0..10, "");
/// ```
#[cfg_attr(
    feature = "testing",
    doc = r#"
```rust
const_format::str_splice!("foo", 0..3, "");
```

```compile_fail
const_format::str_splice!("foo", 0..usize::MAX, "");
```
"#
)]
///
///
/// [`SplicedStr`]: ./struct.SplicedStr.html
#[macro_export]
macro_rules! str_splice {
    ($string:expr, $index:expr, $insert:expr) => {{
        const P_OSRCTFL4A: $crate::__str_methods::StrReplaceArgs =
            $crate::__str_methods::StrReplaceArgsConv($string, $index, $insert).conv();
        {
            use $crate::__hidden_utils::PtrToRef;
            use $crate::__str_methods::{DecomposedString, SplicedStr, StrReplaceArgs};
            use $crate::pmr::{str, transmute, u8};

            const P: &StrReplaceArgs = &P_OSRCTFL4A;

            type DecompIn = DecomposedString<[u8; P.start], [u8; P.range_len], [u8; P.suffix_len]>;

            type DecompOut =
                DecomposedString<[u8; P.start], [u8; P.insert_len], [u8; P.suffix_len]>;

            const OUT_A: (&DecompOut, &str) = unsafe {
                let input = PtrToRef {
                    ptr: P.str.as_ptr() as *const DecompIn,
                }
                .reff;
                let insert = PtrToRef {
                    ptr: P.insert.as_ptr() as *const [u8; P.insert_len],
                }
                .reff;

                (
                    &DecomposedString {
                        prefix: input.prefix,
                        middle: *insert,
                        suffix: input.suffix,
                    },
                    $crate::__priv_transmute_bytes_to_str!(&input.middle),
                )
            };

            const OUT: SplicedStr = unsafe {
                let output = OUT_A.0 as *const DecompOut as *const [u8; P.out_len];
                SplicedStr {
                    output: $crate::__priv_transmute_raw_bytes_to_str!(output),
                    removed: OUT_A.1,
                }
            };

            OUT
        }
    }};
}

// const _: crate::SplicedStr = str_splice!("hello", 0, "world");
