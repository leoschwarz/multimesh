//! Face-vertex mesh representation.
// TODO(blocked): Const-generics for node and element items?

use data::attribute::Attr;
use data::mesh::ReadElement;
use data::mesh::ReadEntity;
use data::mesh::ReadNode;
use data::mesh::ReadVector;
use data::*;
use de::*;
use nalgebra::DVector;
use ser::*;
use std::borrow::Cow;

/// A mesh represented in face-vertex form, referred to as elements and nodes in the following.
///
/// This data structure is not optimized for efficiency, but intended as an easy to use
/// data structure for serialization and deserialization results.
#[derive(Default, Debug)]
pub struct Mesh {
    dimension: u8,

    nodes: Vec<Group<Node>>,
    elements: Vec<Group<Element>>,
    vectors: Vec<Group<Vector>>,
    others: Vec<Group<Entity>>,
}

#[derive(Clone, Debug)]
pub struct Entity {
    attr: Attr,
}

#[derive(Clone, Debug)]
pub struct Node {
    position: DVector<f64>,
    attr: Attr,
}

#[derive(Clone, Debug)]
pub struct Element {
    indices: DVector<usize>,
    attr: Attr,
}

#[derive(Clone, Debug)]
pub struct Vector {
    components: DVector<f64>,
    attr: Attr,
}

macro_rules! impl_read_entity {
    ($target:ident) => {
        impl<'m> ReadEntity for &'m $target {
            type Error = ();

            fn attr_at(&self, index: usize) -> Result<Option<(Cow<AttrName>, Cow<str>)>, ()> {
                Ok(self
                    .attr
                    .get_at(index)
                    .map(|(n, v)| (Cow::Borrowed(n), Cow::Borrowed(v.as_ref()))))
            }

            fn attr_get(&self, name: &AttrName) -> Result<Option<Cow<str>>, ()> {
                Ok(self.attr.get(name).map(|s| s.into()))
            }
        }
    };
}

impl_read_entity!(Entity);
impl_read_entity!(Node);
impl_read_entity!(Element);
impl_read_entity!(Vector);

impl<'m> ReadNode for &'m Node {
    fn position(&self) -> Result<Cow<DVector<f64>>, ()> {
        Ok(Cow::Borrowed(&self.position))
    }
}

impl<'m> ReadElement for &'m Element {
    fn node_indices(&self) -> Result<Option<Cow<DVector<usize>>>, ()> {
        Ok(Some(Cow::Borrowed(&self.indices)))
    }
}

impl<'m> ReadVector for &'m Vector {
    fn components(&self) -> Result<Cow<DVector<f64>>, ()> {
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
) -> Result<(), DeserializerError> {
    if let Some(ref mut group) = target.last_mut() {
        if *group_data == group.data {
            group.items.push(entity);
            return Ok(());
        }
    }

    Err(DeserializerError::BrokenInvariant(
        "de_group_begin was not invoked".into(),
    ))
}

impl<'a> DeserializeMesh for &'a mut Mesh {
    fn de_dimension(&mut self, dim: u8) {
        self.dimension = dim;
    }

    fn de_group_begin(&mut self, group_data: &GroupData) -> Result<(), DeserializerError> {
        match group_data.kind() {
            GroupKind::Node => self.nodes.push(Group::new_empty(group_data.clone())),
            GroupKind::Element => self.elements.push(Group::new_empty(group_data.clone())),
            GroupKind::Vector => self.vectors.push(Group::new_empty(group_data.clone())),
            GroupKind::Other => self.others.push(Group::new_empty(group_data.clone())),
        }

        Ok(())
    }

    fn de_group_end(&mut self, _group: &GroupData) -> Result<(), DeserializerError> {
        Ok(())
    }

    fn de_entity<R>(&mut self, entity: &R, group_data: &GroupData) -> Result<(), DeserializerError>
    where
        R: ReadEntity<Error = DeserializerError>,
    {
        let attr = Attr::from_entity(entity)?;
        impl_de_entity(Entity { attr }, &mut self.others, group_data)
    }

    fn de_node<R>(&mut self, node: &R, group_data: &GroupData) -> Result<(), DeserializerError>
    where
        R: ReadNode<Error = DeserializerError>,
    {
        let attr = Attr::from_entity(node)?;
        let position = node.position()?.into_owned();
        impl_de_entity(Node { attr, position }, &mut self.nodes, group_data)
    }

    // TODO
    /*
    fn de_element<De: DeserializeElement>(&mut self, element: De, group: &GroupData) -> Result<(), DeserializerError> {
        unimplemented!()
    }

    fn de_vector<De: DeserializeVector>(&mut self, vector: De, group: &GroupData) -> Result<(), DeserializerError> {
        unimplemented!()
    }
    */
}
