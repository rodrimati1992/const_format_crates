use crate::{
    formatting::Formatting,
    pwrapper::PWrapper,
};

/// The uniform representation for every argument of the concatp macro.
pub struct PArgument{
    pub elem: PVariant,
    pub fmt_len: usize,
    pub fmt: Formatting,
}

#[doc(hidden)]
pub enum PVariant {
    Str(&'static str),
    Int(Integer),
}

#[derive(Copy, Clone)]
pub struct Integer{
    pub is_negative: bool,
    pub unsigned: u128,
}


pub struct PConvWrapper<T>(pub T);


macro_rules! pconvwrapper_impls {
    ($( ($Signed:ty, $Unsigned:ty) )*) => (
        $(
            impl PConvWrapper<$Signed> {
                pub const fn to_pargument(self, fmt: Formatting)->PArgument{
                    PArgument {
                        fmt_len: $crate::pmr::PWrapper(self.0).fmt_len(fmt),
                        fmt,
                        elem: PVariant::Int(Integer{
                            is_negative: self.0 < 0,
                            unsigned: self.0.wrapping_abs() as u128,
                        }),
                    }
                }
            }

            impl PConvWrapper<$Unsigned> {
                pub const fn to_pargument(self, fmt: Formatting)->PArgument{
                    PArgument {
                        fmt_len: $crate::pmr::PWrapper(self.0).fmt_len(fmt),
                        fmt,
                        elem: PVariant::Int(Integer{
                            is_negative: false,
                            unsigned: self.0 as u128,
                        }),
                    }
                }
            }
        )*
    )
}

pconvwrapper_impls! {
    (i8, u8)
    (i16, u16)
    (i32, u32)
    (i64, u64)
    (i128, u128)
    (isize, usize)
}

impl PConvWrapper<bool> {
    pub const fn to_pargument(self, _: Formatting)->PArgument{
        PConvWrapper(if self.0 { "true" }else{ "false" })
            .to_pargument(Formatting::Display)
    }
}

impl PConvWrapper<&'static str> {
    pub const fn to_pargument(self, fmt: Formatting)->PArgument{
        PArgument {
            fmt_len: PWrapper(self.0).fmt_len(fmt),
            fmt,
            elem: PVariant::Str(self.0),
        }
    }
}

