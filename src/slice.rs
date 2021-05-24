//! 数组类契定。
//！

trait CloneFromSliceFlex<T: Clone> {
    /// Copies the elements from `src` into `self`.
    ///
    /// The length of `src` could be different to `self`.
    fn clone_from_slice_flex(&mut self, src: &[T]);
}

impl<T: Clone> CloneFromSliceFlex<T> for [T] {
    fn clone_from_slice_flex(&mut self, src: &[T]) {
        let len = self.len().min(src.len());
        for i in 0..len {
            self[i].clone_from(&src[i]);
        }
    }
}

trait CopyFromSliceFlex<T: Copy> {
    /// Copies all elements from `src` into `self`, using a memcpy.
    ///
    /// The length of `src` could be different to `self`.
    fn copy_from_slice_flex(&mut self, src: &[T]);
}

impl<T: Copy> CopyFromSliceFlex<T> for [T] {
    fn copy_from_slice_flex(&mut self, src: &[T]) {
        let len = self.len().min(src.len());
        unsafe {
            std::ptr::copy_nonoverlapping(src.as_ptr(), self.as_mut_ptr(), len);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clone_from_slice_flex() {
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
        struct Bar(usize);

        let mut a4 = [Bar(1), Bar(2), Bar(3), Bar(4)];
        a4.clone_from_slice_flex(&[]);
        assert_eq!(a4, [Bar(1), Bar(2), Bar(3), Bar(4)]);
        a4.clone_from_slice_flex(&[Bar(5)]);
        assert_eq!(a4, [Bar(5), Bar(2), Bar(3), Bar(4)]);
        a4.clone_from_slice_flex(&[Bar(5), Bar(6)]);
        assert_eq!(a4, [Bar(5), Bar(6), Bar(3), Bar(4)]);
        a4.clone_from_slice_flex(&[Bar(5), Bar(6), Bar(7)]);
        assert_eq!(a4, [Bar(5), Bar(6), Bar(7), Bar(4)]);
        a4.clone_from_slice_flex(&[Bar(5), Bar(6), Bar(7), Bar(8)]);
        assert_eq!(a4, [Bar(5), Bar(6), Bar(7), Bar(8)]);
        a4.clone_from_slice_flex(&[Bar(5), Bar(6), Bar(7), Bar(8), Bar(9)]);
        assert_eq!(a4, [Bar(5), Bar(6), Bar(7), Bar(8)]);
    }

    #[test]
    fn test_copy_from_slice_flex() {
        let mut a4 = [1, 2, 3, 4];
        a4.copy_from_slice_flex(&[]);
        assert_eq!(a4, [1, 2, 3, 4]);
        a4.copy_from_slice_flex(&[5]);
        assert_eq!(a4, [5, 2, 3, 4]);
        a4.copy_from_slice_flex(&[5, 6]);
        assert_eq!(a4, [5, 6, 3, 4]);
        a4.copy_from_slice_flex(&[5, 6, 7]);
        assert_eq!(a4, [5, 6, 7, 4]);
        a4.copy_from_slice_flex(&[5, 6, 7, 8]);
        assert_eq!(a4, [5, 6, 7, 8]);
        a4.copy_from_slice_flex(&[5, 6, 7, 8, 9]);
        assert_eq!(a4, [5, 6, 7, 8]);
    }
}
