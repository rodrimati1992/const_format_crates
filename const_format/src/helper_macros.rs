#[doc(hidden)]
#[macro_export]
macro_rules! __for_range{
    ( $var:ident in $range:expr => $($for_body:tt)* )=>({
        let $crate::pmr::Range{mut start,end} = $range;
        while start < end {
            let $var = start;
            {$($for_body)*}
            start+=1;
        }
    })
}

#[doc(hidden)]
#[macro_export]
macro_rules! __write_pvariant {
    (int, $parg:expr, $elem:ident => $out:ident) => {{
        let mut sa = $crate::pmr::PWrapper($elem).to_start_array($parg.fmt);
        while sa.start < sa.array.len() {
            $out.array[$out.len] = sa.array[sa.start];
            $out.len += 1;
            sa.start += 1;
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
                if $crate::pmr::is_escaped_simple(str[i]) {
                    $out.array[$out.len] = b'\\';
                    $out.len += 1;
                }
                $out.array[$out.len] = str[i];
                $out.len += 1;
                i += 1;
            }
            $out.array[$out.len] = b'"';
            $out.len += 1;
        }
    }};
}
