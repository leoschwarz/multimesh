#![allow(dead_code)]
#![allow(unused_imports)]

extern crate nalgebra;
extern crate regex;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate failure;

mod util;

/// Deserialization.
pub mod de;

/// Serialization.
pub mod ser;

pub mod data;
pub mod format;

pub mod naming;

pub mod error;
