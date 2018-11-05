use naming::Name;
use std::hash::{Hash, Hasher};

pub mod attribute;
//pub mod printing;

pub mod face_vertex;

use error::Error;
use std::borrow::Cow;
use data::attribute::AttributeContainer;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Copy)]
pub enum EntityKind {
    /// A mesh node/vertex.
    Node,
    /// A mesh element/face/volume.
    Element,
    /// A vector.
    Vector,
    /// Any other entity which does not fit into the other categories.
    Other,
}

pub struct Entity {

}

pub trait SetMeshGroup<'m> {
    /// Will not necessarily be called by every deserializer, but if it is called, then the contract
    /// is that the size will not change anymore.
    fn reserve(&mut self, num: usize) -> Result<(), Error> { Ok(()) }

    fn add_entity(&mut self, entity: Entity) -> Result<(), Error>;

    fn end(self) -> Result<(), Error>;
}

pub trait SetMesh<'m> {
    type GroupSetter: SetMeshGroup<'m> + 'm;

    fn set_dimension(&'m mut self, dim: u8);

    fn add_group(&'m mut self, name: Name, kind: EntityKind) -> Result<Self::GroupSetter, Error>;
}

pub trait GetMeshGroup: Iterator<Item=Entity> {
    fn metadata(&self) -> GroupMetadata;
}

pub trait GetMesh<'m> {
    type GroupReader: GetMeshGroup + 'm;
    type GroupReaders: Iterator<Item=&'m Self::GroupReader> + 'm;

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