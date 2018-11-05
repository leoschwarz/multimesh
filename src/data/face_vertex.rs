//! Face-vertex mesh representation.
// TODO(blocked): Const-generics for node and element items?

use data::{
    attribute::{AttributeMap, AttributeName},
    *,
};
use error::Error;
use nalgebra::DVector;
use std::{borrow::Cow, fmt};
use data::SetMesh;
use data::EntityKind;
use naming::Name;
use data::SetMeshGroup;
use data::Entity;
use data::{GetMesh, GetMeshGroup};
use data::MeshMetadata;
use data::GroupMetadata;

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
    entities: Vec<Entity>
}

impl<'m> SetMeshGroup<'m> for MeshGroupSetter<'m> {
    fn add_entity(&mut self, entity: Entity) -> Result<(), Error> {
        self.entities.push(entity);
        Ok(())
    }

    fn end(self) -> Result<(), Error> {
        let group = EntityGroup {};
        match self.kind {
            EntityKind::Node => self.mesh.nodes.push(group),
            EntityKind::Element => self.mesh.elements.push(group),
            EntityKind::Vector => self.mesh.vectors.push(group),
            EntityKind::Other => self.mesh.others.push(group)
        }
        Ok(())
    }
}

// TODO
#[derive(Clone, Debug)]
pub struct EntityGroup {
    
}

impl<'m> GetMesh<'m> for &'m Mesh {
    type GroupReader = MeshGroupReader<'m>;
    type GroupReaders = ::std::slice::Iter<'m, MeshGroupReader<'m>>;

    fn metadata(&self) -> MeshMetadata {
        MeshMetadata {
            dimension: self.dimension,
        }
    }

    fn groups(&self) -> Self::GroupReaders {
        let groups_it = self.nodes.iter().chain(self.elements.iter()).chain(self.vectors.iter()).chain(self.others.iter());
        groups_it.map(|group| MeshGroupReader {mesh: self, entity_group: group, index: 0})
    }
}

struct MeshGroupReader<'m> {
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
    type Item = Entity;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        if index < self.entity_group.len() {
        }
    }
}

/*
#[derive(Clone, Debug)]
pub struct Entity {
    pub attr: AttributeMap,
}

#[derive(Clone, Debug)]
pub struct Node {
    pub position: DVector<f64>,
    pub attr: AttributeMap,
}

#[derive(Clone, Debug)]
pub struct Element {
    pub indices: DVector<usize>,
    pub attr: AttributeMap,
}

#[derive(Clone, Debug)]
pub struct Vector {
    pub components: DVector<f64>,
    pub attr: AttributeMap,
}

macro_rules! impl_read_entity {
    ($target:ident) => {
        impl<'m> ReadEntity for &'m $target {
            type Attributes = AttributeMap;

            fn attributes(&self) -> Cow<AttributeMap> {
                Cow::Borrowed(&self.attr)
            }
        }
    };
}

impl_read_entity!(Entity);
impl_read_entity!(Node);
impl_read_entity!(Element);
impl_read_entity!(Vector);

impl<'m> ReadNode for &'m Node {
    fn position(&self) -> Result<Cow<DVector<f64>>, Error> {
        Ok(Cow::Borrowed(&self.position))
    }
}

impl<'m> ReadElement for &'m Element {
    fn node_indices(&self) -> Result<Option<Cow<DVector<usize>>>, Error> {
        Ok(Some(Cow::Borrowed(&self.indices)))
    }
}

impl<'m> ReadVector for &'m Vector {
    fn components(&self) -> Result<Cow<DVector<f64>>, Error> {
        Ok(Cow::Borrowed(&self.components))
    }
}

#[derive(Debug)]
pub struct Group<I> {
    data: GroupData,
    pub(crate) items: Vec<I>,
}

impl<I> Group<I> {
    fn new_empty(data: GroupData) -> Self {
        let group_size = data.size();
        Group::<I> {
            data,
            items: match group_size {
                Some(size) => Vec::with_capacity(size),
                None => Vec::new(),
            },
        }
    }
}

impl<'m, T> SerializableGroup for &'m Group<T> {
    type Item = &'m T;

    fn metadata(&self) -> GroupMetadata {
        GroupMetadata {
            name: self.data.name.clone(),
            size: self.items.len(),
        }
    }

    fn len(&self) -> usize {
        self.items.len()
    }

    fn item_at(&self, index: usize) -> Option<Self::Item> {
        self.items.get(index)
    }
}

pub struct GroupIterator<'m, T: 'm> {
    index: usize,
    items: &'m Vec<Group<T>>,
}

impl<'m, T> Iterator for GroupIterator<'m, T> {
    type Item = &'m Group<T>;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        let opt = self.items.get(self.index);
        self.index += 1;
        opt
    }
}

impl Mesh {
    pub fn metadata(&self) -> MeshMetadata {
        SerializableMesh::metadata(&self)
    }
}

impl<'m> SerializableMesh for &'m Mesh {
    type Node = &'m Node;
    type NodeGroup = &'m Group<Node>;
    type NodeGroups = GroupIterator<'m, Node>;
    type Element = &'m Element;
    type ElementGroup = &'m Group<Element>;
    type ElementGroups = GroupIterator<'m, Element>;
    type Vector = &'m Vector;
    type VectorGroup = &'m Group<Vector>;
    type VectorGroups = GroupIterator<'m, Vector>;
    type Other = &'m Entity;
    type OtherGroup = &'m Group<Entity>;
    type OtherGroups = GroupIterator<'m, Entity>;

    fn metadata(&self) -> MeshMetadata {
        MeshMetadata {
            dimension: self.dimension,
        }
    }

    fn node_groups(&self) -> Self::NodeGroups {
        GroupIterator {
            index: 0,
            items: &self.nodes,
        }
    }

    fn element_groups(&self) -> Self::ElementGroups {
        GroupIterator {
            index: 0,
            items: &self.elements,
        }
    }

    fn vector_groups(&self) -> Self::VectorGroups {
        GroupIterator {
            index: 0,
            items: &self.vectors,
        }
    }

    fn other_groups(&self) -> Self::OtherGroups {
        GroupIterator {
            index: 0,
            items: &self.others,
        }
    }
}

fn impl_de_entity<E>(
    entity: E,
    target: &mut Vec<Group<E>>,
    group_data: &GroupData,
) -> Result<(), Error> {
    if let Some(ref mut group) = target.last_mut() {
        if *group_data == group.data {
            group.items.push(entity);
            return Ok(());
        }
    }

    Err(Error::BrokenInvariant(
        "de_group_begin was not invoked".into(),
    ))
}

impl<'a> DeserializeMesh for &'a mut Mesh {
    fn de_dimension(&mut self, dim: u8) {
        self.dimension = dim;
    }

    fn de_group_begin(&mut self, group_data: &GroupData) -> Result<(), Error> {
        match group_data.kind() {
            GroupKind::Node => self.nodes.push(Group::new_empty(group_data.clone())),
            GroupKind::Element => self.elements.push(Group::new_empty(group_data.clone())),
            GroupKind::Vector => self.vectors.push(Group::new_empty(group_data.clone())),
            GroupKind::Other => self.others.push(Group::new_empty(group_data.clone())),
        }

        Ok(())
    }

    fn de_group_end(&mut self, _group_data: &GroupData) -> Result<(), Error> {
        Ok(())
    }

    fn de_entity<R>(&mut self, entity: &R, group_data: &GroupData) -> Result<(), Error>
    where
        R: ReadEntity,
    {
        let attr = AttributeContainer::from(entity);
        let attr = Attr::from_entity(entity)?;
        impl_de_entity(Entity { attr }, &mut self.others, group_data)
    }

    fn de_node<R>(&mut self, node: &R, group_data: &GroupData) -> Result<(), Error>
    where
        R: ReadNode,
    {
        let attr = Attr::from_entity(node)?;
        let position = node.position()?.into_owned();
        impl_de_entity(Node { attr, position }, &mut self.nodes, group_data)
    }

    fn de_element<R>(&mut self, element: &R, group_data: &GroupData) -> Result<(), Error>
    where
        R: ReadElement,
    {
        let attr = Attr::from_entity(element)?;
        // TODO
        let indices = element
            .node_indices()?
            .ok_or_else(|| {
                Error::BrokenInvariant("Elements without node indices are not allowed yet.".into())
            })?
            .into_owned();
        impl_de_entity(Element { attr, indices }, &mut self.elements, group_data)
    }

    fn de_vector<R>(&mut self, vector: &R, group_data: &GroupData) -> Result<(), Error>
    where
        R: ReadVector,
    {
        let attr = Attr::from_entity(vector)?;
        let components = vector.components()?.into_owned();
        impl_de_entity(Vector { attr, components }, &mut self.vectors, group_data)
    }
}
*/