use const_format::str_split;

#[test]
fn test_str_split_with_empty_str_arg() {
    assert_eq!(str_split!("", ""), ["", ""]);
    assert_eq!(str_split!("f", ""), ["", "f", ""]);
    assert_eq!(str_split!("fo", ""), ["", "f", "o", ""]);
    assert_eq!(str_split!("fob", ""), ["", "f", "o", "b", ""]);

    assert_eq!(
        str_split!("!Aq¡🧡🧠₀₁oñ个", ""),
        ["", "!", "A", "q", "¡", "", "", "🧡", "🧠", "₀", "₁", "o", "ñ", "个", ""],
    );
}

#[test]
fn test_str_split_with_space_str_arg() {
    assert_eq!(str_split!("fob", " "), ["fob"]);
    assert_eq!(str_split!(" fob", " "), ["", "fob"]);
    assert_eq!(str_split!(" fob ", " "), ["", "fob", ""]);
    assert_eq!(str_split!("foo bar baz", " "), ["foo", "bar", "baz"]);
    assert_eq!(str_split!("foo  bar baz", " "), ["foo", "", "bar", "baz"]);
}

#[test]
fn test_str_split_with_dash_str_arg() {
    assert_eq!(str_split!("fob", "-"), ["fob"]);
    assert_eq!(str_split!("-fob", "-"), ["", "fob"]);
    assert_eq!(str_split!("-fob-", "-"), ["", "fob", ""]);
    assert_eq!(str_split!("foo-bar-baz", "-"), ["foo", "bar", "baz"]);
    assert_eq!(str_split!("foo--bar-baz", "-"), ["foo", "", "bar", "baz"]);
}

#[test]
fn test_str_split_with_word_arg() {
    assert_eq!(str_split!("fob", "XY"), ["fob"]);
    assert_eq!(str_split!("XYfob", "XY"), ["", "fob"]);
    assert_eq!(str_split!("XYfobXY", "XY"), ["", "fob", ""]);
    assert_eq!(str_split!("fooXYbarXYbaz", "XY"), ["foo", "bar", "baz"]);
    assert_eq!(str_split!("fooXY bar XYbaz", "XY"), ["foo", " bar ", "baz"]);
}
