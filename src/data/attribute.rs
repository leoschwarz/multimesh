use data::mesh::ReadEntity;
use error::Error;
use std::collections::BTreeMap;

/// The name of an attribute.
///
/// In some cases attributes don't have a string key attached to them,
/// but are referred to by a numeric index or assigned one according to
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

/// Holds attributes of entities.
///
/// Note that in general this might not be a very efficient solution,
/// as each instance contains a `BTreeMap` internally.
/// It's offered for convenience but in practice you might want to
/// process attributes appropriately while deserializing into your target
/// data structure.
#[derive(Clone, Debug)]
pub struct Attr {
    values: BTreeMap<AttrName, String>,
}

impl Attr {
    /// Create a new and empty attr instance.
    pub fn new() -> Self {
        Attr {
            values: BTreeMap::new(),
        }
    }

    pub fn get(&self, name: &AttrName) -> Option<&String> {
        self.values.get(name)
    }

    pub fn get_at(&self, index: usize) -> Option<(&AttrName, &String)> {
        // TODO: is this efficient enough?
        self.values.iter().nth(index)
    }

    pub fn insert(&mut self, name: AttrName, value: String) {
        self.values.insert(name.into(), value);
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub(crate) fn from_entity<R>(de: &R) -> Result<Self, Error>
    where
        R: ReadEntity,
    {
        let mut attr = Attr::new();
        // TODO: Consider using Iterator instead.
        let mut i = 0;
        while let Some((name, val)) = de.attr_at(i)? {
            attr.insert(name.into_owned(), val.into_owned());
            i += 1;
        }
        Ok(attr)
    }

    // TODO: implement all necessary methods. (Maybe also implement ops::Index and Iter.)
}
