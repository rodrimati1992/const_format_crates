#![allow(non_camel_case_types)]

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
#[cfg(feature = "derive")]
pub struct ConstDebugWhereClause;

/// ```rust
/// #![feature(const_mut_refs)]
///
/// use const_format::StrWriterMut;
///
/// let mut len = 0;
/// let mut buffer = [0; 128];
///
/// let mut writer = StrWriterMut::from_custom(&mut buffer, &mut len);
///
/// writer.write_str("hello").unwrap();
///
/// assert_eq!(writer.as_bytes(), b"hello")
///
/// ```
///
/// ```compile_fail
/// #![feature(const_mut_refs)]
///
/// use const_format::StrWriterMut;
///
/// let mut len = 0;
/// let mut buffer = [0; 128];
///
/// let mut writer = StrWriterMut::from_custom(&mut buffer, &mut len);
///
/// writer.write_str("hello").unwrap();
///
/// assert_eq!(writer.as_str(), "hello")
///
/// ```
///
pub struct AsStr_For_StrWriterMut_NoEncoding;

/// ```rust
/// #![feature(const_mut_refs)]
/// #![feature(const_panic)]
///
/// const_format::assertc!(true, "foo");
///
/// ```
///
/// ```compile_fail
/// #![feature(const_mut_refs)]
/// #![feature(const_panic)]
///
/// const_format::assertc!(false, "foo");
///
/// ```
///
#[cfg(feature = "assert")]
pub struct Assert;
