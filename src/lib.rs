#![allow(dead_code)]
#![allow(unused_import)]

extern crate nalgebra;
extern crate regex;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate failure;

/// Deserialization.
pub mod de;

/// Serialization.
pub mod ser;

pub mod data;
pub mod format;

pub mod naming;
pub mod util;

pub mod error;
