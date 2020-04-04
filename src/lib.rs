//! 用于 `Pavo` 框架的通用契定库。
//!
pub mod wrap;

#[doc(inline)]
pub use wrap::*;

pub mod convert;

#[doc(inline)]
pub use convert::*;

#[cfg(test)]
mod tests {}
