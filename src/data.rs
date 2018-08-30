use nalgebra::{DMatrix, DVector, MatrixArray, Vector2, Vector3};
use std::any::Any;
use std::collections::BTreeMap;
use std::hash::{Hash, Hasher};

/// The name of an attribute.
///
/// In some cases attributes don't have a string key attached to them,
/// but are refered to by a numeric index or assigned one according to
/// their position in a list of attributes.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum AttrName {
    Index(usize),
    Key(String),
}

impl From<usize> for AttrName {
    fn from(i: usize) -> Self {
        AttrName::Index(i)
    }
}

impl From<String> for AttrName {
    fn from(k: String) -> Self {
        AttrName::Key(k)
    }
}

/// Attributes of nodes or elements.
#[derive(Clone, Debug)]
pub struct Attr {
    values: BTreeMap<AttrName, f64>,
}

impl Attr {
    /// Create a new and empty attr instance.
    pub fn new() -> Self {
        Attr {
            values: BTreeMap::new(),
        }
    }

    pub fn get<N: Into<AttrName>>(&self, name: N) -> Option<f64> {
        self.values.get(&name.into()).cloned()
    }

    pub fn insert<N: Into<AttrName>>(&mut self, name: N, value: f64) {
        self.values.insert(name.into(), value);
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    // TODO: implement all nescessary methods.
}

#[derive(Clone, Debug)]
pub struct Group {
    /// A ID which is unique for each distinct group
    /// while parsing.
    parsing_uid: u64,
    name: String,
    attr: Attr,
    size: Option<usize>,
    kind: GroupKind,
}

impl PartialEq for Group {
    fn eq(&self, other: &Group) -> bool {
        self.parsing_uid == other.parsing_uid
    }
}
impl Eq for Group {}

impl Hash for Group {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.parsing_uid.hash(state);
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum GroupKind {
    Node,
    Element,
}

impl Group {
    pub fn new<N: Into<String>>(
        parsing_uid: u64,
        name: N,
        size: Option<usize>,
        kind: GroupKind,
    ) -> Self {
        Group {
            parsing_uid,
            name: name.into(),
            attr: Attr::new(),
            size,
            kind,
        }
    }

    // TODO: add convenience functions later when rest of api stabilized
    /*
    pub fn nodes<N: Into<String>>(parsing_uid: u64,
                name: N,
                size: Option<usize>) -> Group
    {

        Group {
            parsing_uid,
            name: name.into(),
            attr: Attr::new(),
            size,
            kind: GroupKind::Node
        }
    }

    pub fn elements<N: Into<String>>(parsing_uid: u64,
                                     name: N,
                                     size: Option<usize>) -> Group
    */

    pub fn attr(&self) -> &Attr {
        &self.attr
    }

    pub fn size(&self) -> Option<usize> {
        self.size
    }

    pub fn kind(&self) -> GroupKind {
        self.kind
    }
}

// TODO: move to correct place
/// Face-vertex mesh representation.
pub mod face_vertex {
    use super::*;
    use de::*;
    use ser::*;

    #[derive(Clone, Debug)]
    pub struct Node {
        position: DVector<f64>,
        attr: Attr,
    }

    /// A vec of elements of a specific type.
    /// However this will only contain the bare information, further
    /// casting might be desirable.
    #[derive(Debug)]
    pub struct ElementGroup {
        group: Group,
        pub(crate) elements: Vec<Element>,
    }

    #[derive(Debug)]
    pub struct NodeGroup {
        group: Group,
        pub(crate) nodes: Vec<Node>,
    }

    #[derive(Debug)]
    pub struct Element {
        attr: Attr,
        indices: DVector<usize>,
    }

    #[derive(Default, Debug)]
    pub struct Mesh {
        dimension: u8,

        // TODO: Replace with compile time sized `Vector<..>`.
        nodes: Vec<NodeGroup>,
        elements: Vec<ElementGroup>,
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

        fn item_at(&self, index: usize) -> Option<Self::Item>
        {
            self.nodes.get(index)
        }
    }

    pub struct NodeGroupsIterator<'m> {
        index: usize,
        mesh: &'m Mesh
    }

    impl<'m> Iterator for NodeGroupsIterator<'m> {
        type Item = &'m NodeGroup;

        fn next(&mut self) -> Option<<Self as Iterator>::Item> {
            let opt = self.mesh.nodes.get(self.index);
            self.index += 1;
            opt
        }
    }

    impl<'m> SerializableMesh for &'m Mesh {
        type NodeGroup = &'m NodeGroup;
        type NodeGroups = NodeGroupsIterator<'m>;

        fn metadata(&self) -> MeshMetadata {
            MeshMetadata {
                dimension: self.dimension,
            }
        }

        fn node_groups(&self) -> Self::NodeGroups {
            NodeGroupsIterator {
                index: 0,
                mesh: self
            }
        }

        /*
        fn element_groups(&self) -> ElementSerializer {
            ElementSerializer { mesh: self }
        }
        */
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
                        None => Vec::new()
                    }
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
                    attr: attr,
                    position: position
                });
            } else {
                // TODO error!
            }
            Ok(())
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
            } else {
                // TODO error!
            }
            Ok(())
        }
    }
}
