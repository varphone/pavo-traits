//! 类型转换类契定。
//!

/// 定义将只读引用转化至常量指针的契定。
pub trait AsPtr<T>: AsRef<T> {
    /// 将当前只读引用转化至常量指针。
    ///
    /// # Safety
    ///
    /// 强转指针属于危险操作，请务必确保其安全性。
    unsafe fn as_ptr(&self) -> *const T {
        AsRef::<T>::as_ref(self) as *const T
    }
}

/// 定义将只读引用转化至可写指针的契定。
pub trait AsPtrMut<T>: AsPtr<T> {
    /// 将当前只读引用转化至可写指针。
    ///
    /// # Safety
    ///
    /// 强转指针属于危险操作，请务必确保其安全性。
    unsafe fn as_ptr_mut(&self) -> *mut T {
        AsPtr::<T>::as_ptr(self) as *mut T
    }
}

/// 用于帮助实现 [AsRef] 契定的宏。
///
/// [AsRef]: https://doc.rust-lang.org/std/convert/trait.AsRef.html
///
/// # Examples
///
/// ```
/// use pavo_traits::{impl_as_ref};
///
/// struct Bar {}
///
/// struct Foo {
///     bar: Bar,
/// }
///
/// impl_as_ref!(Foo);
/// // Comment/Uncomment to select one of the follow lines.
/// // impl_as_ref!(Foo, Bar); // Exclusive with the bellow.
/// impl_as_ref!(Foo, Bar, bar); // Exclusive with the above.
/// ```
#[macro_export]
macro_rules! impl_as_ref {
    ($Type:ty) => {
        impl AsRef<$Type> for $Type {
            fn as_ref(&self) -> &$Type {
                self
            }
        }
    };

    ($Type:ty, $Target:ty) => {
        impl AsRef<$Target> for $Type {
            fn as_ref(&self) -> $Target {
                self as &$Target
            }
        }
    };

    ($Type:ty, $Target:ty, $Expr:tt) => {
        impl AsRef<$Target> for $Type {
            fn as_ref(&self) -> &$Target {
                &self.$Expr
            }
        }
    };
}

/// 用于帮助实现 [AsMut] 契定的宏。
///
/// [AsMut]: https://doc.rust-lang.org/std/convert/trait.AsMut.html
///
/// # Examples
///
/// ```
/// use pavo_traits::{impl_as_mut};
///
/// struct Bar {}
///
/// struct Foo {
///     bar: Bar,
/// }
///
/// impl_as_mut!(Foo);
/// // Comment/Uncomment to select one of the follow lines.
/// // impl_as_mut!(Foo, Bar); // Exclusive with the bellow.
/// impl_as_mut!(Foo, Bar, bar); // Exclusive with the above.
/// ```
#[macro_export]
macro_rules! impl_as_mut {
    ($Type:ty) => {
        impl AsMut<$Type> for $Type {
            fn as_mut(&mut self) -> &mut $Type {
                self
            }
        }
    };

    ($Type:ty, $Target:ty) => {
        impl AsMut<$Target> for $Type {
            fn as_mut(&mut self) -> &mut $Target {
                unsafe { std::mem::transmute::<&mut $Type, &mut $Target>(self) }
            }
        }
    };

    ($Type:ty, $Target:ty, $Expr:tt) => {
        impl AsMut<$Target> for $Type {
            fn as_mut(&mut self) -> &mut $Target {
                &mut self.$Expr
            }
        }
    };
}

/// 用于帮助实现 `AsMut + AsRef` 等契定的宏。
#[macro_export(local_inner_macros)]
macro_rules! impl_as_mut_and_ref {
    ($($x:tt)*) => {
        impl_as_mut!($($x)*);
        impl_as_ref!($($x)*);
    }
}

/// 用于帮助实现 [AsPtr] 契定的宏。
///
/// [AsPtr]: trait.AsPtr.html
///
/// # Examples
///
/// ```
/// use pavo_traits::{impl_as_ref, impl_as_ptr, AsPtr};
///
/// struct Bar {}
/// struct Foo {
///     inner: Bar,
/// }
///
/// impl_as_ref!(Foo);
/// impl_as_ref!(Foo, Bar, inner);
/// impl_as_ptr!(Foo);
/// impl_as_ptr!(Foo, Bar);
/// ```
#[macro_export]
macro_rules! impl_as_ptr {
    ($Type:ty) => {
        impl AsPtr<$Type> for $Type {}
    };

    ($Type:ty, $Target:ty) => {
        impl AsPtr<$Target> for $Type {}
    };

    ($Type:ty, $Target:ty, $Expr:tt) => {
        impl_as_ptr!($Type, $Target);
    };
}

/// 用于帮助实现 [AsPtrMut] 契定的宏。
///
/// [AsPtrMut]: trait.AsPtrMut.html
///
/// # Examples
///
/// ```
/// use pavo_traits::{impl_as_ref, impl_as_ptr, impl_as_ptr_mut, AsPtr, AsPtrMut};
///
/// struct Bar {}
/// struct Foo {
///     inner: Bar,
/// }
///
/// impl_as_ref!(Foo);
/// impl_as_ref!(Foo, Bar, inner);
/// impl_as_ptr!(Foo);
/// impl_as_ptr!(Foo, Bar);
/// impl_as_ptr_mut!(Foo);
/// impl_as_ptr_mut!(Foo, Bar);
/// ```
#[macro_export]
macro_rules! impl_as_ptr_mut {
    ($Type:ty) => {
        impl AsPtrMut<$Type> for $Type {}
    };

    ($Type:ty, $Target:ty) => {
        impl AsPtrMut<$Target> for $Type {}
    };

    ($Type:ty, $Target:ty, $Expr:tt) => {
        impl_as_mut!($Type, $Target);
    };
}

/// 用于帮助在单个类型上实现 `AsRef + AsMut + AsPtr + AsPtrMut` 等契定的宏。
#[macro_export(local_inner_macros)]
macro_rules! impl_as_bundle {
    () => {};
    ($($x:tt)*) => {
        impl_as_ref!($($x)*);
        impl_as_mut!($($x)*);
        impl_as_ptr!($($x)*);
        impl_as_ptr_mut!($($x)*);
    };
}

/// 用于帮助在多个类型上实现 `AsRef + AsMut + AsPtr + AsPtrMut` 等契定的宏。
#[macro_export(local_inner_macros)]
macro_rules! impl_as_bundle_many {
    ($($Type:ty),* $(,)?) => {
        $(
            impl_as_ref!($Type);
            impl_as_mut!($Type);
            impl_as_ptr!($Type);
            impl_as_ptr_mut!($Type);
        )*
    }
}

// Auto impl AsPtr<U> for &T
impl<T, U> AsPtr<U> for &T
where
    T: AsPtr<U>,
    T: Sized,
    U: Sized,
{
}

// Auto impl AsPtrMut<U> for &T
impl<T, U> AsPtrMut<U> for &T
where
    T: AsPtrMut<U>,
    T: Sized,
    U: Sized,
{
}

#[cfg(test)]
mod tests {
    use super::*;

    #[repr(C)]
    #[derive(Debug)]
    struct Foo {
        v: usize,
    }

    impl_as_ref!(Foo);
    impl_as_ref!(Foo, usize, v);
    impl_as_ptr!(Foo);
    impl_as_ptr!(Foo, usize);
    impl_as_ptr_mut!(Foo);

    fn read_ptr<T>(t: T) -> usize
    where
        T: AsPtrMut<Foo>,
    {
        unsafe { std::ptr::read(t.as_ptr() as *const usize) }
    }

    fn read_ptr2<T>(t: T) -> usize
    where
        T: AsPtr<usize>,
    {
        unsafe { std::ptr::read(t.as_ptr()) }
    }

    fn write_ptr_mut<T>(t: T)
    where
        T: AsPtrMut<Foo>,
    {
        unsafe {
            std::ptr::write(t.as_ptr_mut() as *mut usize, 456);
        }
    }

    #[test]
    fn test_as_ptr() {
        unsafe {
            let f = Foo { v: 123 };
            assert_eq!(f.as_ptr(), &f as *const Foo);
            let v = std::ptr::read(f.as_ptr() as *const usize);
            assert_eq!(v, f.v);
            let v2 = read_ptr(&f);
            assert_eq!(v, v2);
            let v3 = read_ptr2(&f);
            assert_eq!(v2, v3);
            let v4 = read_ptr(f);
            assert_eq!(v3, v4);
        }
    }

    #[test]
    fn test_as_ptr_mut() {
        unsafe {
            let f = Foo { v: 123 };
            assert_eq!(f.as_ptr_mut(), &f as *const Foo as *mut Foo);
            std::ptr::write(f.as_ptr_mut() as *mut usize, 321);
            assert_eq!(321, f.v);
            write_ptr_mut(&f);
            assert_eq!(456, f.v);
            write_ptr_mut(f);
        }
    }
}
