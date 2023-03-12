// #![feature(generic_const_exprs)]
#![cfg_attr(not(feature = "std"), no_std)]

pub mod ecma335;
pub mod pe;

extern crate alloc;
#[cfg(feature = "std")]
extern crate core;
