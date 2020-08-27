/// For implementing DebugLike without using a derive macro.
#[macro_export]
macro_rules! impl_fmt {
    (
        is_std_type;
        $($rem:tt)*
    ) => (
        $crate::__impl_fmt_recursive!{
            impls[
                is_std_type = true;
            ]
            tokens[$($rem)*]
        }
    );
    (
        $($rem:tt)*
    ) => (
        $crate::__impl_fmt_recursive!{
            impls[
                is_std_type = false;
            ]
            tokens[$($rem)*]
        }
    );
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_fmt_recursive{
    (
        impls[$($impls:tt)*]

        tokens[
            $(#[$impl_attr:meta])*
            impl[$($impl_:tt)*] $type:ty
            $(where[ $($where:tt)* ])?;

            $($rem:tt)*
        ]
    ) => (
        $crate::__impl_fmt_recursive!{

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
    // The same as the above macro branch, but it doesn't require the `[]` in `impl[]`
    (
        impls[$($impls:tt)*]

        tokens[
            $(#[$impl_attr:meta])*
            impl $type:ty
            $(where[ $($where:tt)* ])?;

            $($rem:tt)*
        ]
    ) => (
        $crate::__impl_fmt_recursive!{

            impls[
                $($impls)*
                (
                    $(#[$impl_attr])*
                    #[allow(unused_mut)]
                    impl[] $type
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
        $crate::__impl_fmt_inner!{
            @all_impls
            impls $impls
            ($($rem)*)
        }
    );
}

#[doc(hidden)]
#[macro_export]
macro_rules! __impl_fmt_inner {
    (@all_impls
        impls [
            is_std_type = $is_std_type:ident;
            $( $an_impl:tt )+
        ]

        $stuff:tt
    )=>{
        $(
            $crate::__impl_fmt_inner!{
                @impl_get_type_kind
                is_std_type = $is_std_type;
                $an_impl
            }

            $crate::__impl_fmt_inner!{
                @an_impl
                is_std_type = $is_std_type;
                $an_impl
                $stuff
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
        impl<$($impl_)*> $crate::pmr::FormatMarker for $type
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
        impl<$($impl_)*> $crate::pmr::FormatMarker for $type
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
            $($everything:tt)*
        )
    )=>{
        $(#[$impl_attr])*
        impl<$($impl_)*> $crate::__impl_fmt_inner!(@self_ty $type, $is_std_type )
        where
            $($where)*
        {
            $($everything)*
        }
    };

    (@self_ty $self:ty, /*is_std_type*/ true )=>{
        $crate::pmr::PWrapper<$self>
    };
    (@self_ty $self:ty, /*is_std_type*/ false )=>{
        $self
    };

}
