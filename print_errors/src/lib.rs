#![allow(unused_imports)]

mod formatc_macros;

const _: &str = cfmt::concatcp!(0, 1, ());

const _: &str = cfmt::concatc!(0, 1, ());

mod using_assertc_macros;

mod using_writec_macro;
