use super::{remove_margin, write_with_flag};

use const_format::{
    fmt::{Error, Formatter, FormattingFlags},
    impl_fmt, try_, PWrapper,
};

////////////////////////////////////////////////////////////////////////////////

const fn format_b_field(b: &'static [u32], fmt: &mut Formatter<'_>) -> Result<(), Error> {
    let is_uneven = b[0] % 2 != 0;
    let flags = FormattingFlags::NEW
        .set_alternate(is_uneven)
        .set_hexadecimal();

    PWrapper(b).const_debug_fmt(&mut fmt.make_formatter(flags))
}

////////////////////////////////////////////////////////////////////////////////

struct BracedStruct {
    a: u32,
    b: &'static [u32],
    rec: Option<&'static BracedStruct>,
}

impl_fmt! {
    impl BracedStruct;

    pub const fn const_debug_fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), Error> {
        let mut fmt = fmt.debug_struct("BracedStruct");
        {
            let mut fmt = fmt.field("margin");
            let margin = fmt.margin();
            try_!(fmt.write_usize_display(margin));
        }
        try_!(fmt.field("a").write_u32_debug(self.a));
        try_!(format_b_field(self.b, fmt.field("b")));
        if let Some(x) = self.rec {
            try_!(x.const_debug_fmt(fmt.field("rec")));
        }
        fmt.finish()
    }
}

#[test]
fn formatting_struct() {
    let expected = remove_margin(
        "
        BracedStruct {
            margin: 4,
            a: 3,
            b: [
                0x9,
                0xC,
                0xF,
            ],
            rec: BracedStruct {
                margin: 8,
                a: 8,
                b: [A, E, 12],
                rec: BracedStruct {
                    margin: 12,
                    a: 21,
                    b: [
                        0xF,
                        0x14,
                        0x19,
                    ],
                },
            },
        }\
    ",
    );

    let list = BracedStruct {
        a: 3,
        b: &[9, 12, 15],
        rec: Some(&BracedStruct {
            a: 8,
            b: &[10, 14, 18],
            rec: Some(&BracedStruct {
                a: 21,
                b: &[15, 20, 25],
                rec: None,
            }),
        }),
    };

    let flags = FormattingFlags::NEW.set_alternate(true);

    write_with_flag(flags, &expected, &|mut fmt| {
        list.const_debug_fmt(&mut fmt).unwrap();
    })
}

////////////////////////////////////////////////////////////////////////////////

struct TupleStruct {
    a: u32,
    b: &'static [u32],
    rec: Option<&'static TupleStruct>,
}

impl_fmt! {
    impl TupleStruct;

    pub const fn const_debug_fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), Error> {
        let mut fmt = fmt.debug_tuple("TupleStruct");
        {
            let mut fmt = fmt.field();
            let margin = fmt.margin();
            try_!(fmt.write_usize_display(margin));
        }
        try_!(fmt.field().write_u32_debug(self.a));
        try_!(format_b_field(self.b, fmt.field()));
        if let Some(x) = self.rec {
            try_!(x.const_debug_fmt(fmt.field()));
        }
        fmt.finish()
    }
}

#[test]
fn formatting_tuple() {
    let expected = remove_margin(
        "
        TupleStruct(
            4,
            3,
            [
                0x9,
                0xC,
                0xF,
            ],
            TupleStruct(
                8,
                8,
                [A, E, 12],
                TupleStruct(
                    12,
                    21,
                    [
                        0xF,
                        0x14,
                        0x19,
                    ],
                ),
            ),
        )\
    ",
    );

    let list = TupleStruct {
        a: 3,
        b: &[9, 12, 15],
        rec: Some(&TupleStruct {
            a: 8,
            b: &[10, 14, 18],
            rec: Some(&TupleStruct {
                a: 21,
                b: &[15, 20, 25],
                rec: None,
            }),
        }),
    };

    let flags = FormattingFlags::NEW.set_alternate(true);

    write_with_flag(flags, &expected, &|mut fmt| {
        list.const_debug_fmt(&mut fmt).unwrap();
    })
}

////////////////////////////////////////////////////////////////////////////////

struct List {
    a: u32,
    b: &'static [u32],
    rec: Option<&'static List>,
}

impl_fmt! {
    impl List;

    pub const fn const_debug_fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), Error> {
        let mut fmt = fmt.debug_list();
        {
            let mut fmt = fmt.entry();
            let margin = fmt.margin();
            try_!(fmt.write_usize_display(margin));
        }
        try_!(fmt.entry().write_u32_debug(self.a));
        try_!(format_b_field(self.b, fmt.entry()));
        if let Some(x) = self.rec {
            try_!(x.const_debug_fmt(fmt.entry()));
        }
        fmt.finish()
    }
}

#[test]
fn formatting_list() {
    let expected = remove_margin(
        "
        [
            4,
            3,
            [
                0x9,
                0xC,
                0xF,
            ],
            [
                8,
                8,
                [A, E, 12],
                [
                    12,
                    21,
                    [
                        0xF,
                        0x14,
                        0x19,
                    ],
                ],
            ],
        ]\
    ",
    );

    let list = List {
        a: 3,
        b: &[9, 12, 15],
        rec: Some(&List {
            a: 8,
            b: &[10, 14, 18],
            rec: Some(&List {
                a: 21,
                b: &[15, 20, 25],
                rec: None,
            }),
        }),
    };

    let flags = FormattingFlags::NEW.set_alternate(true);

    write_with_flag(flags, &expected, &|mut fmt| {
        list.const_debug_fmt(&mut fmt).unwrap();
    })
}

////////////////////////////////////////////////////////////////////////////////

struct Set {
    a: u32,
    b: &'static [u32],
    rec: Option<&'static Set>,
}

impl_fmt! {
    impl Set;

    pub const fn const_debug_fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), Error> {
        let mut fmt = fmt.debug_set();
        {
            let mut fmt = fmt.entry();
            let margin = fmt.margin();
            try_!(fmt.write_usize_display(margin));
        }
        try_!(fmt.entry().write_u32_debug(self.a));
        try_!(format_b_field(self.b, fmt.entry()));
        if let Some(x) = self.rec {
            try_!(x.const_debug_fmt(fmt.entry()));
        }
        fmt.finish()
    }
}

#[test]
fn formatting_set() {
    let expected = remove_margin(
        "
        {
            4,
            3,
            [
                0x9,
                0xC,
                0xF,
            ],
            {
                8,
                8,
                [A, E, 12],
                {
                    12,
                    21,
                    [
                        0xF,
                        0x14,
                        0x19,
                    ],
                },
            },
        }\
    ",
    );

    let set = Set {
        a: 3,
        b: &[9, 12, 15],
        rec: Some(&Set {
            a: 8,
            b: &[10, 14, 18],
            rec: Some(&Set {
                a: 21,
                b: &[15, 20, 25],
                rec: None,
            }),
        }),
    };

    let flags = FormattingFlags::NEW.set_alternate(true);

    write_with_flag(flags, &expected, &|mut fmt| {
        set.const_debug_fmt(&mut fmt).unwrap();
    })
}
