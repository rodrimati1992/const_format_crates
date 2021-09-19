macro_rules! with_shared_docs {(
    $(#[$before_clarification:meta])*
    ;clarification
    $(#[$before_syntax:meta])*
    ;syntax
    $(#[$after_syntax:meta])*
    ;limitations
    $item:item
) => (
    $(#[$before_clarification])*
    ///
    /// [For **examples** look here](#examples)
    ///
    /// This macro requires the "assertcp" feature to be exported.<br>
    ///
    $(#[$before_syntax])*
    /// # Syntax
    ///
    /// This macro uses the same syntax
    /// for the format string and formatting arguments as the
    /// [`formatcp`] macro.
    ///
    $(#[$after_syntax])*
    /// # Limitations
    ///
    /// This macro can only take constants of these types as arguments:
    ///
    /// - `&str`
    ///
    /// - `i*`/`u*` (all the primitive integer types).
    ///
    /// - `char`
    ///
    /// - `bool`
    ///
    /// This macro also has these limitations:
    ///
    /// - It can only use constants that involve concrete types,
    /// so while a `Type::<u8>::FOO` in an argument would be fine,
    /// `Type::<T>::FOO` would not be (`T` being a type parameter).
    ///
    /// - Integer arguments must have a type inferrable from context,
    /// [as described in the integer arguments section in the root module
    /// ](./index.html#integer-args).
    ///
    $item
)}

with_shared_docs! {
    /// Compile-time assertions with formatting.
    ///
    ;clarification
    ;syntax
    ;limitations
    ///
    /// # Examples
    ///
    /// ### Passing assertion
    ///
    /// ```rust
    /// use const_format::assertcp;
    ///
    /// use std::mem::align_of;
    ///
    /// assertcp!(
    ///     align_of::<&str>() == align_of::<usize>(),
    ///     "The alignment of `&str`({} bytes) and `usize`({} bytes) isn't the same?!?!",
    ///     align_of::<&str>(),
    ///     align_of::<usize>(),
    /// );
    ///
    /// # fn main(){}
    /// ```
    ///
    /// ### Failing assertion
    ///
    /// This example demonstrates a failing assertion,
    /// and how the compiler error looks like as of 2021-09-18.
    ///
    /// ```compile_fail
    /// use const_format::assertcp;
    ///
    /// const L: u64 = 2;
    /// const R: u32 = 5;
    ///
    /// assertcp!(L.pow(R) == 64, "{L} to the {R} isn't 64, it's {}", L.pow(R));
    ///
    /// # fn main(){}
    /// ```
    ///
    /// This is the compiler output:
    ///
    /// ```text
    /// error[E0080]: evaluation of constant value failed
    ///   --> src/macros/assertions/assertcp_macros.rs:124:11
    ///    |
    /// 10 | assertcp!(L.pow(R) == 64, "{L} to the {R} isn't 64, it's {}", L.pow(R));
    ///    |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at '
    /// assertion failed.
    /// 2 to the 5 isn't 64, it's 32
    /// ', src/macros/assertions/assertcp_macros.rs:10:11
    ///
    /// ```
    ///
    #[cfg_attr(feature = "docsrs", doc(cfg(feature = "assertc")))]
    #[macro_export]
    macro_rules! assertcp {
        ($($parameters:tt)*) => (
            $crate::__assertc_inner!{
                __formatcp_if_impl
                ($($parameters)*)
                ($($parameters)*)
            }
        );
    }
}
