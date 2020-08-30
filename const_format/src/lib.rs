//! Compile-time string formatting.
//!
//! This crate provides types and macros for formatting strings at compile-time.
//!
//! # Rust versions
//!
//! This crate provides some features that require Rust 1.46.0,
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
//! into a [`Formatter`] or [`StrWriter`].
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
//! This example demonstrates how you can use the StrWriter to format
//! a compile-time panic message.
//!
//! As of writing these docs (2020-08-29), panicking at compile-time requires a
//! nightly feature, and only supports passing a `&'static str` argument,
//! so this only works in the initialization block of `const` items.
//!
#![cfg_attr(feature = "fmt", doc = "```compile_fail")]
#![cfg_attr(not(feature = "fmt"), doc = "```ignore")]
//! #![feature(const_mut_refs)]
//! #![feature(const_panic)]
//!
//! use const_format::{StrWriter, strwriter_as_str, writec};
//! use const_format::utils::str_eq;
//!
//! struct PizzaError;
//!
//! const fn write_message(
//!     buffer: &mut StrWriter,
//!     bought_by: &str,
//!     topping: &str,
//! ) -> Result<(), PizzaError> {
//!     buffer.clear();
//!     let mut writer = buffer.as_mut();
//!     if str_eq(topping, "pineapple") {
//!         let _ = writec!(
//!             writer,
//!             "\n{SEP}\n\nYou can't put pineapple on pizza, {}.\n\n{SEP}\n",
//!             bought_by,
//!             SEP = "----------------------------------------------------------------"
//!         );
//!         return Err(PizzaError);
//!     }
//!     Ok(())
//! }
//!
//! const CAP: usize = 256;
//! // Defined a `const fn` as a workaround for mutable references not
//! // being allowed in `const`ants.
//! const fn message_and_result(
//!     bought_by: &str,
//!     topping: &str,
//! ) -> (StrWriter<[u8; CAP]>, Result<(), PizzaError>) {
//!     let mut buffer = StrWriter::new([0; CAP]);
//!     let res = write_message(&mut buffer, bought_by, topping);
//!     (buffer, res)
//! }
//!
//! const _: () = {
//!     if let (buffer, Err(_)) = message_and_result("Steve", "pineapple") {
//!         let promoted: &'static StrWriter = &{buffer};
//!         let message = strwriter_as_str!(promoted);
//!         panic!(message);
//!     }
//! };
//!
//! ```
//!
//! This is what it prints in rust nightly :
//!
//! ```text
//! error: any use of this value will cause an error
//!   --> src/lib.rs:166:9
//!    |
//! 43 | / const _: () = {
//! 44 | |     if let (buffer, Err(_)) = message_and_result("Steve", "pineapple") {
//! 45 | |         let promoted: &'static StrWriter = &{buffer};
//! 46 | |         let message = strwriter_as_str!(promoted);
//! 47 | |         panic!(message);
//!    | |         ^^^^^^^^^^^^^^^^ the evaluated program panicked at '
//! ----------------------------------------------------------------
//!
//! You can't put pineapple on pizza, Steve.
//!
//! ----------------------------------------------------------------
//! ', src/lib.rs:47:9
//! 48 | |     }
//! 49 | | };
//!    | |__-
//!    |
//!    = note: `#[deny(const_err)]` on by default
//!    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
//!
//! ```
//!
//! <div id="macro-limitations"></div>
//!
//! # Limitations
//!
//! All of the macros from `const_format` have these limitations:
//!
//! - The macros that expand to `&'static str`s can only use constants of concrete types,
//! so while `Type::<u8>::FOO` is fine,`Type::<T>::FOO` is not (`T` being a type parameter).
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
//! "fmt": Enables the [`std::fmt`]-like API,
//! requires Rust nightly becauase it uses mutable references in const fn.
//!
//! "derive": implies the "fmt" feature,
//! provides the `ConstDebug` derive macro to format user-defined types at compile-time.
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
//!
#![no_std]
#![cfg_attr(feature = "fmt", feature(const_mut_refs))]
#![cfg_attr(
    feature = "constant_time_as_str",
    feature(
        const_slice_from_raw_parts,
        const_str_from_utf8_unchecked,
        const_fn_union
    )
)]

extern crate self as const_format;

include! {"const_debug_derive.rs"}

#[macro_use]
mod macros;

mod formatting;

mod pargument;

pub mod utils;

#[cfg(feature = "fmt")]
pub mod for_examples;

#[cfg(feature = "fmt")]
pub mod marker_traits;

#[cfg(test)]
mod misc_tests;

#[cfg(test)]
mod test_utils;

#[cfg(feature = "fmt")]
#[cfg(feature = "testing")]
pub mod doctests;

#[cfg(feature = "fmt")]
pub mod fmt;

#[cfg(feature = "fmt")]
#[doc(hidden)]
pub mod msg;

pub mod wrapper_types;

#[cfg(feature = "fmt")]
#[doc(no_inline)]
pub use crate::fmt::{Error, Formatter, FormattingFlags, StrWriter, StrWriterMut};

#[cfg(feature = "fmt")]
pub use crate::wrapper_types::ascii_str::AsciiStr;

#[cfg(feature = "fmt")]
pub use crate::wrapper_types::sliced::Sliced;

pub use crate::wrapper_types::pwrapper::PWrapper;

#[doc(hidden)]
pub mod pmr {
    pub use const_format_proc_macros::__formatcp_impl;

    #[cfg(feature = "fmt")]
    pub use const_format_proc_macros::{__formatc_impl, __writec_impl};

    pub use core::{
        cmp::Reverse,
        convert::identity,
        mem::transmute,
        num::Wrapping,
        ops::Range,
        option::Option::{None, Some},
        result::Result::{self, Err, Ok},
    };

    #[cfg(feature = "fmt")]
    pub use crate::{
        fmt::{ComputeStrLength, Error, Formatter, StrWriter, StrWriterMut},
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
