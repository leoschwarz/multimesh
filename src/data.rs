use nalgebra::{DVector, DMatrix, MatrixArray, Vector2, Vector3};
use std::collections::BTreeMap;
use std::any::Any;

/*
// TODO: extract to crate (if the other todo can be fixed)
mod storage {
    use std::collections::BTreeMap;
    use std::any::Any;
    use std::cell::Cell;
    use std::rc::Rc;
    use std::mem;

    // TODO: cannot be made generic, right?
    use ::element::Element as Trait;

    pub struct DynVecMap<Key> {
        data: BTreeMap<Key, Box<dyn Any + 'static>>,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
    pub enum GetError {
        Missing,
        WrongType
    }

    impl<Key> DynVecMap<Key> where Key: Ord {
        pub fn insert<V: Trait + 'static>(&mut self, key: Key, value: Vec<V>) {
            self.data.insert(key, Box::new(value));
        }

        pub fn remove<V: Trait + 'static>(&mut self, key: &Key)
            -> Result<Vec<V>, GetError>
        {
            let box_any = self.data.remove(key).ok_or(GetError::Missing)?;
            box_any.downcast_mut::<Vec<V>>()
                .map(|vecref| mem::replace(vecref, Vec::new()))
                .ok_or(GetError::WrongType)
        }

        pub fn contains_key(&self, key: &Key) -> bool {
            self.data.contains_key(key)
        }

        /*
        pub fn contains_key_typed<V: Trait>(&self, key: &Key) -> bool {
            if let Some(ref any) = self.data.get(key) {
                any.is::<Cell<V>>()
            } else {
                false
            }
        }
        */
    }

}
*/

/// The name of an attribute.
///
/// In some cases attributes don't have a string key attached to them,
/// but are refered to by a numeric index or assigned one according to
/// their position in a list of attributes.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum AttrName {
    Index(usize),
    Key(String)
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
            values: BTreeMap::new()
        }
    }

    pub fn insert<N: Into<AttrName>>(&mut self, name: N, value: f64) {
        self.values.insert(name.into(), value);
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    // TODO: implement all nescessary methods.
}

/*
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct ElementKind {
    name: String,
}

pub struct Element {
    nodes: DVector<usize>,
    kind: ElementKind,
    attr: Attr,
}
*/

// TODO / OPTIONS:
// 1) 

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

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum GroupKind {
    Node,
    Element
}

impl Group {
    pub fn new<N: Into<String>>(
        parsing_uid: u64,
        name: N,
        size: Option<usize>,
        kind: GroupKind) -> Self {
        Group {
            parsing_uid,
            name: name.into(),
            attr: Attr::new(),
            size,
            kind
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

pub trait DeserializeElement {
    fn indices(&mut self) -> Option<DVector<usize>>;
    fn attr(&mut self) -> Attr;
}

impl DeserializeElement for (DVector<usize>, Attr) {
    fn indices(&mut self) -> Option<DVector<usize>>
    {
        // TODO: this is bad
        Some(self.0.clone())
    }

    fn attr(&mut self) -> Attr {
        // TODO: this could be even worse
        self.1.clone()
    }
}

/// Types which are able to receive a deserialized mesh.
pub trait DeserializeMesh {
    fn de_group_begin(&mut self, group: &Group) {}
    fn de_group_end(&mut self, group: &Group) {}

    /// Deserialize a node at a position and with attributes.
    ///
    /// TODO: DVector should be const generic size (when supported)
    fn de_node(&mut self, position: DVector<f64>, attr: Attr, group: &Group);
    fn de_element<De: DeserializeElement>(
        &mut self, element: De, group: &Group);

    /*
    fn de_node(&mut self, &, position: DVector<f64>, attr: Attr);
    fn de_element_indices<It>(&mut self, indices_it: It) where
        It: Iterator<Item=(DVector<usize>, Attr)>;

    fn reserve_nodes(&mut self, _num_nodes: usize, _dim: usize, _num_attr: usize) {}
    fn reserve_elements(&mut self, _name: String, _num_elements: usize) {}
    */
}

// TODO: move to correct place
/// Face-vertex mesh representation.
pub mod face_vertex {
    use super::*;

    pub struct Node {
        position: DVector<f64>,
        attr: Attr,
    }

    /// A vec of elements of a specific type.
    /// However this will only contain the bare information, further
    /// casting might be desirable.
    pub struct ElementGroup {
        name: String,
        elements: Vec<Element>,
        group: Group,
    }

    pub struct Element {
        attr: Attr,
        indices: DVector<usize>
    }

    #[derive(Default)]
    pub struct Mesh {
        // TODO: Replace with compile time sized `Vector<..>`.
        nodes: Vec<Node>,
        nodes_attr: Vec<Attr>,
        elements: Vec<ElementGroup>,
    }

    impl<'a> DeserializeMesh for &'a mut Mesh {
        fn de_group_begin(&mut self, group: &Group) {
            if group.kind() == GroupKind::Element {

            }
        }

        fn de_group_end(&mut self, group: &Group) {}

        fn de_node(&mut self, position: DVector<f64>, attr: Attr, group: &Group)
        {
            // TODO groups!
            self.nodes.push(Node {position, attr});
        }

        fn de_element<De: DeserializeElement>(
            &mut self, element: De, group: &Group)
        {

        }

        /*
        fn de_node(&mut self, position: DVector<f64>, attr: Attr) {
            self.nodes.push(Node {position, attr});
        }

        fn de_element_indices<It>(&mut self, indices_it: It) where
            It: Iterator<Item=(DVector<usize>, Attr)>
        {
            let mut el_vec = ElementGroup {
                // TODO get the name
                name: "".into(),
                elements: Vec::new()
            };
            for (indices, attr) in indices_it {
                el_vec.elements.push(Element { attr, indices });
            }
            self.elements.push(el_vec);
        }

        fn reserve_nodes(&mut self, num_nodes: usize, dim: usize, num_attr: usize) {
            self.nodes.reserve_exact(num_nodes);
            self.nodes_attr.reserve_exact(num_nodes);
        }

        fn reserve_elements(&mut self, name: String, num: usize) {
            // TODO
            //self.elements.reserve_exact(num);
        }
        */
    }
}
