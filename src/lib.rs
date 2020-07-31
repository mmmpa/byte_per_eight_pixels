#![feature(stmt_expr_attributes)]

mod eight_px_eight_bit;
mod error;

pub use crate::eight_px_eight_bit::*;
pub use error::*;

pub type EightPxU8Result<T> = Result<T, EightPxU8Error>;
