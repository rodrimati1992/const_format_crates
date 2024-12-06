use const_format::{str_split, str_split_pat};

#[test]
fn test_str_split_pat_basic_equivalence() {
    assert_eq!(str_split_pat!("fob", "XY"), &["fob"][..]);
    assert_eq!(str_split_pat!("XYfob", "XY"), &["", "fob"][..]);
    assert_eq!(str_split_pat!("XYfobXY", "XY"), &["", "fob", ""][..]);
    assert_eq!(
        str_split_pat!("fooXYbarXYbaz", "XY"),
        &["foo", "bar", "baz"][..]
    );
    assert_eq!(
        str_split_pat!("fooXY bar XYbaz", "XY"),
        &["foo", " bar ", "baz"][..]
    );
}

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

#[test]
fn test_str_split_with_ascii_char_arg() {
    assert_eq!(str_split!("fob", '-'), ["fob"]);
    assert_eq!(str_split!("-fob", '-'), ["", "fob"]);
    assert_eq!(str_split!("-fob-", '-'), ["", "fob", ""]);
    assert_eq!(str_split!("foo-bar-baz", '-'), ["foo", "bar", "baz"]);
    assert_eq!(str_split!("foo- bar -baz", '-'), ["foo", " bar ", "baz"]);
}

#[test]
fn test_str_split_with_non_ascii_char_arg() {
    {
        assert_eq!(''.len_utf8(), 1);
        assert_eq!(str_split!("fob", ''), ["fob"]);
        assert_eq!(str_split!("fob", ''), ["", "fob"]);
        assert_eq!(str_split!("fob", ''), ["", "fob", ""]);
        assert_eq!(str_split!("foobarbaz", ''), ["foo", "bar", "baz"]);
        assert_eq!(str_split!("foo bar baz", ''), ["foo", " bar ", "baz"]);
    }
    {
        assert_eq!('ñ'.len_utf8(), 2);
        assert_eq!(str_split!("fob", 'ñ'), ["fob"]);
        assert_eq!(str_split!("ñfob", 'ñ'), ["", "fob"]);
        assert_eq!(str_split!("ñfobñ", 'ñ'), ["", "fob", ""]);
        assert_eq!(str_split!("fooñbarñbaz", 'ñ'), ["foo", "bar", "baz"]);
        assert_eq!(str_split!("fooñ bar ñbaz", 'ñ'), ["foo", " bar ", "baz"]);
    }
    {
        assert_eq!('₀'.len_utf8(), 3);
        assert_eq!(str_split!("fob", '₀'), ["fob"]);
        assert_eq!(str_split!("₀fob", '₀'), ["", "fob"]);
        assert_eq!(str_split!("₀fob₀", '₀'), ["", "fob", ""]);
        assert_eq!(str_split!("foo₀bar₀baz", '₀'), ["foo", "bar", "baz"]);
        assert_eq!(str_split!("foo₀ bar ₀baz", '₀'), ["foo", " bar ", "baz"]);
    }
    {
        assert_eq!('🧡'.len_utf8(), 4);
        assert_eq!(str_split!("fob", '🧡'), ["fob"]);
        assert_eq!(str_split!("🧡fob", '🧡'), ["", "fob"]);
        assert_eq!(str_split!("🧡fob🧡", '🧡'), ["", "fob", ""]);
        assert_eq!(str_split!("foo🧡bar🧡baz", '🧡'), ["foo", "bar", "baz"]);
        assert_eq!(str_split!("foo🧡 bar 🧡baz", '🧡'), ["foo", " bar ", "baz"]);
    }
}
