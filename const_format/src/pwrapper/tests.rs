use crate::{formatting::Formatting, pwrapper::PWrapper};

const MAX_POWER: u32 = 38;

macro_rules! check_number_of_digits_ {
    ($ty:ty) => {{
        let arr = generate_numbers();
        for (n, digits) in arr
            .iter()
            .copied()
            .filter(|v| v.0 <= (<$ty>::max_value() as u128))
            .map(|v| (v.0 as $ty, v.1))
        {
            for fmt in [Formatting::Debug, Formatting::Display].iter().copied() {
                assert_eq!(
                    PWrapper(n).fmt_len(fmt) as u32,
                    digits,
                    "\nn:{} ty:{} \n",
                    n,
                    core::any::type_name::<$ty>(),
                );
            }
        }
    }};
}

const ARR_LEN: usize = (MAX_POWER as usize) * 3 + 3;

fn generate_numbers() -> [(u128, u32); ARR_LEN] {
    let ten: u128 = 10;
    let mut out: [(u128, u32); ARR_LEN] = [(0, 0); ARR_LEN];
    out[0] = (0, 1);
    out[1] = (1, 1);
    out[2] = (9, 1);

    for power in 1..MAX_POWER {
        let digits = power + 1;

        let i = power as usize * 3;
        out[i] = (ten.pow(power), digits);
        out[i + 1] = (ten.pow(power) + 1, digits);
        out[i + 2] = (ten.pow(power + 1) - 1, digits);
    }
    out[ARR_LEN - 3] = (ten.pow(MAX_POWER), MAX_POWER + 1);
    out[ARR_LEN - 2] = (ten.pow(MAX_POWER) + 1, MAX_POWER + 1);
    out[ARR_LEN - 1] = (u128::max_value(), MAX_POWER + 1);

    out
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
