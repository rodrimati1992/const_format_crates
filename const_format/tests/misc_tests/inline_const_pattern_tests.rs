use crate::cfmt_b;

use crate::cfmt_b::{
    concatc,
    concatcp,
    formatc,
    formatcp,
    map_ascii_case,
    str_get,
    str_index,
    str_repeat,
    str_replace,
    str_split,
};


#[test]
fn concatc_inline_pat_tests() {
    assert!(matches!("foo", concatc!("fo", "o")));
    assert!(!matches!("bar", concatc!("bar", "r")));
}

#[test]
fn concatcp_inline_pat_tests() {
    assert!(matches!("foo", concatcp!("fo", "o")));
    assert!(!matches!("bar", concatcp!("bar", "r")));
}

#[test]
fn formatc_inline_pat_tests() {
    assert!(matches!("foo", formatc!("f{0}{0}", "o")));
    assert!(!matches!("bar", formatc!("bar{}", "r")));
}

#[test]
fn formatcp_inline_pat_tests() {
    assert!(matches!("foo", formatcp!("f{0}{0}", "o")));
    assert!(!matches!("bar", formatcp!("bar{}", "r")));
}

#[test]
fn map_ascii_case_inline_pat_tests() {
    assert!(matches!("foo", map_ascii_case!(cfmt_b::Case::Lower, "FOO")));
    assert!(!matches!("bar", map_ascii_case!(cfmt_b::Case::Upper, "bar")));
}

#[test]
fn str_get_inline_pat_tests() {
    assert!(matches!(Some("foo"), str_get!(" foobar", 1..4)));
    assert!(matches!(None, str_get!(" foobar", 10)));
}

#[test]
fn str_index_inline_pat_tests() {
    assert!(matches!("foo", str_index!(" foobar", 1..4)));
    assert!(!matches!("foo", str_index!(" foobar", 1..5)));
}

#[test]
fn str_repeat_inline_pat_tests() {
    assert!(matches!("foofoofoo", str_repeat!("foo", 3)));
    assert!(!matches!("foo", str_repeat!("foo", 0)));
}

#[test]
fn str_replace_inline_pat_tests() {
    assert!(matches!("fooo", str_replace!("fo", "o", "ooo")));
    assert!(!matches!("foo", str_replace!("fo", "o", "bar")));
}

