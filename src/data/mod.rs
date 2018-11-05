use naming::Name;
use std::hash::{Hash, Hasher};

pub mod attribute;
//pub mod printing;

pub mod face_vertex;
pub mod entity;

pub use self::entity::{Entity, EntityBox, EntityKind};

use error::Error;
use std::borrow::Cow;
use data::attribute::AttributeContainer;
use std::fmt::Debug;
use data::attribute::AttributeMap;


pub trait SetMeshGroup<'m> {
    /// Will not necessarily be called by every deserializer, but if it is called, then the contract
    /// is that the size will not change anymore.
    fn reserve(&mut self, _num: usize) -> Result<(), Error> { Ok(()) }

    fn add_entity<E: Entity>(&mut self, entity: E) -> Result<(), Error>;

    fn end(self) -> Result<(), Error>;
}

pub trait SetMesh<'m> {
    //type Entity: Entity;
    //type GroupSetter: SetMeshGroup<'m, Entity=Self::Entity> + 'm;
    type GroupSetter: SetMeshGroup<'m> + 'm;

    fn set_dimension(&'m mut self, dim: u8);

    fn add_group(&'m mut self, name: Name, kind: EntityKind) -> Result<Self::GroupSetter, Error>;
}

pub trait GetMeshGroup: Iterator {
    fn metadata(&self) -> GroupMetadata;
}

pub trait GetMesh<'m> {
    type Entity;
    type GroupReader: GetMeshGroup<Item=Self::Entity> + 'm;
    type GroupReaders: Iterator<Item=Self::GroupReader> + 'm;

    fn metadata(&self) -> MeshMetadata;
    fn groups(&self) -> Self::GroupReaders;
}


#[derive(Clone, Debug)]
pub struct MeshMetadata {
    /// The dimensionality of the mesh, usually `2` or `3`.
    // TODO: builder/constructor or public
    pub(crate) dimension: u8,
}

impl MeshMetadata {
    /// The dimensionality of the mesh, usually `2` or `3`.
    pub fn dimension(&self) -> u8 {
        self.dimension
    }
}



pub struct GroupMetadata {
    // TODO: builder/constructor or public
    pub(crate) name: Name,
    pub(crate) size: usize,
}

impl GroupMetadata {
    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn len(&self) -> usize {
        self.size
    }
}

/*
#[derive(Clone, Debug)]
pub struct GroupData {
    name: Name,
    attr: AttributeContainer,
    size: Option<usize>,
    kind: EntityKind,
}
*/