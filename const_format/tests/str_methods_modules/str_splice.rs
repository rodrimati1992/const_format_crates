use const_format::{str_splice, SplicedStr};

fn ss(output: &'static str, removed: &'static str) -> SplicedStr {
    SplicedStr { output, removed }
}

const IN: &str = "abcdefghij";
const RW: &str = "_.-";

#[test]
fn splice_ranges() {
    assert_eq!(str_splice!(IN, 2, RW), ss("ab_.-defghij", "c"));
    assert_eq!(str_splice!(IN, 4, RW), ss("abcd_.-fghij", "e"));

    assert_eq!(str_splice!(IN, 2..4, RW), ss("ab_.-efghij", "cd"));
    assert_eq!(str_splice!(IN, 4..4, RW), ss("abcd_.-efghij", ""));

    assert_eq!(str_splice!(IN, 2..=4, RW), ss("ab_.-fghij", "cde"));
    assert_eq!(str_splice!(IN, 4..=4, RW), ss("abcd_.-fghij", "e"));

    assert_eq!(str_splice!(IN, ..2, RW), ss("_.-cdefghij", "ab"));
    assert_eq!(str_splice!(IN, ..4, RW), ss("_.-efghij", "abcd"));

    assert_eq!(str_splice!(IN, ..=1, RW), ss("_.-cdefghij", "ab"));
    assert_eq!(str_splice!(IN, ..=3, RW), ss("_.-efghij", "abcd"));

    assert_eq!(str_splice!(IN, 5.., RW), ss("abcde_.-", "fghij"));
    assert_eq!(str_splice!(IN, 5..IN.len(), RW), ss("abcde_.-", "fghij"));
    assert_eq!(str_splice!(IN, 7.., RW), ss("abcdefg_.-", "hij"));

    assert_eq!(str_splice!(IN, .., RW), ss("_.-", "abcdefghij"));
}

#[test]
fn replacements() {
    assert_eq!(str_splice!("abcde", 2..4, ""), ss("abe", "cd"));
    assert_eq!(str_splice!("abcde", 2..4, "h"), ss("abhe", "cd"));
    assert_eq!(str_splice!("abcde", 2..4, "he"), ss("abhee", "cd"));
    assert_eq!(str_splice!("abcde", 2..4, "hel"), ss("abhele", "cd"));
    assert_eq!(str_splice!("abcde", 2..4, "hell"), ss("abhelle", "cd"));
    assert_eq!(str_splice!("abcde", 2..4, "hello"), ss("abhelloe", "cd"));
}
