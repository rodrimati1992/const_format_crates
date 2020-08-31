//! Types for the documentation examples.
//!
//! # Features
//!
//! This module is only exported with the "fmt" feature, and the nightly compiler,
//! because at the time of writing these docs (2020-08-XX) mutable references in const fn
//! require the unstable
//! [`const_mut_refs`](https://github.com/rust-lang/rust/issues/57349) feature.

use crate::{impl_fmt, try_, Error, Formatter, PWrapper};

#[derive(Debug, Copy, Clone)]
pub struct Point3 {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

impl_fmt! {
    impl Point3;

    pub const fn const_debug_fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let mut f = f.debug_struct("Point3");
        try_!(PWrapper(self.x).const_debug_fmt(f.field("x")));
        try_!(PWrapper(self.y).const_debug_fmt(f.field("y")));
        try_!(PWrapper(self.z).const_debug_fmt(f.field("z")));
        f.finish()
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Unit;

impl_fmt! {
    impl Unit;

    pub const fn const_debug_fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.debug_struct("Unit").finish()
    }
}
