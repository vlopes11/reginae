#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
use alloc::{collections::BTreeSet, vec, vec::Vec};

#[cfg(feature = "std")]
use std::{collections::BTreeSet, vec, vec::Vec};

mod board;
pub use board::Board;

mod cell;
pub use cell::Cell;
