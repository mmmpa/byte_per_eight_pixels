#![feature(stmt_expr_attributes)]

mod eight_px_uint_eight;
mod error;

pub use crate::eight_px_uint_eight::*;
pub use error::*;

pub type EightPxUintEightResult<T> = Result<T, EightPxUintEightError>;
