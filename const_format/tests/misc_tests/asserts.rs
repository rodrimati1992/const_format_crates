use const_format::assertc;

struct Foo;

assertc!(true, "Hello, world {:?}", {
    impl Foo {
        const BAR: u32 = 10;
    }
},);

assertc!(true, concat!("Hello", r#"world {:?}"#), {
    impl Foo {
        const BAZ: u32 = 11;
    }
},);

#[test]
fn assertc_emits_formatting() {
    assert_eq!(Foo::BAR, 10);
    assert_eq!(Foo::BAZ, 11);
}
