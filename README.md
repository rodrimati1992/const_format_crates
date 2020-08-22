[![Build Status](https://travis-ci.org/rodrimati1992/const_format_crates.svg?branch=master)](https://travis-ci.org/rodrimati1992/const_format_crates)
[![crates-io](https://img.shields.io/crates/v/const_format.svg)](https://crates.io/crates/const_format)
[![api-docs](https://docs.rs/const_format/badge.svg)](https://docs.rs/const_format/*)


Macros for concatenating and formatting constants into `&'static str` constants.

# Examples

### Concatenation

```rust
use const_format::concatcp;

const NAME: &str = "Bob";
const FOO: &str = concatcp!(NAME, ", age ", 21u8,"!");

assert_eq!(FOO, "Bob, age 21!");
```

### Formatting

```rust
use const_format::formatcp;

const NAME: &str = "John";

const FOO: &str = formatcp!("{NAME}, age {}!", compute_age(NAME));

assert_eq!(FOO, "John, age 24!");

const fn compute_age(s: &str) -> usize { s.len() * 6 }

```


<div id="macro-limitations"></div>

# Limitations

All of the macros from `const_format` have these limitations:

- They cannot take constants that *use* generic parameters,
so while `Type::<u8>::FOO` is fine `Type::<T>::FOO` is not (`T` being a type parameter).

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

# Cargo features

None yet.

# No-std support

`const_format` is unconditionally `#![no_std]`, so it can be used anywhere Rust can be used.

# Minimum Supported Rust Version

`const_format` requires Rust 1.46.0, because it uses looping an branching in const contexts.

Features that require newer versions of Rust, or the nightly compiler,
need to be explicitly enabled with cargo features.

