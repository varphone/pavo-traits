//! 用于 `Pavo` 框架的通用契定库。
//!

pub mod convert;
#[doc(inline)]
pub use convert::*;

pub mod num;
#[doc(inline)]
pub use num::*;

pub mod wrap;
#[doc(inline)]
pub use wrap::*;

#[cfg(test)]
mod tests {}
