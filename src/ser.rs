//! Defines the serialization interface for mesh container types.

use nalgebra::DVector;
use data::{Attr, Group};

/*
// TODO: How should the apis handle presence of data which is not supported by a specific format?!
// it should return a warning result?
//
// i.e. should it use a type like:
// (problem: carrier would only propagate err(e), but how would you propagate warn?
//  → this could be done with a local variable and a custo macro)
enum SerializerResult<T, W, E> {
    Ok(T),
    Warn(T, W),
    Err(E),
}

pub trait NodeSerializer {
    fn position(&mut self, p: DVector<f64>);
    fn attr(&mut self, a: Attr);
}

pub trait ElementSerializer {
    fn node_indices(&mut self, i: DVector<usize>);
    fn node_positions(&mut self, i: DVector<usize>);
    fn attr(&mut self, a: Attr);
}

// TODO: should groups be handled by this trait or the node/element serializers?
pub trait SerializeMesh {
    fn ser_dimension(&self) -> u32;
    fn ser_nodes<S: NodeSerializer>(&self, ser: S, group: &Group);
    fn ser_elements<S: ElementSerializer>(&self, ser: S, group: &Group);
}
*/

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
    pub(crate) name: String,
    pub(crate) size: usize,
}

impl GroupMetadata {
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

pub trait SerializableGroup {
    fn metadata(&self) -> GroupMetadata;

    fn len(&self) -> usize {
        self.metadata().size
    }
}

pub trait SerializableNodeGroup: SerializableGroup {
    type Item: SerializableNode;

    fn item_at(&self, index: usize) -> Option<Self::Item>;
}

pub trait SerializableElementGroup: SerializableGroup {
    type Item: SerializableElement;

    fn item_at(&self, index: usize) -> Option<Self::Item>;
}

/*
pub trait SerializableGroup {
    type Error;
    type Item;
    type Items: Iterator<Item>;

    fn metadata(&self) -> GroupMetadata;
    fn items(&self) -> Self::Items;

    fn write_at<S: Serializer>(&self, index: usize, target: &mut S) -> Result<(), Self::Error>;

    fn write_all<S: Serializer>(&self, target: &mut S) -> Result<(), Self::Error>
    {
        // TODO: if further fields are added to metadata, this should be revised
        let size = self.metadata().size();
        for i in 0..size {
            self.write_at(i, target)?;
        }
        Ok(())
    }
}
*/

pub trait SerializableMesh {
    type NodeGroup: SerializableNodeGroup;
    type NodeGroups: Iterator<Item=Self::NodeGroup>;

    /*
    type ElementGroup: SerializableElementGroup;
    type ElementGroups: Iterator<Item=Self::ElementGroup>;
    */

    fn metadata(&self) -> MeshMetadata;

    fn node_groups(&self) -> Self::NodeGroups;
    //fn element_groups(&self) -> Self::ElementGroups;
}

pub trait Serializer {
    fn serialize<M>(&self, mesh: M)
    where M: SerializableMesh;
    /*
    fn serialize<M, NG, EG>(&self, mesh: M)
    where M: SerializableMesh,
          M::SerializableNodeGroups: Iterator<Item=NG>,
          M::SerializableElementGroups: Iterator<Item=EG>,
          NG: SerializableNodeGroup,
          EG: SerializableElementGroup;
    */
}

/*
pub trait Serializer {
    fn serialize<M>(&self, mesh: M)
    where M: SerializableMesh<NodeGroupsSerializer=Iterator<Item=SerializableGroup<Item=Node>>>
}
*/

