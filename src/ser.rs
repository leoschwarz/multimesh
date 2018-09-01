//! Defines the serialization interface for mesh container types.

use data::Attr;
use nalgebra::DVector;
use naming::Name;
use std::io::Write;

pub trait Serializer {
    type Result;

    fn serialize<M, W>(&self, mesh: M, target: W) -> Self::Result
    where
        M: SerializableMesh,
        W: Write;
}

pub trait SerializableNode {
    fn position(&self) -> &DVector<f64>;
    fn attr(&self) -> &Attr;
}

pub trait SerializableElement {
    fn node_indices(&self) -> Option<&DVector<usize>>;
    // TODO use matrix type?
    //fn node_positions(&self) -> Option<&DVector<f64>>
    fn attr(&self) -> &Attr;
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
    type Node: SerializableNode;
    type NodeGroup: SerializableGroup<Item = Self::Node>;
    type NodeGroups: Iterator<Item = Self::NodeGroup>;
    type Element: SerializableElement;
    type ElementGroup: SerializableGroup<Item = Self::Element>;
    type ElementGroups: Iterator<Item = Self::ElementGroup>;

    fn metadata(&self) -> MeshMetadata;

    fn node_groups(&self) -> Self::NodeGroups;
    fn element_groups(&self) -> Self::ElementGroups;
}
