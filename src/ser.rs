//! Defines the serialization interface for mesh container types.

use data::mesh::{ReadElement, ReadEntity, ReadNode, ReadVector};
use naming::Name;
use std::io::Write;

pub trait Serializer {
    type Result;

    fn serialize<M, W>(&self, mesh: M, target: W) -> Self::Result
    where
        M: SerializableMesh,
        W: Write;
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

pub trait SerializableGroup {
    type Item;

    fn metadata(&self) -> GroupMetadata;

    fn len(&self) -> usize {
        self.metadata().size
    }

    fn item_at(&self, index: usize) -> Option<Self::Item>;
}

pub trait SerializableMesh {
    // TODO: This is ugly, but GAT are not implemented yet.
    // Reference: https://github.com/rust-lang/rust/issues/44265
    type Node: ReadNode;
    type NodeGroup: SerializableGroup<Item = Self::Node>;
    type NodeGroups: Iterator<Item = Self::NodeGroup>;
    type Element: ReadElement;
    type ElementGroup: SerializableGroup<Item = Self::Element>;
    type ElementGroups: Iterator<Item = Self::ElementGroup>;
    type Vector: ReadVector;
    type VectorGroup: SerializableGroup<Item = Self::Vector>;
    type VectorGroups: Iterator<Item = Self::VectorGroup>;
    type Other: ReadEntity;
    type OtherGroup: SerializableGroup<Item = Self::Other>;
    type OtherGroups: Iterator<Item = Self::OtherGroup>;

    fn metadata(&self) -> MeshMetadata;

    fn node_groups(&self) -> Self::NodeGroups;
    fn element_groups(&self) -> Self::ElementGroups;
    fn vector_groups(&self) -> Self::VectorGroups;
    fn other_groups(&self) -> Self::OtherGroups;
}
