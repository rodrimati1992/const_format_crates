use crate::{formatting::Formatting, pwrapper::PWrapper};

use arrayvec::ArrayString;

use core::fmt::{Display, Write};

fn count_digits(n: impl Display) -> usize {
    let mut buff = ArrayString::<[u8; 64]>::new();
    write!(buff, "{}", n).unwrap();
    buff.len()
}

macro_rules! number_of_digits_test_case {
    ($val:expr) => {
        assert_eq!(
            PWrapper($val).fmt_len(Formatting::Display),
            count_digits($val)
        );
    };
}

macro_rules! check_number_of_digits_ {
    ($ty:ty) => {{
        let zero: $ty = 0;
        let one: $ty = 1;
        let two: $ty = 2;

        number_of_digits_test_case!(zero);
        number_of_digits_test_case!(one);
        number_of_digits_test_case!(two);

        let mut n: $ty = 10;

        loop {
            number_of_digits_test_case!(n - 1);
            number_of_digits_test_case!(n);
            number_of_digits_test_case!(n + 1);

            match n.checked_mul(10) {
                Some(next) => n = next,
                None => break,
            }
        }

        let max_s2: $ty = <$ty>::MAX - 2;
        let max_s1: $ty = <$ty>::MAX - 1;
        let max_s0: $ty = <$ty>::MAX;

        number_of_digits_test_case!(max_s2);
        number_of_digits_test_case!(max_s1);
        number_of_digits_test_case!(max_s0);
    }};
}

#[test]
fn number_of_digits() {
    check_number_of_digits_!(i8);
    check_number_of_digits_!(u8);
    check_number_of_digits_!(i16);
    check_number_of_digits_!(u16);
    check_number_of_digits_!(i32);
    check_number_of_digits_!(u32);
    check_number_of_digits_!(u64);
    check_number_of_digits_!(i64);
    check_number_of_digits_!(usize);
    check_number_of_digits_!(isize);
    check_number_of_digits_!(u128);
    check_number_of_digits_!(i128);
}
