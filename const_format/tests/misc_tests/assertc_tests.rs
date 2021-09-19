#![allow(unreachable_code)]

use cfmt_b::for_examples::{Point3, Unit};
use cfmt_b::{assertc, assertc_eq, assertc_ne, call_debug_fmt};

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

// braces in arguments that take a formatter should work
assertc!(
    true,
    "{foo}\n{}",
    |fmt| {
        impl Foo {
            const FMT_FOO: u32 = 12;
        }
        call_debug_fmt!(array, [100u8], fmt)
    },
    foo = |fmt| {
        impl Foo {
            const FMT_BAR: u32 = 13;
        }
        call_debug_fmt!(array, [Unit, Unit], fmt)
    },
);

// single expressions that take the formatter should also work
assertc!(
    true,
    "{foo}\n{foo:#?}\n{}",
    |fmt| call_debug_fmt!(array, [100u8], fmt),
    foo = |fmt| call_debug_fmt!(array, [Unit, Unit], fmt),
);

#[test]
fn assertc_emits_formatting() {
    assert_eq!(Foo::BAR, 10);
    assert_eq!(Foo::BAZ, 11);
    assert_eq!(Foo::FMT_FOO, 12);
    assert_eq!(Foo::FMT_BAR, 13);
}

// The formatting code should not run if the assertion is true
assertc!(true, "{}", {
    let _x: u32 = loop {};
    _x
});

#[allow(unused_variables)]
const _: () = {
    const POINT: Point3 = Point3 { x: 3, y: 5, z: 8 };
    const OTHER_POINT: Point3 = Point3 {
        x: 13,
        y: 21,
        z: 34,
    };

    ////////////////////////////////////////////////////////////////////////////////
    ////        assertc_eq

    assertc_eq!(POINT, POINT);
    assertc_eq!(OTHER_POINT, OTHER_POINT);

    assertc_eq!(Unit, Unit);

    assertc_eq!(0u8, 0u8);
    assertc_eq!("foo", "foo", "hello");
    assertc_eq!(Some("foo"), Some("foo"), "hello {}", {
        let x: u32 = loop {};
        x
    });
    assertc_eq!([false], [false], "{}", |f| {
        loop {}
        f.write_str("hello")
    });

    ////////////////////////////////////////////////////////////////////////////////
    ////        assertc_ne
    assertc_ne!(POINT, OTHER_POINT);
    assertc_ne!(OTHER_POINT, POINT);

    assertc_ne!(0u8, 3u8);
    assertc_ne!("foo", "bar", "hello");
    assertc_ne!(Some("foo"), Some("bar"), "hello {}", {
        let x: u32 = loop {};
        x
    });
    assertc_ne!([false], [true], "{}", |f| {
        loop {}
        f.write_str("hello")
    });
};
