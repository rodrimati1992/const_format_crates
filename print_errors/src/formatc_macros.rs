use cfmt::formatcp;

#[cfg(feature = "nightly")]
use cfmt::formatc;

const _: &str = formatcp!("{}");

const _: &str = formatcp!("{}", foo = "", 100u8 + 0);

const _: &str = formatcp!("{}", 0 + 0);

const _: &str = formatcp!("{}", 0u8, 0u8 + 1);

const _: &str = formatcp!("{}", |fmt| 0 + 0);

#[cfg(feature = "nightly")]
const _: () = {
    const _: &str = formatc!("{}");

    const _: &str = formatc!("{}", foo = "", 100u8 + 0);

    const _: &str = formatc!("{}", 0 + 0);

    const _: &str = formatc!("{}", {
        let a = 0;
        let b = 0;
        a + b
    });
};
