use crate::{
    fmt::{ComputeStrLength, Error, FormattingFlags, StrWriter},
    wrapper_types::PWrapper,
};

use core::{
    cmp::Ordering,
    marker::{PhantomData, PhantomPinned},
    num::NonZeroU8,
    ops::{Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive},
    ptr::NonNull,
    sync::atomic::Ordering as AtomicOrdering,
};

const FLAGS: &[FormattingFlags] = &[
    FormattingFlags::NEW,
    FormattingFlags::NEW.set_alternate(true).set_hexadecimal(),
];

macro_rules! test_fmt {
    (
        $Ty:ty;
        $( ($value:expr, $expected_debug:expr, $expected_hex:expr $(,)? )  )+
    ) => ({
        const fn inner(
            this: &PWrapper<$Ty>,
            writer: &mut StrWriter,
            flags: FormattingFlags,
        ) -> Result<usize, Error> {
            try_!(this.const_debug_fmt(&mut writer.make_formatter(flags)));

            let mut str_len = ComputeStrLength::new();
            try_!(this.const_debug_fmt(&mut str_len.make_formatter(flags)));

            Ok(str_len.len())
        }

        fn test_case(this: &PWrapper<$Ty>, expected: &str, expected_hex: &str) {
            let writer: &mut StrWriter = &mut StrWriter::new([0; 256]);

            for (&expected, &flag) in [expected,expected_hex].iter().zip(FLAGS.iter()) {
                writer.clear();
                let len = inner(this, writer, flag).unwrap();

                assert_eq!(writer.as_str(), expected);
                assert_eq!(writer.len(), len, "{}", writer.as_str());
            }
        }

        $({
            test_case( &coerce_to_fmt!(&$value) , $expected_debug, $expected_hex);
        })*
    });
}

#[test]
fn array_impls() {
    test_fmt! {&[u8];
        (
            [8u8, 13, 21, 34],
            "[8, 13, 21, 34]",
            "[\n    0x8,\n    0xD,\n    0x15,\n    0x22,\n]",
        )
    }
    test_fmt! {&[&str];
        (
            ["foo\n", "bar\t"],
            "[\"foo\\n\", \"bar\\t\"]",
            "[\n    \"foo\\n\",\n    \"bar\\t\",\n]"
        )
    }
}

#[test]
fn range_impls() {
    test_fmt! {Range<usize>; (11..64, "11..64", "0xB..0x40") }
    test_fmt! {RangeFrom<usize>; (11.., "11..", "0xB..") }
    test_fmt! {RangeFull; (.., "..", "..") }
    test_fmt! {RangeInclusive<usize>; (11..=64, "11..=64", "0xB..=0x40") }
    test_fmt! {RangeTo<usize>; (..64, "..64", "..0x40") }
    test_fmt! {RangeToInclusive<usize>; (..=64, "..=64", "..=0x40") }
}

#[test]
fn options() {
    test_fmt! {Option<&str>;
        (None::<&str>, "None", "None")
        (Some("hello\n"), "Some(\"hello\\n\")", "Some(\n    \"hello\\n\",\n)")
    }
    test_fmt! {Option<u8>;
        (None::<u8>, "None", "None")
        (Some(10u8), "Some(10)", "Some(\n    0xA,\n)")
    }
    test_fmt! {Option<bool>;
        (None::<bool>, "None", "None")
        (Some(false), "Some(false)", "Some(\n    false,\n)")
        (Some(true), "Some(true)", "Some(\n    true,\n)")
    }
    test_fmt! {Option<NonZeroU8>;
        (None::<NonZeroU8>, "None", "None")
        (NonZeroU8::new(10), "Some(10)", "Some(\n    0xA,\n)")
    }
    test_fmt! {Option<NonNull<u8>>;
        (None::<NonNull<u8>>, "None", "None")
        (Some(NonNull::<u8>::dangling()), "Some(<pointer>)", "Some(\n    <pointer>,\n)")
    }
}

#[test]
fn pointers() {
    test_fmt! {*const u8; (core::ptr::null(), "<pointer>", "<pointer>") }
    test_fmt! {*mut u8; (core::ptr::null_mut(), "<pointer>", "<pointer>") }
    test_fmt! {NonNull<u8>; (NonNull::dangling(), "<pointer>", "<pointer>") }
}

#[test]
fn marker() {
    test_fmt! {PhantomData<()>; (PhantomData, "PhantomData", "PhantomData") }
    test_fmt! {PhantomPinned; (PhantomPinned, "PhantomPinned", "PhantomPinned") }
    test_fmt! {(); ((), "()", "()") }
}

#[test]
fn miscelaneous_enums() {
    test_fmt! {
        Ordering;
        (Ordering::Less, "Less", "Less")
        (Ordering::Equal, "Equal", "Equal")
        (Ordering::Greater, "Greater", "Greater")
    }
    test_fmt! {
        AtomicOrdering;
        (AtomicOrdering::Relaxed, "Relaxed", "Relaxed")
        (AtomicOrdering::Release, "Release", "Release")
    }
}
