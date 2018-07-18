use nalgebra::{DVector, MatrixArray, Vector2, Vector3};
use std::collections::BTreeMap;
use std::any::Any;

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

pub struct NodeAttr {
    values: BTreeMap<AttrName, f64>,
}

impl NodeAttr {
    pub fn new() -> Self {
        NodeAttr {
            values: BTreeMap::new()
        }
    }

    pub fn insert<N: Into<AttrName>>(&mut self, name: N, value: f64) {
        self.values.insert(name.into(), value);
    }
}

pub struct ElementAttr {}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct ElementKind {
    name: String,
}

pub struct Element {
    nodes: DVector<usize>,
    kind: ElementKind,
    attr: ElementAttr,
}

pub trait DeserializeMesh {
    fn de_node(&mut self, position: DVector<f64>, attr: NodeAttr);
    fn de_element_indices(&mut self, indices: DVector<usize>, attr: ElementAttr);

    fn reserve_nodes(&mut self, _num_nodes: usize, _dim: usize, _num_attr: usize) {}
    fn reserve_elements(&mut self, _num_elements: usize);
}

// TODO: move to correct place
/// Face-vertex mesh representation.
pub mod face_vertex {
    use super::*;

    pub struct Mesh {
        // TODO: Replace with compile time sized `Vector<..>`.
        nodes: Vec<DVector<f64>>,
        nodes_attr: Vec<NodeAttr>,
        elements: Vec<Element>,
        //element_kinds: BTreeMap<>
    }

    impl DeserializeMesh for Mesh {
        fn de_node(&mut self, position: DVector<f64>, attr: NodeAttr) {
            self.nodes.push(position);
            self.nodes_attr.push(attr);
        }

        fn de_element_indices(&mut self, indices: DVector<usize>, attr: ElementAttr) {
            self.elements.push(indices);
        }

        fn reserve_nodes(&mut self, num_nodes: usize, dim: usize, num_attr: usize) {
            self.nodes.reserve_exact(num_nodes);
            self.nodes_attr.reserve_exact(num_nodes);
        }

        fn reserve_elements(&mut self, num: usize) {
            self.elements.reserve_exact(num);
        }
    }
}
