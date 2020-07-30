#![feature(stmt_expr_attributes)]

mod byte_per_eight_pixels;
mod error;

pub use crate::byte_per_eight_pixels::*;
pub use error::*;

pub type BytePerEightPixelsResult<T> = Result<T, BytePerEightPixelsError>;
