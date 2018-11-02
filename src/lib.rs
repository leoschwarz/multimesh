#![allow(dead_code)]
#![allow(unused_imports)]

//! # Overview
//! The goal of this crate is to provide a generic interface for serializing and
//! deserializing geometric meshes into various formats.
//!
//! ## Status
//! The API is still in draft state, since it's necessary to find abstractions
//! which are suitable for a wide range of formats while still being efficient and
//! ergonomic for real world and big data set usage.

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
