///
/// ```rust
///
/// struct Foo<T>(T);
///
/// const_format::impl_fmt!{
///     impl[T,] Foo<T>
///     where[T: 'static,];
///
///     fn foo(){}
///
/// }
/// ```
///
/// ```compile_fail
///
/// struct Foo<T>(T);
///
/// const_format::impl_fmt!{
///     impl[T,] Foo<T>
///     where[asodkaspodaoskd,];
///
///     fn foo(){}
/// }
/// ```
///
/// ```compile_fail
///
/// struct Foo<T>(T);
///
/// const_format::impl_fmt!{
///     impl[T,] Foo<T>
///     where[T: T];
///
///     fn foo(){}
/// }
/// ```
///
pub struct ImplFmtWhereClause;

///
/// ```rust
/// #![feature(const_mut_refs)]
///
/// #[derive(const_format::ConstDebug)]
/// struct Foo<T>(*const T)
/// where T: 'static;
///
/// fn main(){}
/// ```
///
/// ```compile_fail
/// #![feature(const_mut_refs)]
///
/// #[derive(const_format::ConstDebug)]
/// struct Foo<T>(*const T)
/// where AAAA: AAAA;
///
/// fn main(){}
/// ```
///
pub struct ConstDebugWhereClause;
