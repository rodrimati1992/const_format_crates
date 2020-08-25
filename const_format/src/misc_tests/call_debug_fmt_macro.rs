use crate::fmt::{Error, Formatter, FormattingFlags, StrWriter};

use core::{cmp::Reverse, num::Wrapping};

#[test]
fn all_macro_branches() {
    const fn inner(f: &mut Formatter<'_>) -> Result<(), Error> {
        call_debug_fmt!(slice, [Some(10u8), None], f);
        try_!(f.write_whole_str("\n"));

        call_debug_fmt!(Option, Some(&(0..10)), f);
        try_!(f.write_whole_str("\n"));

        call_debug_fmt!(newtype Wrapping, Wrapping("hello"), f);
        try_!(f.write_whole_str("\n"));

        call_debug_fmt!(newtype Reverse, Reverse(&Some(10u8)), f);
        try_!(f.write_whole_str("\n"));

        call_debug_fmt!(newtype Hello, Hello(false), f);
        try_!(f.write_whole_str("\n"));

        call_debug_fmt!(std, 1000u16, f);
        try_!(f.write_whole_str("\n"));

        call_debug_fmt!(other, TupleStruct(256), f);
        try_!(f.write_whole_str("\n"));

        Ok(())
    }

    let writer: &mut StrWriter = &mut StrWriter::new([0; 1024]);

    inner(&mut writer.make_formatter(FormattingFlags::NEW)).unwrap();

    assert_eq!(
        writer.as_str(),
        "[Some(10), None]\n\
         Some(0..10)\n\
         Wrapping(\"hello\")\n\
         Reverse(Some(10))\n\
         Hello(false)\n\
         1000\n\
         TupleStruct(256)\n\
        ",
    );
}

#[test]
fn returns_error() {
    const fn inner(f: &mut Formatter<'_>) -> Result<(), Error> {
        call_debug_fmt!(slice, [Some(10u8), None], f);
        Ok(())
    }

    let writer: &mut StrWriter = &mut StrWriter::new([0; 4]);

    let err = inner(&mut writer.make_formatter(FormattingFlags::NEW)).unwrap_err();

    assert!(matches!(err, Error::NotEnoughSpace));
}

struct Hello<T>(T);

struct TupleStruct(u32);

impl_fmt! {
    impl TupleStruct;

    const fn const_debug_fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        use crate::wrapper_types::PWrapper;

        let mut f = f.debug_tuple("TupleStruct");
        try_!(PWrapper(self.0).const_debug_fmt(f.field()));
        f.finish()
    }
}
