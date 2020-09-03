use const_format::{
    coerce_to_fmt,
    fmt::{Error, Formatter, FormattingFlags, StrWriter},
    impl_fmt, try_,
    wrapper_types::PWrapper,
    ConstDebug,
};

use core::marker::PhantomData;

pub struct Bar(pub u32);

impl_fmt! {
    impl[] Bar;

    const fn const_debug_fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let mut f = f.debug_tuple("Bar");
        try_!(PWrapper(self.0).const_debug_fmt(f.field()));
        f.finish()
    }
}

const fn fmt_bar_in_hex(this: &Bar, f: &mut Formatter<'_>) -> Result<(), Error> {
    let flags = f.flags().set_hexadecimal();
    this.const_debug_fmt(&mut f.make_formatter(flags))
}

/////////////////////////////////////////////////////////////////

pub struct DisplayWrapper<'a, T>(pub &'a T);

impl_fmt! {
    impl DisplayWrapper<'_, &str>;

    const fn const_debug_fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        PWrapper(*self.0).const_display_fmt(f)
    }
}

/////////////////////////////////////////////////////////////////

macro_rules! display_fmt {
    ($reference:expr, $formatter:expr) => {
        coerce_to_fmt!($reference).const_display_fmt($formatter)
    };
}

/////////////////////////////////////////////////////////////////

mod type_named_option {
    use super::*;

    pub struct NotDebug;

    pub struct Option<T>(pub PhantomData<T>);

    impl_fmt! {
        impl[T] Option<T>;

        const fn const_debug_fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
            f.write_str("NotAStdOption")
        }
    }

    // This type tests that ``
    #[derive(ConstDebug)]
    pub(super) struct WrapsNamedOption {
        pub(super) opt_a: self::Option<NotDebug>,

        #[cdeb(is_a(non_std))]
        pub(super) opt_b: Option<NotDebug>,

        #[cdeb(is_a(not_std))]
        pub(super) opt_c: Option<NotDebug>,
    }
}

use self::type_named_option::WrapsNamedOption;

/////////////////////////////////////////////////////////////////

pub mod path {
    pub mod to {
        pub use super::super::Bar;
    }
}

// Defining the `alloc` and `std` modules to avoid having to import them here
// (I'll probably use the real ones in an example)
pub mod alloc {
    pub mod option {
        pub use core::option::Option;
    }
}
pub mod std {
    pub mod option {
        pub use core::option::Option;
    }
}

#[derive(ConstDebug)]
struct Automatic {
    slice_a: &'static [Bar],
    slice_b: &'static &'static [Bar],
    array_a: [Bar; 2],
    array_b: &'static [Bar; 2],
    array_c: &'static &'static [Bar; 2],
    option_a: Option<Bar>,
    option_b: core::option::Option<Bar>,
    option_c: alloc::option::Option<Bar>,
    option_d: std::option::Option<Bar>,
}

type BarSlice = [Bar];
type BarArray = [Bar; 2];
type BarOption = Option<Bar>;

#[derive(ConstDebug)]
struct Manual {
    #[cdeb(is_a(slice))]
    slice_a: &'static BarSlice,

    #[cdeb(is_a(slice))]
    slice_b: &'static &'static BarSlice,

    #[cdeb(is_a(array))]
    array_a: BarArray,

    #[cdeb(is_a(array))]
    array_b: &'static BarArray,

    #[cdeb(is_a(array))]
    array_c: &'static &'static BarArray,

    #[cdeb(is_a(Option))]
    option_a: BarOption,

    #[cdeb(is_a(option))]
    option_b: BarOption,

    #[cdeb(is_a(newtype))]
    newtype: path::to::Bar,

    // This is formatted as a hexadecimal integer
    #[cdeb(with = "fmt_bar_in_hex")]
    with_fn: path::to::Bar,

    #[cdeb(with_wrapper = "DisplayWrapper")]
    with_wrapper: &'static str,

    #[cdeb(with_macro = "display_fmt")]
    with_macro: &'static str,
}

#[test]
fn automatic_type_detection() {
    const fn inner(f: &mut Formatter<'_>) -> Result<(), Error> {
        let auto = Automatic {
            slice_a: &[Bar(3)],
            slice_b: &(&[Bar(5)] as &[_]),
            array_a: [Bar(1000), Bar(8)],
            array_b: &[Bar(2000), Bar(8)],
            array_c: &&[Bar(3000), Bar(8)],
            option_a: Some(Bar(13)),
            option_b: Some(Bar(21)),
            option_c: Some(Bar(34)),
            option_d: Some(Bar(55)),
        };

        try_!(auto.const_debug_fmt(f));

        Ok(())
    }

    let writer: &mut StrWriter = &mut StrWriter::new([0; 1024]);

    inner(&mut writer.make_formatter(FormattingFlags::NEW)).unwrap();

    assert_eq!(
        writer.as_str(),
        "\
            Automatic { \
                slice_a: [Bar(3)], \
                slice_b: [Bar(5)], \
                array_a: [Bar(1000), Bar(8)], \
                array_b: [Bar(2000), Bar(8)], \
                array_c: [Bar(3000), Bar(8)], \
                option_a: Some(Bar(13)), \
                option_b: Some(Bar(21)), \
                option_c: Some(Bar(34)), \
                option_d: Some(Bar(55)) \
            }\
        "
    );
}

#[test]
fn manual_type_detection() {
    const fn inner(f: &mut Formatter<'_>) -> Result<(), Error> {
        let auto = Manual {
            slice_a: &[Bar(3)],
            slice_b: &(&[Bar(5)] as &[_]),
            array_a: [Bar(1000), Bar(8)],
            array_b: &[Bar(2000), Bar(8)],
            array_c: &&[Bar(3000), Bar(8)],
            option_a: Some(Bar(13)),
            option_b: Some(Bar(21)),
            newtype: Bar(34),
            with_fn: Bar(55),
            with_wrapper: "eighty nine",
            with_macro: "-144-",
        };

        try_!(auto.const_debug_fmt(f));

        Ok(())
    }

    let writer: &mut StrWriter = &mut StrWriter::new([0; 1024]);

    inner(&mut writer.make_formatter(FormattingFlags::NEW)).unwrap();

    assert_eq!(
        writer.as_str(),
        "\
            Manual { \
                slice_a: [Bar(3)], \
                slice_b: [Bar(5)], \
                array_a: [Bar(1000), Bar(8)], \
                array_b: [Bar(2000), Bar(8)], \
                array_c: [Bar(3000), Bar(8)], \
                option_a: Some(Bar(13)), \
                option_b: Some(Bar(21)), \
                newtype: Bar(34), \
                with_fn: Bar(37), \
                with_wrapper: eighty nine, \
                with_macro: -144- \
            }\
        "
    );
}

#[test]
fn opting_out_of_std() {
    const fn inner(f: &mut Formatter<'_>) -> Result<(), Error> {
        let wraps_opt = WrapsNamedOption {
            opt_a: type_named_option::Option(PhantomData),
            opt_b: type_named_option::Option(PhantomData),
            opt_c: type_named_option::Option(PhantomData),
        };
        try_!(wraps_opt.const_debug_fmt(f));

        Ok(())
    }

    let writer: &mut StrWriter = &mut StrWriter::new([0; 1024]);

    inner(&mut writer.make_formatter(FormattingFlags::NEW)).unwrap();

    assert_eq!(
        writer.as_str(),
        "\
            WrapsNamedOption { \
                opt_a: NotAStdOption, \
                opt_b: NotAStdOption, \
                opt_c: NotAStdOption \
            }\
        "
    );
}
