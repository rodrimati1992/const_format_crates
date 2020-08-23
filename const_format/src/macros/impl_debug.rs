/// For implementing DebugLike without using a derive macro.
#[macro_export]
macro_rules! impl_debug {
    (
        $($rem:tt)*
    ) => (
        $crate::__impl_debug_recursive!{

            impls[]
            tokens[$($rem)*]
        }
    );
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_debug_recursive{
    (
        impls[$($impls:tt)*]

        tokens[
            $(#[$impl_attr:meta])*
            impl[$($impl_:tt)*] $type:ty
            $(where[ $($where:tt)* ])?;

            $($rem:tt)*
        ]
    ) => (
        $crate::__impl_debug_recursive!{

            impls[
                $($impls)*
                (
                    $(#[$impl_attr])*
                    #[allow(unused_mut)]
                    impl[$($impl_)*] $type
                    where[ $($($where)*)? ];
                )
            ]
            tokens[
                $($rem)*
            ]
        }
    );
    (
        impls $impls:tt
        tokens[
            $($rem:tt)*
        ]
    ) => (
        $crate::__impl_debug_inner!{
            @all_impls
            impls $impls
            ($($rem)*)
        }
    );
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_debug_inner {
    (@all_impls
        impls [ $( $an_impl:tt )+ ]

        $stuff:tt
    )=>{
        $(
            $crate::__impl_debug_inner!{ @an_impl $an_impl $stuff }
        )+
    };
    (@an_impl
        (
            $(#[$impl_attr:meta])*
            impl[$($impl_:tt)*] $type:ty
            where[ $($where:tt)* ];
        )
        (
            struct $type_name:ident {
                $( $field_name:tt $(: $renamed:ident)? $(=> $field_expr:expr )? ),*
                $(,)?
            }
        )
    ) => (
        $(#[$impl_attr])*
        impl<$($impl_)*> $type
        where
            $($where)*
        {
            pub const fn const_debug_len(&self, f: &mut $crate::fmt::FormattingLength) {
                let Self{ $($field_name $(: $renamed )? ,)* ..} = self;

                let mut f = f.debug_struct(stringify!($type_name));
                $(
                    $crate::__impl_debug_field!(
                        @call_len
                        f,
                        $field_name $(: $renamed)? $(=> $field_expr )?,
                    );
                )*
                f.finish();
            }

            pub const fn const_debug_fmt(
                &self,
                f: &mut $crate::fmt::Formatter<'_>,
            ) -> $crate::pmr::Result<(), $crate::fmt::Error> {
                let Self{ $($field_name $(: $renamed )? ,)* ..} = self;

                let mut f = $crate::try_!(f.debug_struct(stringify!($type_name)));
                $(
                    $crate::__impl_debug_field!(
                        @call_fmt
                        f,
                        $field_name $(: $renamed)? $(=> $field_expr )?,
                    );
                )*
                f.finish()
            }
        }
    );
    (@an_impl
        (
            $(#[$impl_attr:meta])*
            impl[$($impl_:tt)*] $type:ty
            where[ $($where:tt)* ];
        )
        (
            struct $type_name:ident $((
                $( $field_name:ident $(=> $field_expr:expr )? ),*
                $(,)?
            ))?$(;)?
        )
    ) => (

        $(#[$impl_attr])*
        impl<$($impl_)*> $type
        where
            $($where)*
        {
            pub const fn const_debug_len(&self, f: &mut $crate::fmt::FormattingLength) {
                let Self $( ( $($field_name,)* ..) )? = self;

                let mut f = f.debug_tuple(stringify!($type_name));
                $($(
                    $crate::__impl_debug_field!(
                        @call_len_tuple
                        f,
                        $field_name $(=> $field_expr )?,
                    );
                )*)?
                f.finish();
            }

            pub const fn const_debug_fmt(
                &self,
                f: &mut $crate::fmt::Formatter<'_>,
            ) -> $crate::pmr::Result<(), $crate::fmt::Error> {
                let Self $( ( $($field_name,)* ..) )? = self;

                let mut f = $crate::try_!(f.debug_tuple(stringify!($type_name)));
                $($(
                    $crate::__impl_debug_field!(
                        @call_fmt_tuple
                        f,
                        $field_name $(=> $field_expr )?,
                    );
                )*)?
                f.finish()
            }
        }
    );
    (@an_impl
        (
            $(#[$impl_attr:meta])*
            impl[$($impl_:tt)*] $type:ty
            where[ $($where:tt)* ];
        )
        (
            enum $type_name:ident {
                $(
                    $variant:ident $( {$($brace_ts:tt)*} )? $( ($($paren_ts:tt)*) )?
                ),*
                $(,)?
            }
        )
    ) => (
        $(#[$impl_attr])*
        impl<$($impl_)*> $type
        where
            $($where)*
        {
            pub const fn const_debug_len(&self, f: &mut $crate::fmt::FormattingLength) {
                match self {
                    $(
                        $crate::__impl_debug_enum!(
                            @pat
                            $variant $( {$($brace_ts)*} )? $( ($($paren_ts)*) )?
                        ) => {
                            $crate::__impl_debug_enum!{
                                @len_method
                                f = f;
                                $variant $( {$($brace_ts)*} )? $( ($($paren_ts)*) )?
                            }
                        }
                    )*
                }
            }

            pub const fn const_debug_fmt(
                &self,
                f: &mut $crate::fmt::Formatter<'_>,
            ) -> $crate::pmr::Result<(), $crate::fmt::Error> {
                match self {
                    $(
                        $crate::__impl_debug_enum!(
                            @pat
                            $variant $( {$($brace_ts)*} )? $( ($($paren_ts)*) )?
                        ) => {
                            $crate::__impl_debug_enum!{
                                @fmt_method
                                f = f;
                                $variant $( {$($brace_ts)*} )? $( ($($paren_ts)*) )?
                            }
                        }
                    )*
                }
            }
        }
    );
    (@an_impl
        (
            $(#[$impl_attr:meta])*
            impl[$($impl_:tt)*] $type:ty
            where[ $($where:tt)* ];
        )
        (
            delegating = |$self:ident| $expr:expr
        )
    ) => (
        $(#[$impl_attr])*
        impl<$($impl_)*> $type
        where
            $($where)*
        {
            pub const fn const_debug_len(&self, f: &mut $crate::fmt::FormattingLength) {
                let $self = self;
                $expr.const_debug_len(f)
            }

            pub const fn const_debug_fmt(
                &self,
                f: &mut $crate::fmt::Formatter<'_>,
            ) -> $crate::pmr::Result<(), $crate::fmt::Error> {
                let $self = self;
                $expr.const_debug_fmt(f)
            }
        }
    )
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_debug_enum {
    (@pat
        $variant:ident {
            $( $field_name:tt $(:$renamed:ident)? $(=> $field_expr:expr )? ),*
            $(,)?
        }
    )=>{
        Self::$variant{
            $( $field_name $(:$renamed)?, )*
            ..
        }
    };
    (@pat
        $variant:ident $((
            $( $field_name:tt $(=> $field_expr:expr )? ),*
            $(,)?
        ))?
    )=>{
        Self::$variant $((
            $( $field_name, )*
            ..
        ))?
    };
    (@len_method
        f = $f:ident;
        $variant:ident {
            $( $field_name:tt $(:$renamed:ident)? $(=> $field_expr:expr )? ),*
            $(,)?
        }
    )=>{
        let mut $f = $f.debug_struct(stringify!($variant));
        $(
            $crate::__impl_debug_field!(
                @call_len
                $f,
                $field_name $(: $renamed)? $(=> $field_expr )?,
            );
        )*
        $f.finish()
    };
    (@len_method
        f = $f:ident;
        $variant:ident $((
            $( $field_name:tt $(=> $field_expr:expr )? ),*
            $(,)?
        ))?
    )=>{
        let mut $f = $f.debug_tuple(stringify!($variant));
        $($(
            $crate::__impl_debug_field!(
                @call_len_tuple
                $f,
                $field_name $(=> $field_expr )?,
            );
        )*)?
        $f.finish()
    };
    (@fmt_method
        f = $f:ident;
        $variant:ident {
            $( $field_name:tt $(:$renamed:ident)? $(=> $field_expr:expr )? ),*
            $(,)?
        }
    )=>{
        let mut $f = $crate::try_!($f.debug_struct(stringify!($variant)));
        $(
            $crate::__impl_debug_field!(
                @call_fmt
                $f,
                $field_name $(: $renamed)? $(=> $field_expr )?,
            );
        )*
        $f.finish()
    };
    (@fmt_method
        f = $f:ident;
        $variant:ident $((
            $( $field_name:tt $(=> $field_expr:expr )? ),*
            $(,)?
        ))?
    )=>{
        let mut $f = $crate::try_!($f.debug_tuple(stringify!($variant)));
        $($(
            $crate::__impl_debug_field!(
                @call_fmt_tuple
                $f,
                $field_name $(=> $field_expr )?,
            );
        )*)?
        $f.finish()
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_debug_field {
    (@call_len $fmt:ident, $field_name:tt $(,)?) => {
        $field_name.const_debug_len($fmt.field(stringify!($field_name)));
    };
    (@call_len $fmt:ident, $field_name:tt : $renamed:ident $(,)?) => {
        $renamed.const_debug_len($fmt.field(stringify!($field_name)));
    };
    (@call_len $fmt:ident, $field_name:tt $(: $renamed:ident)? => $expr:expr $(,)?) => {
        $expr.const_debug_len($fmt.field(stringify!($field_name)));
    };
    (@call_fmt $fmt:ident, $field_name:tt $(,)?) => {
        $crate::try_!(
            $field_name.const_debug_fmt($crate::try_!($fmt.field(stringify!($field_name))))
        );
    };
    (@call_fmt $fmt:ident, $field_name:tt : $renamed:ident $(,)?) => {
        $crate::try_!($renamed.const_debug_fmt($crate::try_!($fmt.field(stringify!($field_name)))));
    };
    (@call_fmt $fmt:ident, $field_name:tt $(: $renamed:ident)? => $expr:expr $(,)?) => {
        $crate::try_!($expr.const_debug_fmt($crate::try_!($fmt.field(stringify!($field_name)))));
    };

    // Tuple fields
    (@call_len_tuple $fmt:ident, $field_name:tt $(,)?) => {
        $field_name.const_debug_len($fmt.field());
    };
    (@call_len_tuple $fmt:ident, $field_name:tt => $expr:expr $(,)?) => {
        $expr.const_debug_len($fmt.field());
    };
    (@call_fmt_tuple $fmt:ident, $field_name:tt $(,)?) => {
        $crate::try_!($field_name.const_debug_fmt($crate::try_!($fmt.field())));
    };
    (@call_fmt_tuple $fmt:ident, $field_name:tt => $expr:expr $(,)?) => {
        $crate::try_!($expr.const_debug_fmt($crate::try_!($fmt.field())));
    };
}
