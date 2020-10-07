#![feature(const_mut_refs)]

use const_format::{concatcp, formatcp};

#[cfg(feature = "nightly")]
pub mod nightly {
    use const_format::{
        assertc, assertc_eq, assertc_ne, concatc, for_examples::Unit, formatc, strwriter_as_str,
        writec, StrWriter, StrWriterMut,
    };

    pub const TWO: u32 = 2;
    pub const TEN: u32 = 10;

    assertc!(TWO != TEN, "{} != {}", TWO, TEN);
    assertc_eq!(TWO, TWO);
    assertc_ne!(TWO, TEN);

    pub const CONCATC_A: &str = concatc!("hello", "world");
    pub const CONCATC_B: &str = concatc!(10u8, TWO);

    pub const FORMATC_A: &str = formatc!("{}hello{}{:?}", "foo", 100u8, Unit);

    const fn as_str_ctor() -> StrWriter<[u8; 100]> {
        let mut writer = StrWriter::new([0; 100]);

        let _ = writec!(writer, "{:#?}", Unit);
        {
            let mut writer = StrWriterMut::new(&mut writer);

            let _ = writec!(writer, "{0}{0:?}", 100u8);
        }
        writer
    }

    pub const AS_STR: &str = strwriter_as_str!(&as_str_ctor());
}

pub const CONCATCP_A: &str = concatcp!("hello", "world");
pub const CONCATCP_B: &str = concatcp!(10u8, 20u8);

pub const FORMATCP_A: &str = formatcp!("{}hello{:x?}", "foo", 100u8);

fn main() {}
