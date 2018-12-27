//! API for mesh data containers.
//!
//! This provides an abstraction over different mesh representation types,
//! making it possible to use multimesh with any Rust representation of meshes
//! you want, as long as they implement the necessary traits.

use naming::Name;
use std::hash::{Hash, Hasher};

pub mod attribute;
//pub mod printing;

pub mod entity;
pub mod face_vertex;
pub mod mesh;

pub use self::{
    entity::{Entity, EntityBox, EntityKind},
    mesh::{GetMesh, GetMeshGroup, GroupMetadata, MeshMetadata, SetMesh, SetMeshGroup},
};

use data::attribute::{AttributeContainer, AttributeMap};
use error::Error;
use std::{borrow::Cow, fmt::Debug};

/*
#[derive(Clone, Debug)]
pub struct GroupData {
    name: Name,
    attr: AttributeContainer,
    size: Option<usize>,
    kind: EntityKind,
}
*/
