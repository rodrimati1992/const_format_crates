#![allow(non_fmt_panics)]

#[track_caller]
pub const fn assert_(cond: bool, message: &'static str) {
    if cond {
        panic!(message)
    }
}
