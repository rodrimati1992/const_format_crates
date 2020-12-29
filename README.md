

[![Build Status](https://travis-ci.org/rodrimati1992/const_format_crates.svg?branch=master)](https://travis-ci.org/rodrimati1992/const_format_crates)
[![crates-io](https://img.shields.io/crates/v/const_format.svg)](https://crates.io/crates/const_format)
[![api-docs](https://docs.rs/const_format/badge.svg)](https://docs.rs/const_format/*)


Compile-time string formatting.

This crate provides types and macros for formatting strings at compile-time.

# Rust versions

There are some features that require Rust 1.46.0, and others that require Rust nightly,
the sections below describe the features that are available.

### Rust 1.46.0

These macros are the only things available in Rust 1.46.0:

- [`concatcp`]:
Concatenates `integers`, `bool`, and `&str` constants into a `&'static str` constant.

- [`formatcp`]:
[`format`]-like formatting which takes `integers`, `bool`, and `&str` constants,
and emits a `&'static str` constant.

### Rust nightly

By enabling the "fmt" feature, you can use a [`std::fmt`]-like API.

This requires the nightly compiler because it uses mutable references in const fn,
which have not been stabilized as of writing these docs.

All the other features of this crate are implemented on top of the [`const_format::fmt`] API:

- [`concatc`]:
Concatenates many standard library and user defined types into a `&'static str` constant.

- [`formatc`]:
[`format`]-like macro that can format many standard library and user defined types into 
a `&'static str` constant.

- [`writec`]:
[`write`]-like macro that can format many standard library and user defined types
into a type that implements [`WriteMarker`].

The "derive" feature enables the [`ConstDebug`] macro, and the "fmt" feature.<br>
[`ConstDebug`] derives the [`FormatMarker`] trait,
and implements an inherent `const_debug_fmt` method for compile-time debug formatting.

The "assert" feature enables the [`assertc`], [`assertc_eq`], [`assertc_ne`] macros,
and the "fmt" feature.<br>
These macros are like the standard library assert macros, but evaluated at compile-time.

# Examples

### Concatenation of primitive types

This example works in Rust 1.46.0.

```rust
use const_format::concatcp;

const NAME: &str = "Bob";
const FOO: &str = concatcp!(NAME, ", age ", 21u8,"!");

assert_eq!(FOO, "Bob, age 21!");
```

### Formatting primitive types

This example works in Rust 1.46.0.

```rust
use const_format::formatcp;

const NAME: &str = "John";

const FOO: &str = formatcp!("{NAME}, age {}!", compute_age(NAME));

assert_eq!(FOO, "John, age 24!");

const fn compute_age(s: &str) -> usize { s.len() * 6 }

```

### Formatting custom types

This example demonstrates how you can use the [`ConstDebug`] derive macro,
and then format the type into a `&'static str` constant.

This example requires Rust nightly, and the "derive" feature.


```rust
    #![feature(const_mut_refs)]

use const_format::{ConstDebug, formatc};

#[derive(ConstDebug)]
struct Message{
    ip: [Octet; 4],
    value: &'static str,
}

#[derive(ConstDebug)]
struct Octet(u8);

const MSG: Message = Message{
    ip: [Octet(127), Octet(0), Octet(0), Octet(1)],
    value: "Hello, World!",
};

const FOO: &str = formatc!("{:?}", MSG);

assert_eq!(
    FOO,
    "Message { ip: [Octet(127), Octet(0), Octet(0), Octet(1)], value: \"Hello, World!\" }"
);

```



### Formatted const panics

This example demonstrates how you can use the [`assertc_ne`] macro to
do compile-time inequality assertions with formatted error messages.

This requires the "assert" feature,because as of writing these docs (2020-09-XX),
panicking at compile-time requires a nightly feature.

```rust
#![feature(const_mut_refs)]

use const_format::{StrWriter, assertc_ne, strwriter_as_str, writec};
use const_format::utils::str_eq;

macro_rules! check_valid_pizza{
    ($user:expr, $topping:expr) => {
        assertc_ne!(
            $topping,
            "pineapple",
            "You can't put pineapple on pizza, {}",
            $user,
        );
    }
}

check_valid_pizza!("John", "salami");
check_valid_pizza!("Dave", "sausage");
check_valid_pizza!("Bob", "pineapple");

# fn main(){}
```

This is the compiler output,
the first compilation error is there to have an indicator of what assertion failed,
and the second is the assertion failure:

```text
error: any use of this value will cause an error
  --> src/lib.rs:140:1
   |
22 | check_valid_pizza!("Bob", "pineapple");
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ exceeded interpreter step limit (see `#[const_eval_limit]`)
   |
   = note: `#[deny(const_err)]` on by default
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0080]: could not evaluate constant
  --> /const_format/src/panicking.rs:32:5
   |
32 |     .
   |     ^ the evaluated program panicked at '
--------------------------------------------------------------------------------
module_path: rust_out
line: 22

assertion failed: LEFT != RIGHT

 left: "pineapple"
right: "pineapple"

You can't put pineapple on pizza, Bob
--------------------------------------------------------------------------------
', /const_format/src/panicking.rs:31:1
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to 2 previous errors

```




<div id="macro-limitations"></div>

# Limitations

All of the macros from `const_format` have these limitations:

- The formatting macros that expand to
`&'static str`s can only use constants from concrete types,
so while a `Type::<u8>::FOO` argument would be fine,
`Type::<T>::FOO` would not be (`T` being a type parameter).

- Integer arguments must have a type inferrable from context,
[more details in the Integer arguments section](#integer-args).

- They cannot be used places that take string literals.
So `#[doc = "foobar"]` cannot be replaced with `#[doc = concatcp!("foo", "bar") ]`.

<span id="integer-args"></span>
### Integer arguments

Integer arguments must have a type inferrable from context.
so if you only pass an integer literal it must have a suffix.

Example of what does compile:

```rust
const N: u32 = 1;
assert_eq!(const_format::concatcp!(N + 1, 2 + N), "23");

assert_eq!(const_format::concatcp!(2u32, 2 + 1u8, 3u8 + 1), "234");
```

Example of what does not compile:

```compile_fail
assert_eq!(const_format::concatcp!(1 + 1, 2 + 1), "23");
```
# Plans

None right now.

# Renaming crate

All function-like macros from `const_format` can be used when the crate is renamed.

The [`ConstDebug`] derive macro has the `#[cdeb(crate = "foo::bar")]` attribute to 
tell it where to find the `const_format` crate.

Example of renaming the `const_format` crate in the Cargo.toml file:
```toml
cfmt = {version = "0.*", package = "const_format"}
```

# Cargo features

- "fmt": Enables the [`std::fmt`]-like API,
requires Rust nightly because it uses mutable references in const fn.<br>
This feature includes the [`formatc`]/[`writec`] formatting macros.

- "derive": implies the "fmt" feature,
provides the [`ConstDebug`] derive macro to format user-defined types at compile-time.<br>
This implicitly uses the `syn` crate, so clean compiles take a bit longer than without the feature.

- "assert": implies the "fmt" feature,
enables the assertion macros.<br>
This is a separate cargo feature because:
    - It uses nightly Rust features that are less stable than the "fmt" feature does.<br>
    - It requires the `std` crate, because `core::panic` requires a string literal argument.

- "constant_time_as_str": implies the "fmt" feature.
An optimization that requires a few additional nightly features,
allowing the `as_bytes_alt` methods and `slice_up_to_len_alt` methods to run 
in constant time, rather than linear time proportional to the truncated part of the slice.

- "const_generics":
Enables impls that use const generics, currently only used for ergonomics.
Use this when const generics are usable in stable Rust.

- "nightly_const_generics":
Enables impls that use const generics, currently only used for ergonomics.
This requires a nightly Rust compiler.


# No-std support

`const_format` is `#![no_std]`, it can be used anywhere Rust can be used.

Caveat: The opt-in "assert" feature uses the `std::panic` macro to panic,
as of 2020-09-06 `core::panic` requires the argument to be a literal.

# Minimum Supported Rust Version

`const_format` requires Rust 1.46.0, because it uses looping an branching in const contexts.

Features that require newer versions of Rust, or the nightly compiler,
need to be explicitly enabled with cargo features.


[`assertc`]: https://docs.rs/const_format/0.2.*/const_format/macro.assertc.html

[`assertc_eq`]: https://docs.rs/const_format/0.2.*/const_format/macro.assertc_eq.html

[`assertc_ne`]: https://docs.rs/const_format/0.2.*/const_format/macro.assertc_ne.html

[`concatcp`]: https://docs.rs/const_format/0.2.*/const_format/macro.concatcp.html

[`formatcp`]: https://docs.rs/const_format/0.2.*/const_format/macro.formatcp.html

[`format`]: https://doc.rust-lang.org/std/macro.format.html

[`std::fmt`]: https://doc.rust-lang.org/std/fmt/index.html

[`const_format::fmt`]: https://docs.rs/const_format/0.2.*/const_format/fmt/index.html

[`concatc`]: https://docs.rs/const_format/0.2.*/const_format/macro.concatc.html

[`formatc`]: https://docs.rs/const_format/0.2.*/const_format/macro.formatc.html

[`writec`]: https://docs.rs/const_format/0.2.*/const_format/macro.writec.html

[`write`]: https://doc.rust-lang.org/std/macro.write.html

[`Formatter`]: https://docs.rs/const_format/0.2.*/const_format/fmt/struct.Formatter.html

[`StrWriter`]: https://docs.rs/const_format/0.2.*/const_format/fmt/struct.StrWriter.html

[`ConstDebug`]: https://docs.rs/const_format/0.2.*/const_format/derive.ConstDebug.html

[`FormatMarker`]: https://docs.rs/const_format/0.2.*/const_format/marker_traits/trait.FormatMarker.html

[`WriteMarker`]: https://docs.rs/const_format/0.2.*/const_format/marker_traits/trait.WriteMarker.html

