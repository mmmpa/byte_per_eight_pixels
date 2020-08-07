#![cfg_attr(not(feature = "std"), no_std)]

mod common;
mod eight_data;
mod eight_px_uint_eight;
mod error;
mod horizontal_eight_px_uint_eight;
mod vertical_eight_px_uint_eight;

#[cfg(feature = "std")]
pub mod unix;

pub use crate::eight_px_uint_eight::*;
pub use common::*;
pub use eight_data::*;
pub use error::*;
pub use horizontal_eight_px_uint_eight::*;
pub use vertical_eight_px_uint_eight::*;

pub type EightPxUintEightResult<T> = Result<T, EightPxUintEightError>;
