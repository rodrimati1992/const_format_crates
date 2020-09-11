#![cfg_attr(feature = "nightly", feature(const_mut_refs))]
#![allow(unused_imports)]

mod formatc_macros;

const _: &str = cfmt::concatcp!(0, 1, ());

#[cfg(feature = "nightly")]
const _: &str = cfmt::concatc!(0, 1, ());

#[cfg(feature = "nightly")]
mod using_assertc_macros;

#[cfg(feature = "nightly")]
mod using_writec_macro;
