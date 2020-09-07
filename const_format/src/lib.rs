//! Compile-time string formatting.
//!
//! This crate provides types and macros for formatting strings at compile-time.
//!
//! # Rust versions
//!
//! There are some features that require Rust 1.46.0,
//! and others that require Rust nightly,
//! the sections below describe the features that are available.
//!
//! ### Rust 1.46.0
//!
//! These macros are the only things available in Rust 1.46.0:
//!
//! - [`concatcp`]:
//! Concatenates `integers`, `bool`, and `&str` constants into a `&'static str` constant.
//!
//! - [`formatcp`]:
//! [`format`]-like formatting which takes `integers`, `bool`, and `&str` constants,
//! and emits a `&'static str` constant.
//!
//! ### Rust nightly
//!
//! By enabling the "fmt" feature, you can use a [`std::fmt`]-like API.
//!
//! This requires the nightly compiler because it uses mutable references in const fn,
//! which have not been stabilized as of writing these docs.
//!
//! All the other features of this crate are implemented on top of the [`const_format::fmt`] API:
//!
//! - [`formatc`]:
//! [`format`]-like macro that can format many standard library and user defined types.
//!
//! - [`writec`]:
//! [`write`]-like macro that can format many standard library and user defined types
//! into a type that implements [`WriteMarker`].
//!
//!
//!
//! The "derive" feature enables the [`ConstDebug`] macro,
//! and the "fmt" feature.<br>
//! [`ConstDebug`] derives the [`FormatMarker`] trait,
//! and implements an inherent `const_debug_fmt` method for compile-time debug formatting.
//!
//! # Examples
//!
//! ### Concatenation of primitive types
//!
//! This example works in Rust 1.46.0.
//!
//! ```rust
//! use const_format::concatcp;
//!
//! const NAME: &str = "Bob";
//! const FOO: &str = concatcp!(NAME, ", age ", 21u8,"!");
//!
//! assert_eq!(FOO, "Bob, age 21!");
//! ```
//!
//! ### Formatting primitive types
//!
//! This example works in Rust 1.46.0.
//!
//! ```rust
//! use const_format::formatcp;
//!
//! const NAME: &str = "John";
//!
//! const FOO: &str = formatcp!("{NAME}, age {}!", compute_age(NAME));
//!
//! assert_eq!(FOO, "John, age 24!");
//!
//! # const fn compute_age(s: &str) -> usize { s.len() * 6 }
//!
//! ```
//!
//! ### Formatting custom types
//!
//! This example demonstrates how you can use the [`ConstDebug`] derive macro,
//! and then format the type into a `&'static str` constant.
//!
//! This example requires Rust nightly, and the "derive" feature.
//!
#![cfg_attr(feature = "derive", doc = "```rust")]
#![cfg_attr(not(feature = "derive"), doc = "```ignore")]
//! #![feature(const_mut_refs)]
//!
//! use const_format::{ConstDebug, formatc};
//!
//! #[derive(ConstDebug)]
//! struct Message{
//!     ip: [Octet; 4],
//!     value: &'static str,
//! }
//!
//! #[derive(ConstDebug)]
//! struct Octet(u8);
//!
//! const MSG: Message = Message{
//!     ip: [Octet(127), Octet(0), Octet(0), Octet(1)],
//!     value: "Hello, World!",
//! };
//!
//! const FOO: &str = formatc!("{:?}", MSG);
//!
//! assert_eq!(
//!     FOO,
//!     "Message { ip: [Octet(127), Octet(0), Octet(0), Octet(1)], value: \"Hello, World!\" }"
//! );
//!
//! ```
//!
//! ### Formatted const panics
//!
//! This example demonstrates how you can use the [`assertc_ne`] macro to
//! do compile-time inequality assertions with formatted error messages.
//!
//! This requires the "assert" feature,because as of writing these docs (2020-09-XX),
//! panicking at compile-time requires a nightly feature.
//!
#![cfg_attr(feature = "assert", doc = "```compile_fail")]
#![cfg_attr(not(feature = "assert"), doc = "```ignore")]
//! #![feature(const_mut_refs)]
//!
//! use const_format::{StrWriter, assertc_ne, strwriter_as_str, writec};
//! use const_format::utils::str_eq;
//!
//! macro_rules! check_valid_pizza{
//!     ($user:expr, $topping:expr) => {
//!         assertc_ne!(
//!             $topping,
//!             "pineapple",
//!             "You can't put pineapple on pizza, {}",
//!             $user,
//!         );
//!     }
//! }
//!
//! check_valid_pizza!("John", "salami");
//! check_valid_pizza!("Dave", "sausage");
//! check_valid_pizza!("Bob", "pineapple");
//!
//! # fn main(){}
//! ```
//!
//! This is the compiler output,
//! the first compilation error is there to have an indicator of what assertion failed,
//! and the second is the assertion failure:
//!
//! ```text
//! error: any use of this value will cause an error
//!   --> src/lib.rs:140:1
//!    |
//! 22 | check_valid_pizza!("Bob", "pineapple");
//!    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ exceeded interpreter step limit (see `#[const_eval_limit]`)
//!    |
//!    = note: `#[deny(const_err)]` on by default
//!    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
//!
//! error[E0080]: could not evaluate constant
//!   --> /const_format/src/panicking.rs:31:1
//!    |
//! 31 | panic!();
//!    | ^^^^^^^^^ the evaluated program panicked at '
//! --------------------------------------------------------------------------------
//! module_path: rust_out
//! line: 22
//!
//! assertion failed: LEFT != RIGHT
//!
//!  left: "pineapple"
//! right: "pineapple"
//!
//! You can't put pineapple on pizza, Bob
//! --------------------------------------------------------------------------------
//! ', /const_format/src/panicking.rs:31:1
//!    |
//!    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
//!
//! error: aborting due to 2 previous errors
//!
//! ```
//!
//! <div id="macro-limitations"></div>
//!
//! # Limitations
//!
//! All of the macros from `const_format` have these limitations:
//!
//! - The formatting macros that expand to
//! `&'static str`s can only use constants from concrete types,
//! so while a `Type::<u8>::FOO` argument would be fine,
//! `Type::<T>::FOO` would not be (`T` being a type parameter).
//!
//! - Integer arguments must have a type inferrable from context,
//! [more details in the Integer arguments section](#integer-args).
//!
//! - They cannot be used places that take string literals.
//! So `#[doc = "foobar"]` cannot be replaced with `#[doc = concatcp!("foo", "bar") ]`.
//!
//! <span id="integer-args"></span>
//! ### Integer arguments
//!
//! Integer arguments must have a type inferrable from context.
//! so if you only pass an integer literal it must have a suffix.
//!
//! Example of what does compile:
//!
//! ```rust
//! const N: u32 = 1;
//! assert_eq!(const_format::concatcp!(N + 1, 2 + N), "23");
//!
//! assert_eq!(const_format::concatcp!(2u32, 2 + 1u8, 3u8 + 1), "234");
//! ```
//!
//! Example of what does not compile:
//! ```compile_fail
//! assert_eq!(const_format::concatcp!(1 + 1, 2 + 1), "23");
//! ```
//!
//! # Cargo features
//!
//! - "fmt": Enables the [`std::fmt`]-like API,
//! requires Rust nightly because it uses mutable references in const fn.<br>
//! This feature includes the `formatc`/`writec` formatting macros.
//!
//! - "derive": implies the "fmt" feature,
//! provides the `ConstDebug` derive macro to format user-defined types at compile-time.<br>
//! This implicitly uses the `syn` crate, so clean compiles take a bit longer than without the feature.
//!
//! - "assert": implies the "fmt" feature,
//! enables the assertion macros.<br>
//! This is a separate cargo feature because:
//!     - It uses nightly Rust features  that are less stable than the "fmt" feature does.<br>
//!     - It requires the `std` crate, because `core::panic` requires a string literal argument.
//!
//! - "constant_time_as_str": implies the "fmt" feature.
//! An optimization that requires a few additional nightly features,
//! allowing the `as_bytes_alt` methods and `slice_up_to_len_alt` methods to run
//! in constant time, rather than linear time proportional to the truncated part of the slice.
//!
//!
//! # No-std support
//!
//! `const_format` is unconditionally `#![no_std]`, it can be used anywhere Rust can be used.
//!
//! # Minimum Supported Rust Version
//!
//! `const_format` requires Rust 1.46.0, because it uses looping an branching in const contexts.
//!
//! Features that require newer versions of Rust, or the nightly compiler,
//! need to be explicitly enabled with cargo features.
//!
//!
//! [`assertc_ne`]: ./macro.assertc_ne.html
//!
//! [`concatcp`]: ./macro.concatcp.html
//!
//! [`formatcp`]: ./macro.formatcp.html
//!
//! [`format`]: https://doc.rust-lang.org/std/macro.format.html
//!
//! [`std::fmt`]: https://doc.rust-lang.org/std/fmt/index.html
//!
//! [`const_format::fmt`]: ./fmt/index.html
//!
//! [`formatc`]: ./macro.formatc.html
//!
//! [`writec`]: ./macro.writec.html
//!
//! [`write`]: https://doc.rust-lang.org/std/macro.write.html
//!
//! [`Formatter`]: ./fmt/struct.Formatter.html
//!
//! [`StrWriter`]: ./fmt/struct.StrWriter.html
//!
//! [`ConstDebug`]: ./derive.ConstDebug.html
//!
//! [`FormatMarker`]: ./marker_traits/trait.FormatMarker.html
//!
//! [`WriteMarker`]: ./marker_traits/trait.WriteMarker.html
//!
#![no_std]
#![cfg_attr(feature = "fmt", feature(const_mut_refs))]
#![cfg_attr(feature = "assert", feature(const_panic))]
#![cfg_attr(
    feature = "constant_time_as_str",
    feature(
        const_slice_from_raw_parts,
        const_str_from_utf8_unchecked,
        const_fn_union,
    )
)]
#![deny(rust_2018_idioms)]
// This lint is silly
#![allow(clippy::blacklisted_name)]
// This lint is silly
#![allow(clippy::needless_doctest_main)]
#![deny(clippy::missing_safety_doc)]
#![deny(clippy::shadow_unrelated)]
#![deny(clippy::wildcard_imports)]
#![deny(missing_docs)]

include! {"const_debug_derive.rs"}

// Only used for panicking. Once panicking works without std, I'll remove this.
#[cfg(feature = "assert")]
extern crate std;

#[macro_use]
mod macros;

mod formatting;

#[cfg(feature = "assert")]
mod equality;

#[doc(hidden)]
#[cfg(feature = "assert")]
#[macro_use]
pub mod panicking;

mod pargument;

#[cfg(feature = "fmt")]
pub mod utils;

#[cfg(feature = "fmt")]
pub mod for_examples;

#[cfg(feature = "fmt")]
pub mod marker_traits;

#[cfg(feature = "testing")]
pub mod test_utils;

#[cfg(feature = "fmt")]
#[cfg(feature = "testing")]
#[allow(missing_docs)]
pub mod doctests;

#[cfg(feature = "fmt")]
pub mod fmt;

#[cfg(feature = "fmt")]
#[doc(hidden)]
pub mod msg;

#[cfg_attr(not(feature = "fmt"), doc(hidden))]
pub mod wrapper_types;

#[cfg(feature = "fmt")]
#[doc(no_inline)]
pub use crate::fmt::{Error, Formatter, FormattingFlags, Result, StrWriter, StrWriterMut};

#[cfg(feature = "fmt")]
pub use crate::wrapper_types::ascii_str::AsciiStr;

#[cfg(feature = "fmt")]
pub use crate::wrapper_types::sliced::Sliced;

#[cfg_attr(not(feature = "fmt"), doc(hidden))]
pub use crate::wrapper_types::pwrapper::PWrapper;

#[doc(hidden)]
pub mod pmr {
    pub use const_format_proc_macros::{__concatcp_impl, __formatcp_impl, respan_to};

    #[cfg(feature = "fmt")]
    pub use const_format_proc_macros::{__formatc_if_impl, __formatc_impl, __writec_impl};

    pub use core::{
        cmp::Reverse,
        convert::identity,
        mem::transmute,
        num::Wrapping,
        ops::Range,
        option::Option::{self, None, Some},
        result::Result::{self, Err, Ok},
    };

    #[cfg(feature = "fmt")]
    pub use crate::{
        fmt::{ComputeStrLength, Error, Formatter, StrWriter, StrWriterMut, ToResult},
        marker_traits::{
            FormatMarker, IsAFormatMarker, IsAWriteMarker, IsNotStdKind, IsStdKind, WriteMarker,
        },
    };

    pub use crate::{
        formatting::{
            hex_as_ascii, ForEscaping, Formatting, FormattingFlags, LenAndArray, NumberFormatting,
            StartAndArray, FOR_ESCAPING,
        },
        pargument::{PArgument, PConvWrapper, PVariant},
        wrapper_types::PWrapper,
    };
}

#[cfg(all(test, not(feature = "testing")))]
compile_error! { "tests must be run with the \"testing\" feature" }
