const MAX_POWER: u32 = 38;

fn check_number_of_digits<I, N>(iter: I)
where
    I: IntoIterator<Item = (N, u32)>,
    N: PartialEq + fmt::Display + Default + Copy + IntegerExt,
{
    for (n, digits) in iter {
        println!("n:{} digits:{}", n, digits);
        assert_eq!(n.number_of_digits(), digits, " n:{} ", n);
    }
}

macro_rules! check_number_of_digits_ {
    ($ty:ty) => {{
        let arr = generate_numbers();
        check_number_of_digits(
            arr.iter()
                .copied()
                .filter(|v| v.0 <= (<$ty>::max_value() as UMax))
                .map(|v| (v.0 as $ty, v.1)),
        );
    }};
}

const ARR_LEN: usize = MAX_POWER * 3 + 3;

fn generate_numbers() -> [(UMax, u32); ARR_LEN] {
    let ten: UMax = 10;
    let mut out: [(UMax, u32); ARR_LEN] = [(0,0); ARR_LEN];
    out[0] = (0, 1);
    out[1] = (1, 1);
    out[2] = (9, 1);

    for power in 1..MAX_POWER {
        let digits = power + 1;

        let i = power * 3;
        out[i  ] = (ten.pow(power), digits);
        out[i+1] = (ten.pow(power) + 1, digits);
        out[i+2] = (ten.pow(power + 1) - 1, digits);
    }
    out[ARR_LEN-3] = (ten.pow(MAX_POWER), MAX_POWER + 1);
    out[ARR_LEN-2] = (ten.pow(MAX_POWER) + 1, MAX_POWER + 1);
    out[ARR_LEN-1] = (UMax::max_value(), MAX_POWER + 1);

    out
}

#[test]
fn number_of_digits_i8() {
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