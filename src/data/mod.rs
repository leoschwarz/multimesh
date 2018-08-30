use naming::Name;
use std::collections::BTreeMap;
use std::hash::{Hash, Hasher};

pub mod face_vertex;

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
    name: Name,
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
    pub fn new(parsing_uid: u64, name: Name, size: Option<usize>, kind: GroupKind) -> Self {
        Group {
            parsing_uid,
            name: name,
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
