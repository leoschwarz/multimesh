//! Face-vertex mesh representation.
// TODO(blocked): Const-generics for node and element items?

use data::*;
use de::*;
use ser::*;

/// A mesh represented in face-vertex form, referred to as elements and nodes in the following.
///
/// This data structure is not optimized for efficiency, but intended as an easy to use
/// data structure for serialization and deserialization results.
#[derive(Default, Debug)]
pub struct Mesh {
    dimension: u8,
    nodes: Vec<NodeGroup>,
    elements: Vec<ElementGroup>,
}

#[derive(Clone, Debug)]
pub struct Node {
    position: DVector<f64>,
    attr: Attr,
}

#[derive(Clone, Debug)]
pub struct Element {
    attr: Attr,
    indices: DVector<usize>,
}

#[derive(Debug)]
pub struct NodeGroup {
    group: Group,
    pub(crate) nodes: Vec<Node>,
}

#[derive(Debug)]
pub struct ElementGroup {
    group: Group,
    pub(crate) elements: Vec<Element>,
}

impl<'m> SerializableNode for &'m Node {
    fn position(&self) -> &DVector<f64> {
        &self.position
    }

    fn attr(&self) -> &Attr {
        &self.attr
    }
}

impl<'m> SerializableGroup for &'m NodeGroup {
    fn metadata(&self) -> GroupMetadata {
        GroupMetadata {
            name: self.group.name.clone(),
            size: self.nodes.len(),
        }
    }

    fn len(&self) -> usize {
        self.nodes.len()
    }
}

impl<'m> SerializableNodeGroup for &'m NodeGroup {
    type Item = &'m Node;

    fn item_at(&self, index: usize) -> Option<Self::Item> {
        self.nodes.get(index)
    }
}

pub struct NodeGroupsIterator<'m> {
    index: usize,
    mesh: &'m Mesh,
}

impl<'m> Iterator for NodeGroupsIterator<'m> {
    type Item = &'m NodeGroup;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        let opt = self.mesh.nodes.get(self.index);
        self.index += 1;
        opt
    }
}

impl<'m> SerializableGroup for &'m ElementGroup {
    fn metadata(&self) -> GroupMetadata {
        GroupMetadata {
            name: self.group.name.clone(),
            size: self.elements.len(),
        }
    }

    fn len(&self) -> usize {
        self.elements.len()
    }
}

impl<'m> SerializableElement for &'m Element {
    fn node_indices(&self) -> Option<&DVector<usize>> {
        Some(&self.indices)
    }

    fn attr(&self) -> &Attr {
        &self.attr
    }
}

impl<'m> SerializableElementGroup for &'m ElementGroup {
    type Item = &'m Element;

    fn item_at(&self, index: usize) -> Option<<Self as SerializableElementGroup>::Item> {
        self.elements.get(index)
    }
}

pub struct ElementGroupsIterator<'m> {
    index: usize,
    mesh: &'m Mesh,
}

impl<'m> Iterator for ElementGroupsIterator<'m> {
    type Item = &'m ElementGroup;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        let opt = self.mesh.elements.get(self.index);
        self.index += 1;
        opt
    }
}

impl<'m> SerializableMesh for &'m Mesh {
    type NodeGroup = &'m NodeGroup;
    type NodeGroups = NodeGroupsIterator<'m>;
    type ElementGroup = &'m ElementGroup;
    type ElementGroups = ElementGroupsIterator<'m>;

    fn metadata(&self) -> MeshMetadata {
        MeshMetadata {
            dimension: self.dimension,
        }
    }

    fn node_groups(&self) -> Self::NodeGroups {
        NodeGroupsIterator {
            index: 0,
            mesh: self,
        }
    }

    fn element_groups(&self) -> Self::ElementGroups {
        ElementGroupsIterator {
            index: 0,
            mesh: self,
        }
    }
}

impl<'a> DeserializeMesh for &'a mut Mesh {
    fn de_dimension(&mut self, dim: u8) {
        self.dimension = dim;
    }

    fn de_group_begin(&mut self, group: &Group) {
        if group.kind() == GroupKind::Element {
            self.elements.push(ElementGroup {
                group: group.clone(),
                elements: match group.size() {
                    Some(size) => Vec::with_capacity(size),
                    None => Vec::new(),
                },
            });
        } else if group.kind() == GroupKind::Node {
            self.nodes.push(NodeGroup {
                group: group.clone(),
                nodes: match group.size() {
                    Some(size) => Vec::with_capacity(size),
                    None => Vec::new(),
                },
            });
        }
    }

    fn de_group_end(&mut self, group: &Group) {}

    fn de_node(
        &mut self,
        position: DVector<f64>,
        attr: Attr,
        group: &Group,
    ) -> Result<(), DeserializerError> {
        if let Some(ref mut no_group) = self.nodes.last_mut() {
            no_group.nodes.push(Node {
                attr, position
            });
            Ok(())
        } else {
            Err(DeserializerError::BrokenInvariant("de_group_begin was not invoked".into()))
        }
    }

    fn de_element<De: DeserializeElement>(
        &mut self,
        mut element: De,
        group: &Group,
    ) -> Result<(), DeserializerError> {
        if let Some(ref mut el_group) = self.elements.last_mut() {
            el_group.elements.push(Element {
                attr: element.attr()?,
                // TODO support non-index formats
                indices: element.indices()?.unwrap(),
            });
            Ok(())
        } else {
            Err(DeserializerError::BrokenInvariant("de_group_begin was not invoked".into()))
        }
    }
}