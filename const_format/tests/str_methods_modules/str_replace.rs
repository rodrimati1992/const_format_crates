use const_format::__str_methods::{ReplaceInput, ReplaceInputConv};
use const_format::str_replace;

macro_rules! assert_case {
    ($input:expr, $patt:expr, $replace_with:expr, $output:expr $(,)*) => {{
        const IN: &str = $input;
        const ARGS: ReplaceInput = ReplaceInputConv(IN, $patt, REPLACE_WITH).conv();
        const REPLACE_WITH: &str = $replace_with;
        const OUT: &str = $output;

        assert_eq!(ARGS.replace_length(), OUT.len());

        assert_eq!(
            std::str::from_utf8(&ARGS.replace::<{ OUT.len() }>()).unwrap(),
            OUT,
        );

        assert_eq!(str_replace!(IN, $patt, REPLACE_WITH), OUT);
    }};
}

#[test]
fn test_small_pattern() {
    assert_case! {"hequ", "q", "XY", "heXYu"}
    assert_case! {"hequx", "q", "XYZ", "heXYZux"}
    assert_case! {"hequq", "q", "XY", "heXYuXY"}
    assert_case! {"hequxq", "q", "XYZ", "heXYZuxXYZ"}

    assert_case! {"hequ", "qu", "XY", "heXY"}
    assert_case! {"hequ", "qu", "XYZ", "heXYZ"}
    assert_case! {"hequx", "qu", "XYZ", "heXYZx"}
}

#[test]
fn test_replace_overlapping() {
    assert_case! {"helololololol", "lol", "XY", "heXYoXYoXY"}

    assert_case! {"hequ", "qux", "XY", "hequ"}
    assert_case! {"hequx", "qux", "XYZA", "heXYZA"}
    assert_case! {"heququx", "qux", "XYZAB", "hequXYZAB"}
    assert_case! {"hequxqu", "qux", "XYZABC", "heXYZABCqu"}
}

#[test]
fn test_replace_empty() {
    assert_case! {"", "qux", "-------", ""}

    assert_case! {"hequxqu", "", "-------------", "hequxqu"}

    assert_case! {"hequxqu", "qux", "", "hequ"}
}
