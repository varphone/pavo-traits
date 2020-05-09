//! 数值类契定。
//！

/// 定义将数值向下对齐到指定倍数的契定。
pub trait AlignDownwards {
    /// 将数值向下对齐到指定倍数。
    ///
    /// # Examples
    ///
    /// ```
    /// # use pavo_traits::{AlignDownwards};
    /// assert_eq!(63.align_downwards(64), 0);
    /// assert_eq!(65.align_downwards(64), 64);
    /// ```
    fn align_downwards(self, align: Self) -> Self;
}

/// 定义将数值向上对齐到指定倍数的契定。
pub trait AlignUpwards {
    /// 将数值向上对齐到指定倍数。
    ///
    /// # Examples
    ///
    /// ```
    /// # use pavo_traits::{AlignUpwards};
    /// assert_eq!(63.align_upwards(64), 64);
    /// assert_eq!(65.align_upwards(64), 128);
    /// ```
    fn align_upwards(self, align: Self) -> Self;
}

macro_rules! impl_align_downwards {
    ($Type:ty) => {
        impl AlignDownwards for $Type {
            fn align_downwards(self, align: Self) -> Self {
                self - (self % align)
            }
        }
    };
}

macro_rules! impl_align_upwards {
    ($Type:ty) => {
        impl AlignUpwards for $Type {
            fn align_upwards(self, align: Self) -> Self {
                if (self % align) != 0 {
                    self + align - (self % align)
                } else {
                    self
                }
            }
        }
    };
}

impl_align_downwards!(i8);
impl_align_downwards!(i16);
impl_align_downwards!(i32);
impl_align_downwards!(i64);
impl_align_downwards!(isize);
impl_align_downwards!(u8);
impl_align_downwards!(u16);
impl_align_downwards!(u32);
impl_align_downwards!(u64);
impl_align_downwards!(usize);

impl_align_upwards!(i8);
impl_align_upwards!(i16);
impl_align_upwards!(i32);
impl_align_upwards!(i64);
impl_align_upwards!(isize);
impl_align_upwards!(u8);
impl_align_upwards!(u16);
impl_align_upwards!(u32);
impl_align_upwards!(u64);
impl_align_upwards!(usize);

/// 定义将值限制在指定范围内的契定。
pub trait Clamped {
    /// 将当前值现在 `[min, max]` 的范围之内。
    ///
    /// # Examples
    ///
    /// ```
    /// use pavo_traits::{Clamped};
    ///
    /// assert!(8.clamped(6, 7) == 7);
    /// assert!(8.clamped(7, 8) == 8);
    /// assert!(8.clamped(8, 9) == 8);
    /// assert!(8.clamped(9, 10) == 9);
    /// ```
    fn clamped(self, min: Self, max: Self) -> Self;
}

macro_rules! impl_clamped {
    ($Type:ty) => {
        impl Clamped for $Type {
            fn clamped(self, min: Self, max: Self) -> Self {
                if self < min {
                    return min;
                }
                if self > max {
                    return max;
                }
                self
            }
        }
    };
}

impl_clamped!(i8);
impl_clamped!(i16);
impl_clamped!(i32);
impl_clamped!(i64);
impl_clamped!(isize);
impl_clamped!(u8);
impl_clamped!(u16);
impl_clamped!(u32);
impl_clamped!(u64);
impl_clamped!(usize);
impl_clamped!(f32);
impl_clamped!(f64);

/// 定义判断值是否相近的契定。
pub trait IsApproach {
    /// 当值处于 `+/- factor` 的范围内时返回 `true`。
    ///
    /// # Examples
    ///
    /// ```
    /// use pavo_traits::{IsApproach};
    ///
    /// assert!(!8.is_approach(10, 0.1));
    /// assert!(!8.is_approach(10, 0.15));
    /// assert!(8.is_approach(10, 0.20));
    /// assert!(9.is_approach(10, 0.1));
    /// assert!(!80.is_approach(100, 0.1));
    /// assert!(90.is_approach(100, 0.1));
    /// ```
    fn is_approach(&self, target: Self, factor: f32) -> bool;
}

macro_rules! impl_is_approach {
    ($Type:ty) => {
        impl IsApproach for $Type {
            fn is_approach(&self, target: Self, factor: f32) -> bool {
                let diff = (target as f64 * factor as f64).ceil() as Self;
                *self < target.saturating_add(diff) && *self > target.saturating_sub(diff)
            }
        }
    };
}

impl_is_approach!(i8);
impl_is_approach!(i16);
impl_is_approach!(i32);
impl_is_approach!(i64);
impl_is_approach!(isize);
impl_is_approach!(u8);
impl_is_approach!(u16);
impl_is_approach!(u32);
impl_is_approach!(u64);
impl_is_approach!(usize);

/// 定义判断值是否在范围内的契定。
pub trait IsInRange {
    /// 当值处于 `[min, max]` 的范围内时返回 `true`。
    ///
    /// # Examples
    ///
    /// ```
    /// use pavo_traits::{IsInRange};
    ///
    /// assert!(!8.is_in_range(6, 7));
    /// assert!(8.is_in_range(7, 8));
    /// assert!(8.is_in_range(8, 9));
    /// assert!(!8.is_in_range(9, 10));
    /// ```
    fn is_in_range(self, min: Self, max: Self) -> bool;
}

macro_rules! impl_is_in_range {
    ($Type:ty) => {
        impl IsInRange for $Type {
            fn is_in_range(self, min: Self, max: Self) -> bool {
                self >= min && self <= max
            }
        }
    };
}

impl_is_in_range!(i8);
impl_is_in_range!(i16);
impl_is_in_range!(i32);
impl_is_in_range!(i64);
impl_is_in_range!(isize);
impl_is_in_range!(u8);
impl_is_in_range!(u16);
impl_is_in_range!(u32);
impl_is_in_range!(u64);
impl_is_in_range!(usize);
impl_is_in_range!(f32);
impl_is_in_range!(f64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_approach() {
        assert_eq!(1000u32.is_approach(1000, 0.1), true);
        assert_eq!(900u32.is_approach(1000, 0.099), false);
        assert_eq!(900u32.is_approach(1000, 0.1), true);
        assert_eq!(900u32.is_approach(1000, 0.2), true);
        assert_eq!(800u32.is_approach(1000, 0.2), true);
        assert_eq!(700u32.is_approach(1000, 0.2), false);
        assert_eq!(600u32.is_approach(1000, 0.2), false);
        assert_eq!(0u32.is_approach(1000, 0.2), false);
        let a = std::u32::MAX - std::u32::MAX / 1000 * 99;
        assert_eq!(a.is_approach(std::u32::MAX, 0.2), true);
    }

    #[test]
    fn test_is_in_range() {
        for a in 0..1000000 {
            assert!(!a.is_in_range(a - 2, a - 1));
            assert!(a.is_in_range(a - 1, a));
            assert!(a.is_in_range(a, a));
            assert!(a.is_in_range(a, a + 1));
        }
    }
}
