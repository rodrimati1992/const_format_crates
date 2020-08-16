

#[macro_export]
macro_rules! concatp {
    ($($arg: expr),* $(,)?)=>(
        $crate::concatp!(@with_fmt $(($crate::pmr::Formatting::Display, $arg))* )
    );
    (@with_fmt $(($fmt:expr, $arg: expr))* )=>({
        // The suffix is to avoid name collisions with identifiers in the passed-in expression.
        const CONCATP_NHPMWYD3NJA : (usize, &[$crate::pmr::PArgument]) = {
            let mut len = 0usize;

            let array = [
                $({
                    let arg = $crate::pmr::PConvWrapper($arg).to_pargument($fmt);
                    len += arg.fmt_len;
                    arg
                }),*
            ];

            (len, &{array})
        };

        {
            const ARR_LEN: usize = CONCATP_NHPMWYD3NJA.0;

            const CONCAT_ARR: &$crate::pmr::LenAndArray<[u8; ARR_LEN]> = {
                use $crate::{
                    pmr::PVariant,
                    __write_pvariant,
                };

                let mut out = $crate::pmr::LenAndArray{
                    len: 0,
                    array: [0u8; ARR_LEN],
                };

                let input = CONCATP_NHPMWYD3NJA.1;

                $crate::__for_range!{ outer_i in 0..input.len() =>
                    let current = &input[outer_i];

                    match current.elem {
                        PVariant::Str(s) => __write_pvariant!(str, current, s => out),
                        PVariant::Int(int) => __write_pvariant!(int, current, int => out),
                    }
                }
                &{out}
            };
            const CONCAT_STR: &str = unsafe{
                // This transmute truncates the length of the array to the amound of written bytes.
                let slice =
                    $crate::pmr::Transmute::<&[u8; ARR_LEN], &[u8; CONCAT_ARR.len]>{
                        from: &CONCAT_ARR.array,
                    }.to;

                $crate::pmr::Transmute::<&[u8], &str>{from: slice}.to
            };
            CONCAT_STR
        }
    });
}
