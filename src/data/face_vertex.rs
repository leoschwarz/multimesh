//! Face-vertex mesh representation.
// TODO(blocked): Const-generics for node and element items?

use data::{
    attribute::{AttributeMap, AttributeName},
    Entity, EntityBox, EntityKind, GetMesh, GetMeshGroup, GroupMetadata, MeshMetadata, SetMesh,
    SetMeshGroup, *,
};
use error::Error;
use nalgebra::DVector;
use naming::Name;
use std::{borrow::Cow, fmt};

/// A mesh represented in face-vertex form, referred to as elements and nodes in the following.
///
/// This data structure is not optimized for efficiency, but intended as an easy to use
/// data structure for serialization and deserialization results.
#[derive(Default, Debug)]
pub struct Mesh {
    dimension: u8,

    nodes: Vec<EntityGroup>,
    elements: Vec<EntityGroup>,
    vectors: Vec<EntityGroup>,
    others: Vec<EntityGroup>,
}

impl<'m> SetMesh<'m> for &'m mut Mesh {
    type GroupSetter = MeshGroupSetter<'m>;

    fn set_dimension(&'m mut self, dim: u8) {
        self.dimension = dim;
    }

    fn add_group(&'m mut self, name: Name, kind: EntityKind) -> Result<Self::GroupSetter, Error> {
        Ok(MeshGroupSetter {
            name,
            kind,
            mesh: self,
            entities: Vec::new(),
        })
    }
}

pub struct MeshGroupSetter<'m> {
    name: Name,
    kind: EntityKind,
    mesh: &'m mut Mesh,
    entities: Vec<EntityBox>,
}

impl<'m> SetMeshGroup<'m> for MeshGroupSetter<'m> {
    fn add_entity<E: Entity>(&mut self, entity: E) -> Result<(), Error> {
        self.entities.push(EntityBox::from_entity(&entity));
        Ok(())
    }

    fn end(self) -> Result<(), Error> {
        let group = EntityGroup {
            name: self.name,
            kind: self.kind,
            entities: self.entities,
        };
        match self.kind {
            EntityKind::Node => self.mesh.nodes.push(group),
            EntityKind::Element => self.mesh.elements.push(group),
            EntityKind::Vector => self.mesh.vectors.push(group),
            EntityKind::Other => self.mesh.others.push(group),
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct EntityGroup {
    name: Name,
    kind: EntityKind,
    entities: Vec<EntityBox>,
}

impl<'m> GetMesh<'m> for &'m Mesh {
    type Entity = EntityBox;
    type GroupReader = MeshGroupReader<'m>;
    type GroupReaders = Box<Iterator<Item = Self::GroupReader> + 'm>;

    fn metadata(&self) -> MeshMetadata {
        MeshMetadata {
            dimension: self.dimension,
        }
    }

    fn groups(&self) -> Self::GroupReaders {
        /*
        let groups_it = self.nodes.iter().chain(self.elements.iter()).chain(self.vectors.iter()).chain(self.others.iter());
        Box::new(groups_it.map(|group| MeshGroupReader {mesh: self, entity_group: group, index: 0}))
        */
        unimplemented!()
    }
}

pub struct MeshGroupReader<'m> {
    mesh: &'m Mesh,
    entity_group: &'m EntityGroup,
    index: usize,
}

impl<'m> GetMeshGroup for MeshGroupReader<'m> {
    fn metadata(&self) -> GroupMetadata {
        unimplemented!()
    }
}

impl<'m> Iterator for MeshGroupReader<'m> {
    type Item = EntityBox;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        unimplemented!()
    }
}
