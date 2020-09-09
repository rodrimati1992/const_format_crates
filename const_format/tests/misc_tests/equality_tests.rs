use const_format::coerce_to_fmt;

use core::{
    cmp::Ordering,
    num::{NonZeroU128, NonZeroU8, NonZeroUsize},
    sync::atomic::Ordering as AtomicOrdering,
};

macro_rules! compare_array_case {
    ( $first:expr, $second:expr, $third:expr; $foo:expr, $bar:expr ) => {{
        assert!(!coerce_to_fmt!(&[$first, $second]).const_eq(&[$foo, $bar]));
        assert!(!coerce_to_fmt!(&[$first, $second]).const_eq(&[$first, $foo]));
        assert!(!coerce_to_fmt!(&[$first, $second]).const_eq(&[$first]));
        assert!(!coerce_to_fmt!(&[$first, $second]).const_eq(&[$first, $second, $third]));
        assert!(coerce_to_fmt!(&[$first, $second]).const_eq(&[$first, $second]));
    }};
}

#[test]
fn compare_arrays() {
    compare_array_case!("hello", "world", "cool"; "foo", "bar");

    compare_array_case!(3u8, 5u8, 8u8; 13u8, 21u8);
    compare_array_case!(3u16, 5u16, 8u16; 13u16, 21u16);
    compare_array_case!(3usize, 5usize, 8usize; 13usize, 21usize);

    {
        assert!(!coerce_to_fmt!(&[false, false]).const_eq(&[false, true]));
        assert!(!coerce_to_fmt!(&[false, false]).const_eq(&[true, false]));
        assert!(!coerce_to_fmt!(&[true, true]).const_eq(&[false, true]));
        assert!(!coerce_to_fmt!(&[true, true]).const_eq(&[true, false]));
        assert!(!coerce_to_fmt!(&[false]).const_eq(&[true]));

        assert!(coerce_to_fmt!(&[true, true]).const_eq(&[true, true]));
        assert!(coerce_to_fmt!(&[false, false]).const_eq(&[false, false]));
        assert!(coerce_to_fmt!(&[false]).const_eq(&[false]));
        assert!(coerce_to_fmt!(&[] as &[bool]).const_eq(&[]));
    }
}

macro_rules! compare_option_case {
    ( $ty:ty ; $first:expr, $second:expr ) => {{
        let first: $ty = $first;
        let second: $ty = $second;
        {
            assert!(!coerce_to_fmt!(first).const_eq(&second));
            assert!(!coerce_to_fmt!(second).const_eq(&first));

            assert!(coerce_to_fmt!(first).const_eq(&first));
            assert!(coerce_to_fmt!(second).const_eq(&second));
        }
        {
            assert!(!coerce_to_fmt!(Some(first)).const_eq(&Some(second)));

            assert!(!coerce_to_fmt!(Some(first)).const_eq(&Some(second)));
            assert!(!coerce_to_fmt!(Some(first)).const_eq(&None));
            assert!(!coerce_to_fmt!(None::<$ty>).const_eq(&Some(first)));

            assert!(coerce_to_fmt!(Some(first)).const_eq(&Some(first)));
            assert!(coerce_to_fmt!(None::<$ty>).const_eq(&None));
        }
    }};
}

#[test]
fn compare_options() {
    compare_option_case!(u8; 3, 5);
    compare_option_case!(u128; 3, 5);
    compare_option_case!(usize; 3, 5);

    compare_option_case!(NonZeroU8; NonZeroU8::new(3).unwrap(), NonZeroU8::new(5).unwrap());

    compare_option_case!(NonZeroU128; NonZeroU128::new(3).unwrap(), NonZeroU128::new(5).unwrap());

    compare_option_case!(NonZeroUsize; NonZeroUsize::new(3).unwrap(), NonZeroUsize::new(5).unwrap());

    compare_option_case!(bool; false, true);

    compare_option_case!(&str; "foo", "bar");
}

macro_rules! compare_cases {
    ($($value:expr),* $(,)* ) => ({
        let cases = [$($value,)*];

        for left in cases.iter() {
            for right in cases.iter() {
                assert_eq!(coerce_to_fmt!(left).const_eq(&right), left==right);
            }
        }
    })
}

#[test]
fn enums() {
    compare_cases! {Ordering::Less, Ordering::Equal, Ordering::Greater}
    compare_cases! {
        AtomicOrdering::Relaxed,
        AtomicOrdering::Acquire,
        AtomicOrdering::Release,
        AtomicOrdering::AcqRel,
        AtomicOrdering::SeqCst,
    }
}

#[test]
fn ranges() {
    compare_cases! {0..10, 5..10, 5..15, 0..15}
    compare_cases! {0..=10, 5..=10, 5..=15, 0..=15}
    compare_cases! {0.., 5..}
    compare_cases! {..}
    compare_cases! {..0, ..5}
    compare_cases! {..=0, ..=5}
}
