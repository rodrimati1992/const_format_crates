#[deny(clippy::double_parens)]
#[test]
fn test_clippy_double_parens_not_triggered() {
    std::convert::identity(cfmt_b::formatcp!("hello"));
}
