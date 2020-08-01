#![feature(stmt_expr_attributes)]

mod common;
mod eight_px_uint_eight;
mod error;
mod vertical_eight_px_uint_eight;

pub use crate::eight_px_uint_eight::*;
pub use common::*;
pub use error::*;
pub use vertical_eight_px_uint_eight::*;

pub type EightPxUintEightResult<T> = Result<T, EightPxUintEightError>;
