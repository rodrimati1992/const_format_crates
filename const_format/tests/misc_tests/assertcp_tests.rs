#![allow(unreachable_code)]
#![allow(non_local_definitions)]

use cfmt_b::{assertcp, assertcp_eq, assertcp_ne};

struct Foo;

// no need for formatting string
assertcp!(true);

assertcp!({
    impl Foo {
        const NINE: u32 = 9;
    }
    true
});

assertcp!(true, "Hello, world {:?}", {
    impl Foo {
        const BAR: u32 = 10;
    }
    1u8
},);

assertcp!(true, concat!("Hello", r#"world {:?}"#), {
    impl Foo {
        const BAZ: u32 = 11;
    }
    1u8
},);

#[test]
fn assertcp_emits_formatting() {
    assert_eq!(Foo::NINE, 9);
    assert_eq!(Foo::BAR, 10);
    assert_eq!(Foo::BAZ, 11);
    assert_eq!(Foo::QUX, 12);
    assert_eq!(Foo::SPAT, 13);
    assert_eq!(Foo::OOF, 14);
    assert_eq!(Foo::RAB, 15);
}

// The formatting code should not run if the assertion is true
assertcp!(true, "{}", {
    let _x: u32 = loop {};
    _x
});

const X: u8 = 123;

#[allow(unused_variables)]
const _: () = {
    ////////////////////////////////////////////////////////////////////////////////
    ////        assertcp_eq

    assertcp_eq!(
        {
            impl Foo {
                const QUX: u32 = 12;
            }
            0u8
        },
        0u8,
    );
    assertcp_eq!(false, {
        impl Foo {
            const SPAT: u32 = 13;
        }
        false
    },);
    assertcp_eq!(' ', ' ');
    assertcp_eq!("hello", "hello");

    assertcp_eq!(0u8, 0u8, "world");
    assertcp_eq!(false, false, "world{}", 1u8);
    assertcp_eq!(' ', ' ', "world{foo}", foo = 1u8);
    assertcp_eq!("hello", "hello", "world{X}");

    ////////////////////////////////////////////////////////////////////////////////
    ////        assertcp_ne

    assertcp_ne!("hello", "helo");
    assertcp_ne!(0u8, 1u8, "world");
    assertcp_ne!(
        {
            impl Foo {
                const OOF: u32 = 14;
            }
            false
        },
        true,
        "world{}",
        {
            let x: u32 = loop {};
            x
        },
    );
    assertcp_ne!(
        ' ',
        {
            impl Foo {
                const RAB: u32 = 15;
            }
            'A'
        },
        "world{foo}",
        foo = {
            #[allow(unconditional_panic)]
            let foo: u8 = [][0];
            foo
        },
    );
};
