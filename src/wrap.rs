//! 类型包装类契定。
//!

/// 定义内部类型拷贝契定。
pub trait InnerCopy<T> {
    /// 返回内部类型拷贝。
    fn inner(&self) -> T;
}

/// 定义内部类型引用契定。
pub trait InnerRefer<T> {
    /// 返回内部类型引用。
    fn inner(&self) -> &T;

    /// 返回内部类型可变引用。
    fn inner_mut(&mut self) -> &mut T;
}

/// 实现包装枚举的 [From] 及 [Into] 特性。
///
/// [From]: https://doc.rust-lang.org/std/convert/trait.From.html
/// [Into]: https://doc.rust-lang.org/std/convert/trait.Into.html
///
/// # Examples
///
/// ```
/// use pavo_traits::{impl_from_into_for_enum};
///
/// mod ffi {
///     // The enum in ffi with C style.
///     #[repr(u32)]
///     #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd)]
///     pub enum MODE_E {
///         MODE_E_A,
///         MODE_E_B,
///         MODE_E_C,
///     }
///
///     // The API of argumented with MODE_E.
///     pub fn set_mode(mode: MODE_E) -> MODE_E {
///         mode
///     }
/// }
///
/// // The enum wrapped with Rust style.
/// #[repr(u32)]
/// #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd)]
/// pub enum Mode {
///     A,
///     B,
///     C,
/// }
///
/// // impl From<ffi::MODE_E>/Into<ffi::MODE_E> for Mode.
/// impl_from_into_for_enum!(Mode, ffi::MODE_E);
///
/// // Use from/into to convert the types.
/// assert_eq!(Mode::from(ffi::set_mode(Mode::A.into())), Mode::A);
/// ```
#[macro_export]
macro_rules! impl_from_into_for_enum {
    ($Wrapper:ty, $Inner:ty) => {
        impl From<$Inner> for $Wrapper {
            fn from(val: $Inner) -> Self {
                unsafe { std::mem::transmute::<$Inner, Self>(val) }
            }
        }

        impl Into<$Inner> for $Wrapper {
            fn into(self) -> $Inner {
                unsafe { std::mem::transmute::<Self, $Inner>(self) }
            }
        }
    };
}

/// 实现包装结构的 [From] 及 [Into] 契定。
///
/// [From]: https://doc.rust-lang.org/std/convert/trait.From.html
/// [Into]: https://doc.rust-lang.org/std/convert/trait.Into.html
///
/// # Examples
///
/// ```
/// use pavo_traits::{impl_from_into_for_struct};
///
/// mod ffi {
///     // The struct in ffi with C style.
///     #[repr(C)]
///     #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd)]
///     pub struct DATA_S {
///         a: usize,
///         b: usize,
///         c: usize,
///         d: usize,
///     }
///
///     // The API of argumented with DATA_S.
///     pub fn set_data(data: &DATA_S) -> DATA_S {
///         *data
///     }
/// }
///
/// // The struct wrapped with Rust style.
/// #[repr(C)]
/// #[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// pub struct Data {
///     inner: ffi::DATA_S,
/// }
///
/// // impl From<ffi::DATA_S>/Into<ffi::DATA_S> for Data.
/// impl_from_into_for_struct!(Data, ffi::DATA_S);
///
/// impl Data {
///     fn new() -> Self {
///         Self { inner: Default::default() }
///     }
/// }
///
/// // Use from/into to convert the types.
/// let d = Data::new();
/// assert_eq!(Data::from(ffi::set_data(&d.into())), d);
/// ```
#[macro_export]
macro_rules! impl_from_into_for_struct {
    ($Wrapper:ty, $Inner:ty) => {
        impl From<$Inner> for $Wrapper {
            fn from(val: $Inner) -> Self {
                Self { inner: val }
            }
        }

        impl Into<$Inner> for $Wrapper {
            fn into(self) -> $Inner {
                self.inner
            }
        }
    };
}

/// 实现包装类型的 [InnerCopy] 契定。
///
/// [InnerCopy]: trait.InnerCopy.html
///
/// # Examples
///
/// ```
/// use pavo_traits::{impl_inner_copy, InnerCopy};
///
/// #[derive(Clone, Copy)]
/// struct Bar {}
///
/// struct Foo {
///    inner: Bar,
/// }
///
/// impl_inner_copy!(Foo, Bar);
/// ```
#[macro_export]
macro_rules! impl_inner_copy {
    ($Wrapper:ty, $Inner:ty) => {
        impl InnerCopy<$Inner> for $Wrapper {
            fn inner(&self) -> $Inner {
                self.inner
            }
        }
    };
}

/// 实现包装类型的 [InnerRefer] 契定。
///
/// [InnerRefer]: trait.InnerRefer.html
///
/// # Examples
///
/// ```
/// use pavo_traits::{impl_inner_refer, InnerRefer};
///
/// struct Bar {}
///
/// struct Foo {
///    inner: Bar,
/// }
///
/// impl_inner_refer!(Foo, Bar);
/// ```
#[macro_export]
macro_rules! impl_inner_refer {
    ($Wrapper:ty, $Inner:ty) => {
        impl InnerRefer<$Inner> for $Wrapper {
            fn inner(&self) -> &$Inner {
                &self.inner
            }

            fn inner_mut(&mut self) -> &mut $Inner {
                &mut self.inner
            }
        }
    };
}

/// 实现包装结构的通用契定。
/// 包括：[AsRef], [AsPtr], [AsPtrMut], [InnerRefer]。
///
/// [AsRef]: https://doc.rust-lang.org/std/convert/trait.AsRef.html
/// [AsPtr]: trait.AsPtr.html
/// [AsPtrMut]: trait.AsPtrMut.html
/// [InnerRefer]: trait.InnerRefer.html
///
/// # Examples
///
/// ```
/// use pavo_traits::{impl_struct_wrapper, AsPtr, AsPtrMut, InnerRefer};
///
/// #[derive(Clone, Copy, Debug)]
/// struct Bar {}
///
/// #[derive(Debug)]
/// struct Foo {
///    inner: Bar,
/// }
///
/// impl_struct_wrapper!(Foo, Bar);
///
/// let f = Foo { inner: Bar {} };
/// assert_eq!(std::ptr::eq(f.as_ref(), &f.inner), true);
/// assert_eq!(std::ptr::eq(f.as_ref(), f.inner()), true);
/// assert_eq!(std::ptr::eq(f.inner(), &f.inner), true);
/// ```
#[macro_export(local_inner_macros)]
macro_rules! impl_struct_wrapper {
    ($Wrapper:ty, $Inner:ty) => {
        impl_as_ref!($Wrapper, $Inner, inner);
        impl_as_ptr!($Wrapper, $Inner);
        impl_as_ptr_mut!($Wrapper, $Inner);
        impl_from_into_for_struct!($Wrapper, $Inner);
        impl_inner_refer!($Wrapper, $Inner);
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[repr(C)]
    #[derive(Debug)]
    struct FooCopy {
        inner: usize,
    }

    impl_inner_copy!(FooCopy, usize);

    #[test]
    fn test_inner_copy() {
        let f = FooCopy { inner: 123 };
        let v = f.inner();
        assert_eq!(v, f.inner);
    }

    #[repr(C)]
    #[derive(Debug)]
    struct FooRefer {
        inner: Arc<usize>,
    }

    impl_inner_refer!(FooRefer, Arc<usize>);

    #[test]
    fn test_inner_refer() {
        let mut f = FooRefer {
            inner: Arc::new(123),
        };
        assert_eq!(&**f.inner(), &123usize);
        assert_eq!(std::ptr::eq(f.inner(), &f.inner), true);
        *Arc::make_mut(f.inner_mut()) = 456;
        assert_eq!(&**f.inner(), &456usize);
    }
}
