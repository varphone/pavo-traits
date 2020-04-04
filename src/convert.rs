//! 类型转换类契定。
//!

/// 定义将只读引用转化至常量指针的契定。
pub trait AsPtr<T>: AsRef<T> {
    /// 将当前只读引用转化至常量指针。
    ///
    /// # Safety
    ///
    /// 强转指针属于危险操作，请务必确保其安全性。
    unsafe fn as_ptr(&self) -> *const T;
}

/// 定义将只读引用转化至可写指针的契定。
pub trait AsPtrMut<T>: AsPtr<T> {
    /// 将当前只读引用转化至可写指针。
    ///
    /// # Safety
    ///
    /// 强转指针属于危险操作，请务必确保其安全性。
    unsafe fn as_ptr_mut(&self) -> *mut T;
}

/// 用于帮助实现 AsRef<T> 契定的宏。
#[macro_export]
macro_rules! impl_as_ref {
    ($Type:ty) => {
        impl AsRef<$Type> for $Type {
            fn as_ref(&self) -> &$Type {
                &*self
            }
        }

        // impl AsRef<$Type> for &$Type {
        //     fn as_ref(&self) -> &$Type {
        //         &**self
        //     }
        // }
    };

    ($Type:ty, $Target:ty) => {
        impl AsRef<$Target> for $Type {
            fn as_ref(&self) -> &$Target {
                &*self as &$Target
            }
        }

        // impl AsRef<$Target> for &$Type {
        //     fn as_ref(&self) -> &$Target {
        //         &**self as &$Target
        //     }
        // }
    };

    ($Type:ty, $Target:ty, $Expr:tt) => {
        impl AsRef<$Target> for $Type {
            fn as_ref(&self) -> &$Target {
                &self.$Expr
            }
        }

        // impl AsRef<$Target> for &$Type {
        //     fn as_ref(&self) -> &$Target {
        //         &*self.$Expr
        //     }
        // }
    };
}

/// 用于帮助实现 AsPtr<T> 契定的宏。
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
        impl AsPtr<$Type> for $Type {
            unsafe fn as_ptr(&self) -> *const $Type {
                self.as_ref() as *const $Type
            }
        }

        // impl AsPtr<$Type> for &$Type {
        //     fn as_ptr(&self) -> *const $Type {
        //         self.as_ref() as *const $Type
        //     }
        // }
    };

    ($Type:ty, $Target:ty) => {
        impl AsPtr<$Target> for $Type {
            unsafe fn as_ptr(&self) -> *const $Target {
                self.as_ref() as *const $Target
            }
        }

        // impl AsPtr<$Target> for &$Type {
        //     unsafe fn as_ptr(&self) -> *const $Target {
        //         self.as_ref() as *const $Target
        //     }
        // }
    };
}

/// 用于帮助实现 AsPtrMut<T> 契定的宏。
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
        impl AsPtrMut<$Type> for $Type {
            unsafe fn as_ptr_mut(&self) -> *mut $Type {
                self.as_ptr() as *const $Type as *mut $Type
            }
        }

        // impl AsPtrMut<$Type> for &$Type {
        //     unsafe fn as_ptr_mut(&self) -> *mut $Type {
        //         self.as_ptr() as *const $Type as *mut $Type
        //     }
        // }
    };

    ($Type:ty, $Target:ty) => {
        impl AsPtrMut<$Target> for $Type {
            unsafe fn as_ptr_mut(&self) -> *mut $Target {
                self.as_ptr() as *const $Target as *mut $Target
            }
        }

        // impl AsPtrMut<$Target> for &$Type {
        //     unsafe fn as_ptr_mut(&self) -> *mut $Target {
        //         self.as_ptr() as *cont $Target as *mut $Target
        //     }
        // }
    };
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
    impl_as_ptr!(Foo);
    impl_as_ptr_mut!(Foo);

    fn read_ptr<T>(t: &T) -> usize
    where
        T: AsPtrMut<Foo>,
    {
        unsafe { std::ptr::read(t.as_ptr() as *const usize) }
    }

    fn write_ptr_mut<T>(t: &T)
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
        }
    }
}
