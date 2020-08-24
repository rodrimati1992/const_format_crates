/// For implementing DebugLike without using a derive macro.
#[macro_export]
macro_rules! impl_debug {
    (
        is_std_type;
        $($rem:tt)*
    ) => (
        $crate::__impl_debug_recursive!{
            impls[
                is_std_type = true;
            ]
            tokens[$($rem)*]
        }
    );
    (
        $($rem:tt)*
    ) => (
        $crate::__impl_debug_recursive!{
            impls[
                is_std_type = false;
            ]
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
        impls [
            is_std_type = $is_std_type:ident;
            $( $an_impl:tt )+
        ]

        $stuff:tt
    )=>{
        $(
            $crate::__impl_debug_inner!{
                @an_impl
                is_std_type = $is_std_type;
                $an_impl
                $stuff
            }

            $crate::__impl_debug_inner!{
                @impl_get_type_kind
                is_std_type = $is_std_type;
                $an_impl
            }
        )+
    };
    (@impl_get_type_kind
        is_std_type = true;
        (
            $(#[$impl_attr:meta])*
            impl[$($impl_:tt)*] $type:ty
            where[ $($where:tt)* ];
        )
    )=>{
        $(#[$impl_attr])*
        impl<$($impl_)*> $crate::pmr::GetTypeKind for $type
        where
            $($where)*
        {
            type Kind = $crate::pmr::IsStdKind;
            type This = Self;
        }

        $(#[$impl_attr])*
        impl<$($impl_)* __T> $crate::pmr::TypeKindMarker<IsStdKind, $type, __T>
        where
            $($where)*
        {
            #[inline(always)]
            pub const fn coerce(self, reference: &$type) -> PWrapper<$type> {
                PWrapper(*reference)
            }
        }
    };
    (@impl_get_type_kind
        is_std_type = false;
        (
            $(#[$impl_attr:meta])*
            impl[$($impl_:tt)*] $type:ty
            where[ $($where:tt)* ];
        )
    )=>{
        $(#[$impl_attr])*
        impl<$($impl_)*> $crate::pmr::GetTypeKind for $type
        where
            $($where)*
        {
            type Kind = $crate::pmr::IsNotStdKind;
            type This = Self;
        }
    };
    (@an_impl
        is_std_type = $is_std_type:ident;
        (
            $(#[$impl_attr:meta])*
            impl[$($impl_:tt)*] $type:ty
            where[ $($where:tt)* ];
        )
        (
            struct $type_name:ident {
                $( $field_name:ident $(: $renamed:ident)? $(=> $field_expr:expr )? ),*
                $(, .. $(@$nonexh:ident@)? )?
                $(,)?
            }
        )
    ) => (
        $(#[$impl_attr])*
        impl<$($impl_)*> $crate::__impl_debug_inner!(@self_ty $type, $is_std_type )
        where
            $($where)*
        {
            pub const fn const_debug_len(&self, f: &mut $crate::fmt::FormattingLength) {
                $crate::__impl_debug_inner!(@project_field self, this, $is_std_type);
                let Self{ $($field_name $(: $renamed )? ,)* $(.. $(@$nonexh@)? )?} = this;

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
                $crate::__impl_debug_inner!(@project_field self, this, $is_std_type);
                let Self{ $($field_name $(: $renamed )? ,)* ..} = this;

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
        is_std_type = $is_std_type:ident;
        (
            $(#[$impl_attr:meta])*
            impl[$($impl_:tt)*] $type:ty
            where[ $($where:tt)* ];
        )
        (
            struct $type_name:ident $((
                $( $field_name:ident $(=> $field_expr:expr )? ),*
                $(, .. $(@$nonexh:ident@)? )?
                $(,)?
            ))?$(;)?
        )
    ) => (

        $(#[$impl_attr])*
        impl<$($impl_)*> $crate::__impl_debug_inner!(@self_ty $type, $is_std_type )
        where
            $($where)*
        {
            pub const fn const_debug_len(&self, f: &mut $crate::fmt::FormattingLength) {
                $crate::__impl_debug_inner!(@project_field self, this, $is_std_type);
                let Self $( ( $($field_name,)* $(.. $(@$nonexh@)? )? ) )? = this;

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
                $crate::__impl_debug_inner!(@project_field self, this, $is_std_type);
                let Self $( ( $($field_name,)* ..) )? = this;

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
        is_std_type = $is_std_type:ident;
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
                $(, .. $(@$nonexh:ident@)? )?
                $(,)?
            }
        )
    ) => (
        $(#[$impl_attr])*
        impl<$($impl_)*> $crate::__impl_debug_inner!(@self_ty $type, $is_std_type )
        where
            $($where)*
        {
            pub const fn const_debug_len(&self, f: &mut $crate::fmt::FormattingLength) {
                $crate::__impl_debug_inner!(@project_field self, this, $is_std_type);

                match this {
                    $(
                        $crate::__impl_debug_enum!(
                            @pat
                            $type_name :: $variant $( {$($brace_ts)*} )? $( ($($paren_ts)*) )?
                        ) => {
                            $crate::__impl_debug_enum!{
                                @len_method
                                f = f;
                                $variant $( {$($brace_ts)*} )? $( ($($paren_ts)*) )?
                            }
                        }
                    )*
                    $(
                        _ $(@$nonexh@)? => { f.add_len("<unknown_variant>".len()); }
                    )?
                }
            }

            pub const fn const_debug_fmt(
                &self,
                f: &mut $crate::fmt::Formatter<'_>,
            ) -> $crate::pmr::Result<(), $crate::fmt::Error> {
                $crate::__impl_debug_inner!(@project_field self, this, $is_std_type);

                match this {
                    $(
                        $crate::__impl_debug_enum!(
                            @pat
                            $type_name :: $variant $( {$($brace_ts)*} )? $( ($($paren_ts)*) )?
                        ) => {
                            $crate::__impl_debug_enum!{
                                @fmt_method
                                f = f;
                                $variant $( {$($brace_ts)*} )? $( ($($paren_ts)*) )?
                            }
                        }
                    )*
                    $(
                        _ $(@$nonexh@)? => f.w().write_whole_str("<unknown_variant>"),
                    )?
                }
            }
        }
    );
    (@an_impl
        is_std_type = $is_std_type:ident;
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
    );
    (@project_field $self:ident, $this:ident, /*is_std_type*/ true)=>{
        let $this = &$self.0;
    };
    (@project_field $self:ident, $this:ident, /*is_std_type*/ false)=>{
        let $this = $self;
    };
    (@self_ty $self:ty, /*is_std_type*/ true )=>{
        $crate::pmr::PWrapper<$self>
    };
    (@self_ty $self:ty, /*is_std_type*/ false )=>{
        $self
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_debug_enum {
    (@pat
        $type_name:ident :: $variant:ident {
            $( $field_name:ident $(:$renamed:ident)? $(=> $field_expr:expr )? ),*
            $(, .. $(@$nonexh:ident@)? )?
            $(,)?
        }
    )=>{
        $type_name::$variant{
            $( $field_name $(:$renamed)?, )*
            $(.. $(@$nonexh@)? )?
        }
    };
    (@pat
        $type_name:ident :: $variant:ident $((
            $( $field_name:ident $(=> $field_expr:expr )? ),*
            $(, .. $(@$nonexh:ident@)? )?
            $(,)?
        ))?
    )=>{
        $type_name::$variant $((
            $( $field_name, )*
            $(.. $(@$nonexh@)? )?
        ))?
    };
    (@len_method
        f = $f:ident;
        $variant:ident {
            $( $field_name:ident $(:$renamed:ident)? $(=> $field_expr:expr )? ),*
            $(, .. $(@$nonexh:ident@)? )?
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
            $( $field_name:ident $(=> $field_expr:expr )? ),*
            $(, .. $(@$nonexh:ident@)? )?
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
            $( $field_name:ident $(:$renamed:ident)? $(=> $field_expr:expr )? ),*
            $(, .. $(@$nonexh:ident@)? )?
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
            $( $field_name:ident $(=> $field_expr:expr )? ),*
            $(, .. $(@$nonexh:ident@)? )?
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
    (@call_len $fmt:ident, $field_name:ident $(,)?) => {
        $field_name.const_debug_len($fmt.field(stringify!($field_name)));
    };
    (@call_len $fmt:ident, $field_name:ident : $renamed:ident $(,)?) => {
        $renamed.const_debug_len($fmt.field(stringify!($field_name)));
    };
    (@call_len $fmt:ident, $field_name:ident $(: $renamed:ident)? => $expr:expr $(,)?) => {
        $expr.const_debug_len($fmt.field(stringify!($field_name)));
    };
    (@call_fmt $fmt:ident, $field_name:ident $(,)?) => {
        $crate::try_!(
            $field_name.const_debug_fmt($crate::try_!($fmt.field(stringify!($field_name))))
        );
    };
    (@call_fmt $fmt:ident, $field_name:ident : $renamed:ident $(,)?) => {
        $crate::try_!($renamed.const_debug_fmt($crate::try_!($fmt.field(stringify!($field_name)))));
    };
    (@call_fmt $fmt:ident, $field_name:ident $(: $renamed:ident)? => $expr:expr $(,)?) => {
        $crate::try_!($expr.const_debug_fmt($crate::try_!($fmt.field(stringify!($field_name)))));
    };

    // Tuple fields
    (@call_len_tuple $fmt:ident, $field_name:ident $(,)?) => {
        $field_name.const_debug_len($fmt.field());
    };
    (@call_len_tuple $fmt:ident, $field_name:ident => $expr:expr $(,)?) => {
        $expr.const_debug_len($fmt.field());
    };
    (@call_fmt_tuple $fmt:ident, $field_name:ident $(,)?) => {
        $crate::try_!($field_name.const_debug_fmt($crate::try_!($fmt.field())));
    };
    (@call_fmt_tuple $fmt:ident, $field_name:ident => $expr:expr $(,)?) => {
        $crate::try_!($expr.const_debug_fmt($crate::try_!($fmt.field())));
    };
}
