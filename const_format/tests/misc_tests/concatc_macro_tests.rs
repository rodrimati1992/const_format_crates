use cfmt_b::Formatter;
use cfmt_b::{ascii_str, concatc, impl_fmt, try_};

use std::num::NonZeroUsize;

const STD_TYPES: &str = concatc!(
    r#"\\\-hello-\\\"#,
    100u8,
    false,
    true,
    match NonZeroUsize::new(34) {
        Some(x) => x,
        None => loop {},
    },
);

const USER_TYPES: &str = concatc!(Twice("hello "), ascii_str!("world!"));

#[test]
fn concatc_test() {
    assert_eq!(STD_TYPES, r#"\\\-hello-\\\100falsetrue34"#);
    assert_eq!(USER_TYPES, r#"hello hello world!"#);
}

struct Twice(&'static str);

impl_fmt! {
    impl Twice;

    const fn const_display_fmt(&self, fmt:&mut Formatter<'_>) -> cfmt_b::Result {
        try_!(fmt.write_str(self.0));
        try_!(fmt.write_str(self.0));
        Ok(())
    }
}
