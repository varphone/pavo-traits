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

/// 实现包装类型的 InnerCopy 契定。
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

/// 实现包装类型的 InnerRefer 契定。
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
