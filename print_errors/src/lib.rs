#![feature(const_mut_refs)]
#![allow(unused_imports)]

mod formatc_macros;

const _: &str = const_format::concatcp!(0, 1, ());

mod using_assertc_macros;

mod using_writec_macro;
