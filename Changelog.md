This is the changelog,summarising changes in each version(some minor changes may be ommited).

# 0.2 

### 0.2.34

Now all features that used to require nightly only require Rust 1.83.0

Added `"rust_1_83"` feature that enables `"rust_1_64"` feature

Changed `"fmt"` feature to enable `"rust_1_83"` feature

Made many macros forward compatible with inline const patterns(when the `"rust_1_83"` feature is enabled):
- `concatc`
- `concatcp`
- `formatc`
- `formatcp`
- `map_ascii_case`
- `str_get`
- `str_index`
- `str_repeat`
- `str_replace`

Added these macros:
- `str_splice_out`
- `str_split_alt`

### 0.2.32

Breaking change: bumped Minimum Supported Rust Version to Rust 1.57 and changed crate's edition to 2021. This change is motivated by proc-macro2 increasing its MSRV to 1.56.

Changed these items that needed the `"rust_1_51"` feature into always being enabled:
- `map_ascii_case`
- `str_replace`

### 0.2.31

Added a workaround for rustdoc bug (https://github.com/rust-lang/rust/issues/112085).

### 0.2.29

Added lowercase hexadecimal formatting support.
    
Breaking: to add lowercase hexadecimal formatting, this crate changed the uppercase hexadecimal formatter from `{:x}` to `{:X}`


### 0.2.27

Replacing existing features with these:
- `"rust_1_64"`: superceeding the soft-deprecated `"more_str_macros"` feature.
- `"rust_1_51"`: superceeding the soft-deprecated `"const_generics"` feature.
The new features are enabled by the feature they superceede.

Now the `"fmt"` feature enables the `"rust_1_64"` feature.

### 0.2.26 

Added `"more_str_macros"` crate feature.

Added `str_split` macro, conditional on the `"more_str_macros"` feature.

Added `char` pattern support to `str_replace`.

### 0.2.25

Fixed the `clippy::double_parens` (false positive) warning by 
encoding the `&'static str` type annotation some other way.

Made `SplicedStr`, `Formatting`, and `NumberFormatting` derive `Eq` .

### 0.2.24

Fixed error that caused formatting macros not to be usable in statement position.

### 0.2.23

Added type annotations to `concatp`, `concatcp`, `formatc` and `formatcp` macros to help IDEs infer the type.

### 0.2.22


Added the `assertcp`, `assertcp_ne`, and `assertcp_eq` macros under the "assertcp"  feature.

Added `const_eq` methods for `PWrapper<&[char]>` and `PWrapper<Option<char>>`

Added the "assertcp" feature, which enables the `assertcp*` macros.

Aliased "assert" crate feature to "assertc", and removed old name from docs to reduce confusion.

### 0.2.21

Rewrote assertion macros to:
- Have more concise error messages
- Point to all their arguments when the assertion fails
- Resemble std error messages more

### 0.2.19

Added `char` support to all formatting macros.

Added `char`, `&[char]`, and `Option<char>` impls of FormatMarker trait, with debug formatting methods.

Added `Formatter::{write_char, write_char_debug}` methods.

Added `StrWriterMut::{as_str_alt, write_char, write_char_debug}` methods.

Added `StrWriter::{as_str_alt, unsize}` methods.

Deprecated `strwriter_as_str` macro, superceded by `StrWriter::as_str_alt`.

Bumped the minimum required nightly version to 2021-07-05 due to use of const-stabilized `core::str::from_utf8_unchecked`.

### 0.2.18

Fixed potential soundness bug where unions used to do pointer casts were not `#[repr(C)]`

### 0.2.16

Added these macros that act like `str` methods:
- `str_get`
- `str_index`
- `str_repeat`
- `str_replace`
- `str_splice`

Added the `SplicedStr` struct.

### 0.2.15

Added `map_ascii_case` macro to convert the casing style of a `&'static str`.

Added the `Case` enum.

Fixed "constant_time_as_str" crate feature in newer nightlies,
this will break code that uses the feature and hasn't updated the nightly compiler
to a release post mid-july 2021.

### 0.2.14

Fixed a few documentation issues.

Made the `const_format::fmt` API that uses const generics unconditional, since const generics were stabilized in late 2020 and the `fmt` API requires the nightly compiler.

Repurposed the "const_generics" feature to generate less code in the `concatcp` and `formatcp` macros,
by moving some of their implementation to a function that uses const generics.

### 0.2.13

Fixed the assertion macros not to use `std::panic`, using `core::panic` instead, since `core::panic` changed to allow passing a non-literal `&'static str` argument.

### 0.2.11

Fixed the documentation in case that the https://github.com/rust-lang/rust/pull/80243 
rustc pull request is merged.

### 0.2.8

Added minimal const generic support, for use in the added methods.

Added these methods to `StrWriter<[u8; N]>`:
- `r`: for casting it to a `StrWriter<[u8]>`
- `as_mut`: for casting it to a `StrWriterMut`.

Added "const_generics" and "nightly_const_generics" features.

Fixed hygiene bug in assertion macros.

Bumped version number to 0.2.8 .

### 0.2.6

Made the macros in `const_format` usable when the crate is renamed.

Added a `#[cdeb(crate = "foo")]` helper attribute to
pass the path to `const_format` to `ConstDebug`, useful when reexporting the derive macro.

Documented that `writec!(buff, "{foo}")` (where `foo` is a local variable) works,
and is equivelent to `writec!(buff, "{}", foo)`.

### 0.2.5

Added the "assert" cargo feature,
defining the `assertc`/`assertc_eq`/`assertc_ne` macros for 
compile-time assertions with formatting.

Added custom formatting support in the `const_format::fmt`-based formatting macros,
by prefixing any argument with `|identifier|`,
accessing a `Formatter` to format that argument however one wants.

Added `concatc` macro for concatenating std/user-defined types into a `&'static str` constant.

Added `const_format::Result` alias for `std::result::Result<(), const_format::Error>`.

Added `const_format::fmt::ToResult` type for converting  
`()` and `const_format::Result` to `const_format::Result`.

Added `Pwrapper::const_eq` methods for comparing many std types in 
the `assertc_eq`/`assertc_ne` macros.

Added `Pwrapper::const_display_fmt` methods for `NonZero*` types.

Added support for passing `concat!(....)` as the format string.

### 0.2.0

Every single new item added requires Rust nightly to use, with at least the "fmt" cargo feature enabled.

Defined a `core::fmt`-like API with these these types:
`ComputeStrLength`, `DebugList`, `DebugSet`, `DebugStruct`, `DebugTuple`, `Error`, `Formatter`, `FormattingFlags`, `NumberFormatting`, `StrWriter`, `StrWriterMut`, `NoEncoding`, `Utf8Encoding`.

Added `formatc` macro, for formatting std and user-defined types into a `&'static str` constant.

Added `writec` macro, for writing formatted std and user-defined types, 
into a type that implements `WriteMarker`.

Added `marker_traits::FormatMarker` trait, for types that implement const formatting,
with either the `const_debug_fmt`, or `const_display_fmt` inherent methods.

Added `ConstDebug` derive macro, for implementing `FormatMarker`,
and implement the `const_debug_fmt` inherent method.

Added `marker_traits::WriteMarker` trait, for types that can be written into,
defining the `borrow_mutably` and `make_formatter` methods.

Added these type in `marker_traits` module: `IsAFormatMarker`, `IsAStrWriter`, `IsAWriteMarker`, 
`IsArrayKind`, `IsNotAStrWriter`, `IsNotStdKind`, `IsStdKind`

Added hexadecimal and binary formatting to the `formatcp` macro
(also usable in `formatc`, and `writec`)

Defined the `AsciiStr` type, a wrapper type for `&[u8]` slices which are valid ascii,
with an `ascii_str` macro for constructing it at compile-time,
and `wrapper_types::NotAsciiError` returned by the fallible constructor.

Exposed the `PWrapper` type, wrapper for std types to call some methods on them.

Defined the `Sliced` type, to output a slice from a `&str`.

Defined these macros for implementing/doing compile-time formatting:
`call_debug_fmt`, `coerce_to_fmt`, `impl_fmt`

Defined the `strwriter_as_str` macro to cast a `&'static StrWriter` to a `&'static str`

Defined these error handling macros: `try_`, `unwrap`, `unwrap_or_else`

Defined the `for_examples` module with examples of types that implement const formatting.

Defined these utility functions in the `utils` module: 
`slice_up_to_len`, `slice_up_to_len_alt`, `str_eq u8`, `slice_eq `

Fixed error reporting in `formatcp` and `concatcp` macros,
now compiler errors point at the argument that caused an error rather than the whole macro invocation.

Added the "fmt" cargo feature, to enable the `fmt`-like API, and every other thing that depends on it.

Added the "derive" cargo feature, to enable the `ConstDebug` macro.

Added the "constant_time_as_str", to optimize some methods, requires additional nightly features.

Made `syn` an optional dependency, only enabled when the "derive" feature is used.

Added `unicode-xid` dependency.

# 0.1

Created `const_format` crate,
`const_format_proc_macros` crate(implementation detail of `const_format`)

Defined the concatcp macro,
for concatenating constants of primitive types into a `&'static str` constant.

Defined the formatcp macro,
for formatting constants of primitive types into a `&'static str` constant.

Added dependencies: syn with none of the expensive features, quote, proc_macro2

