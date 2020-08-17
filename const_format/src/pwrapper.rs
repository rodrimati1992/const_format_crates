use crate::{
    formatting::{is_escaped_simple, Formatting, StartAndArray},
    pargument::Integer,
};

#[cfg(test)]
mod tests;

#[derive(Copy, Clone)]
pub struct PWrapper<T>(pub T);

macro_rules! impl_number_of_digits {
    (num number_of_digits;delegate $n:ident $len:ident)=>{
        $n.number_of_digits()
    };
    (num number_of_digits;128 $n:ident $len:ident)=>{{
        if $n >= 1_0000_0000_0000_0000{$n /= 1_0000_0000_0000_0000; $len += 16;}
        impl_number_of_digits!(num number_of_digits;64 $n $len)
    }};
    (num number_of_digits;64 $n:ident $len:ident)=>{{
        if $n >= 1_0000_0000_0000{$n /= 1_0000_0000_0000; $len += 12;}
        impl_number_of_digits!(num number_of_digits;32 $n $len)
    }};
    (num number_of_digits;32 $n:ident $len:ident)=>{{
        if $n >= 1_0000_0000{$n /= 100_000_000; $len += 8;}
        impl_number_of_digits!(num number_of_digits;16 $n $len)
    }};
    (num number_of_digits;16 $n:ident $len:ident)=>{{
        if $n >= 1_0000{$n /= 1_0000; $len += 4;}
        impl_number_of_digits!(num number_of_digits;8 $n $len)
    }};
    (num number_of_digits;8 $n:ident $len:ident)=>{{
        if $n >= 100{$n /= 100; $len += 2;}
        if $n >= 10{            $len += 1;}
        $len
    }};

    (impl_either;
        signed,
        ($This:ty, $Unsigned:ty),
        $bits:tt ,
    )=>{
        impl PWrapper<$This> {
            #[allow(unused_mut,unused_variables)]
            pub const fn fmt_len(self, _: Formatting)-> usize {
                let mut n = self.0.wrapping_abs() as $Unsigned;
                let mut len = 1 + (self.0 < 0) as usize;
                impl_number_of_digits!(num number_of_digits;$bits n len)
            }
        }
    };
    (impl_either;
        unsigned,
        ($This:ty, $Unsigned:ty),
        $bits:tt ,
    )=>{
        impl PWrapper<$This> {
            #[allow(unused_mut,unused_variables)]
            pub const fn fmt_len(self, _: Formatting)-> usize {
                let mut n = self.0;
                let mut len = 1usize;
                impl_number_of_digits!(num number_of_digits;$bits n len)
            }
        }
    };
}

macro_rules! int_to_array_impls {
    (
        $( ($signedness:ident, $bits:tt, ($Int:ty, $Signed:ty), $array_cap:expr) )*
    ) => (
        $(
            impl_number_of_digits!{
                impl_either;
                $signedness,
                ($Int, $Signed),
                $bits,
            }

            impl PWrapper<$Int> {
                const ARR_CAP: usize = $array_cap;
                int_to_array_impls!{@inner $signedness, $bits, $Int}
            }
        )*
    );
    (@inner $signedness:ident, $bits:tt, $int_type:ty)=>{
        pub const fn to_start_array(
            mut self: Self,
            _fmt: Formatting,
        ) -> StartAndArray<[u8; Self::ARR_CAP]> {
            let mut out = StartAndArray {
                start: Self::ARR_CAP,
                array: [0u8; Self::ARR_CAP],
            };

            loop {
                out.start-=1;
                let digit = (self.0 as u8) % 10;
                out.array[out.start] = b'0' + digit;
                self.0/=10;
                if self.0 == 0 { break }
            }

            out
        }
    };
}

int_to_array_impls! {
    (signed  , 8, (i8, u8), 4)
    (signed  , 16, (i16, u16), 6)
    (signed  , 32, (i32, u32), 11)
    (signed  , 64, (i64, u64), 20)
    (signed  , 128, (i128, u128), 40)
    (unsigned, 8, (u8, u8), 3)
    (unsigned, 16, (u16, u16), 5)
    (unsigned, 32, (u32, u32), 10)
    (unsigned, 64, (u64, u64), 20)
    (unsigned, 128, (u128, u128), 40)
}

#[cfg(target_pointer_width = "16")]
type UWord = u16;
#[cfg(target_pointer_width = "32")]
type UWord = u32;
#[cfg(target_pointer_width = "64")]
type UWord = u64;
#[cfg(target_pointer_width = "128")]
type UWord = u128;

#[cfg(target_pointer_width = "16")]
type IWord = u16;
#[cfg(target_pointer_width = "32")]
type IWord = u32;
#[cfg(target_pointer_width = "64")]
type IWord = u64;
#[cfg(target_pointer_width = "128")]
type IWord = u128;

impl PWrapper<usize> {
    #[inline(always)]
    pub const fn to_start_array(
        self,
        fmt: Formatting,
    ) -> StartAndArray<[u8; PWrapper::<IWord>::ARR_CAP]> {
        PWrapper(self.0 as UWord).to_start_array(fmt)
    }

    #[inline(always)]
    pub const fn fmt_len(self, fmt: Formatting) -> usize {
        PWrapper(self.0 as UWord).fmt_len(fmt)
    }
}

impl PWrapper<isize> {
    #[inline(always)]
    pub const fn to_start_array(
        self,
        fmt: Formatting,
    ) -> StartAndArray<[u8; PWrapper::<IWord>::ARR_CAP]> {
        PWrapper(self.0 as IWord).to_start_array(fmt)
    }
    #[inline(always)]
    pub const fn fmt_len(self, fmt: Formatting) -> usize {
        PWrapper(self.0 as IWord).fmt_len(fmt)
    }
}

impl PWrapper<Integer> {
    #[inline(always)]
    pub const fn to_start_array(
        self,
        fmt: Formatting,
    ) -> StartAndArray<[u8; PWrapper::<u128>::ARR_CAP]> {
        if self.0.is_negative {
            PWrapper((self.0.unsigned as i128).wrapping_neg()).to_start_array(fmt)
        } else {
            PWrapper(self.0.unsigned).to_start_array(fmt)
        }
    }
}

impl PWrapper<&'static str> {
    #[inline(always)]
    pub const fn fmt_len(self, fmt: Formatting) -> usize {
        let mut sum = self.0.len();
        let bytes = self.0.as_bytes();
        if !fmt.is_display() {
            __for_range! {i in 0..self.0.len() =>
                sum += is_escaped_simple(bytes[i]) as usize;
            }
        }
        sum + 2 // The quote characters
    }
}
