#[macro_use]
mod helper_macros;

mod formatting;
#[macro_use]
mod fmt_macros;
mod pargument;
mod pwrapper;
mod utils;


#[doc(hidden)]
pub mod pmr{
    pub use crate::{
        formatting::{Formatting, LenAndArray, StartAndArray, is_escaped_simple},
        pargument::{PArgument, PConvWrapper, PVariant},
        pwrapper::PWrapper,
        utils::Transmute,
    };
}


