#[macro_use]
#[cfg(feature = "with_fmt")]
mod call_debug_fmt;

#[macro_use]
mod constructors;

#[macro_use]
mod helper_macros;

#[macro_use]
mod fmt_macros;

#[macro_use]
#[cfg(feature = "with_fmt")]
mod impl_fmt;

/// Equivalent to the old `try` macro, or the `?` operator.
#[macro_export]
macro_rules! try_ {
    ($e:expr) => {
        match $e {
            $crate::pmr::Ok(x) => x,
            $crate::pmr::Err(e) => return Err(e),
        }
    };
}

/// Equivalent to `Result::unwrap_or_else` but allows returning from the enclosing function.
#[macro_export]
macro_rules! unwrap_or_else {
    ($e:expr, |$error:ident| $orelse:expr ) => {
        match $e {
            $crate::pmr::Ok(x) => x,
            $crate::pmr::Err($error) => $orelse,
        }
    };
}

/// Coerces a reference to a type that has a `const_*_fmt` method.
///
/// # Behavior
///
/// For arrays it coerces them into a slice, and wraps them in a [`PWrapper`].
///
/// For std types, it wraps them in a [`PWrapper`], which implements the
/// `const_*_fmt` methods.
///
/// For std types, it just returns back the same reference.
///
/// [`PWrapper`]: ./
#[cfg(feature = "with_fmt")]
#[macro_export]
macro_rules! coerce_to_fmt {
    ($reference:expr) => {{
        match $reference {
            ref reference => {
                let mut marker = $crate::pmr::TypeKindMarker::NEW;
                if false {
                    marker = marker.infer_type(reference);
                }
                marker.coerce(marker.unreference(reference))
            }
        }
    }};
}

macro_rules! std_kind_impl {
    (
        impl[$($impl:tt)*] $self:ty
        $(where[ $($where_:tt)* ])?
    )=>{
        impl<$($impl)*> $crate::pmr::GetTypeKind for $self
        where
            $($($where_)*)?
        {
            type Kind = $crate::pmr::IsStdKind;
            type This = Self;
        }

        impl<$($impl)* __T> $crate::pmr::TypeKindMarker<$crate::pmr::IsStdKind, $self, __T>
        where
            $($($where_)*)?
        {
            #[inline(always)]
            pub const fn coerce(self, reference: &$self) -> $crate::pmr::PWrapper<$self> {
                $crate::pmr::PWrapper(*reference)
            }
        }
    }
}
