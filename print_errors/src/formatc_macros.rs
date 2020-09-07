use const_format::{formatc, formatcp};

const _: &str = formatcp!("{}");

const _: &str = formatcp!("{}", foo = "", 100u8);

const _: &str = formatcp!("{}", 0);

const _: &str = formatc!("{}");

const _: &str = formatc!("{}", foo = "", 100u8);

const _: &str = formatc!("{}", 0);

const _: &str = formatc!("{}", {
    let a = 0;
    let b = 0;
    a + b
});
