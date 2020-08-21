#[doc(hidden)]
#[macro_export]
macro_rules! __for_range{
    ( $var:ident in $range:expr => $($for_body:tt)* )=>({
        let $crate::pmr::Range{start: mut $var, end} = $range;
        while $var < end {
            {$($for_body)*}
            $var+=1;
        }
    })
}

#[doc(hidden)]
#[macro_export]
macro_rules! __write_pvariant {
    (int, $parg:expr, $elem:ident => $out:ident) => {{
        let wrapper = $crate::pmr::PWrapper($elem);

        let debug_display;
        let bin;
        let hex;

        let sa: &$crate::pmr::StartAndArray<[_]> = match $parg.fmt {
            $crate::pmr::Formatting::Display => {
                debug_display = wrapper.to_start_array_display();
                &debug_display
            }
            $crate::pmr::Formatting::Debug => match $parg.fmt_flags.mode() {
                $crate::pmr::FormattingMode::Regular => {
                    debug_display = wrapper.to_start_array_debug();
                    &debug_display
                }
                $crate::pmr::FormattingMode::Binary => {
                    bin = wrapper.to_start_array_binary();
                    &bin
                }
                $crate::pmr::FormattingMode::Hexadecimal => {
                    hex = wrapper.to_start_array_hexadecimal();
                    &hex
                }
            },
        };

        let mut start = sa.start;
        while start < sa.array.len() {
            $out.array[$out.len] = sa.array[start];
            $out.len += 1;
            start += 1;
        }
    }};
    (str, $parg:expr, $elem:ident => $out:ident) => {{
        let str = $elem.as_bytes();
        let is_display = $parg.fmt.is_display();
        let mut i = 0;
        if is_display {
            while i < str.len() {
                $out.array[$out.len] = str[i];
                $out.len += 1;
                i += 1;
            }
        } else {
            $out.array[$out.len] = b'"';
            $out.len += 1;
            while i < str.len() {
                use $crate::pmr::{hex_as_ascii, ForEscaping, FOR_ESCAPING};

                let c = str[i];
                let mut written_c = c;
                if c < 128 {
                    let shifted = 1 << c;

                    if (FOR_ESCAPING.is_escaped & shifted) != 0 {
                        $out.array[$out.len] = b'\\';
                        $out.len += 1;
                        if (FOR_ESCAPING.is_backslash_escaped & shifted) == 0 {
                            $out.array[$out.len] = b'x';
                            $out.array[$out.len + 1] = hex_as_ascii(c >> 4);
                            $out.len += 2;
                            written_c = hex_as_ascii(c & 0b1111);
                        } else {
                            written_c = ForEscaping::get_backslash_escape(c);
                        };
                    }
                }
                $out.array[$out.len] = written_c;
                $out.len += 1;
                i += 1;
            }
            $out.array[$out.len] = b'"';
            $out.len += 1;
        }
    }};
}
