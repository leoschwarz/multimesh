use naming::Name;
use std::hash::{Hash, Hasher};

pub mod attribute;
pub mod face_vertex;
pub mod mesh;

pub use self::attribute::AttrName;

// TODO: This belongs somewhere else?
#[derive(Clone, Debug)]
pub struct GroupData {
    /// A ID which is unique for each distinct group while parsing.
    parsing_uid: u64,
    name: Name,
    // TODO: Should Attr be used here?
    attr: attribute::Attr,
    size: Option<usize>,
    kind: GroupKind,
}

impl PartialEq for GroupData {
    fn eq(&self, other: &GroupData) -> bool {
        self.parsing_uid == other.parsing_uid
    }
}

impl Eq for GroupData {}

impl Hash for GroupData {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.parsing_uid.hash(state);
    }
}

// TODO: Rename to EntityKind? (Problem: Entity/Property terminology is a bit problematic.)
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum GroupKind {
    /// A mesh node/vertex.
    Node,
    /// A mesh element/face/volume.
    Element,
    /// A vector.
    Vector,
    /// Any other entity which does not fit into the other categories.
    Other,
}

impl GroupData {
    pub fn new(parsing_uid: u64, name: Name, size: Option<usize>, kind: GroupKind) -> Self {
        GroupData {
            parsing_uid,
            name,
            attr: attribute::Attr::new(),
            size,
            kind,
        }
    }

    pub fn attr(&self) -> &attribute::Attr {
        &self.attr
    }

    pub fn kind(&self) -> GroupKind {
        self.kind
    }

    pub fn name(&self) -> &Name {
        &self.name
    }

    pub fn size(&self) -> Option<usize> {
        self.size
    }
}
