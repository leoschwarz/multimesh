use error::Error;
use std::collections::{BTreeMap, btree_map};
use std::iter::FromIterator;

/// The name of an attribute.
///
/// In some cases attributes don't have a string key attached to them,
/// but are referred to by a numeric index or assigned one according to
/// their position in a list of attributes.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum AttributeName {
    Index(usize),
    Key(String),
}

impl From<usize> for AttributeName {
    fn from(i: usize) -> Self {
        AttributeName::Index(i)
    }
}

impl From<String> for AttributeName {
    fn from(k: String) -> Self {
        AttributeName::Key(k)
    }
}

pub trait AttributeContainer {
    /// The number of contained attributes.
    fn len(&self) -> usize;

    /// Get the attribute with the specified name.
    fn get(&self, name: &AttributeName) -> Option<&String>;

    fn iter<'a>(&'a self) -> Box<Iterator<Item=(&'a AttributeName, &'a String)> + 'a>;

    fn iter_names<'a>(&'a self) -> Box<Iterator<Item=&'a AttributeName> + 'a> {
        Box::new(self.iter().map(|(k,v)| k))
    }
}

pub trait AttributeContainerMut: AttributeContainer {
    /// Set the attribute to a value.
    fn set(&mut self, name: AttributeName, value: String);
}

#[derive(Clone, Debug, Default)]
pub struct AttributeMap {
    data: BTreeMap<AttributeName, String>,
}

impl AttributeContainer for AttributeMap {
    fn len(&self) -> usize {
        self.data.len()
    }

    fn get(&self, name: &AttributeName) -> Option<&String> {
        self.data.get(name)
    }

    fn iter<'a>(&'a self) -> Box<Iterator<Item=(&'a AttributeName, &'a String)> + 'a> {
        Box::new(self.data.iter())
    }

    fn iter_names<'a>(&'a self) -> Box<Iterator<Item=&'a AttributeName> + 'a> {
        Box::new(self.data.keys())
    }
}

impl AttributeContainerMut for AttributeMap {
    fn set(&mut self, name: AttributeName, value: String) {
        self.data.insert(name, value);
    }
}

impl AttributeMap {
    fn from_container<A: AttributeContainer>(c: A) -> Self {
        let mut data = BTreeMap::new();
        for (k, v) in c.iter() {
            data.insert(k.clone(), v.clone());
        }
        AttributeMap {
            data
        }
    }
}

impl AttributeName {
    pub fn is_index(&self) -> bool {
        match *self {
            AttributeName::Index(_) => true,
            AttributeName::Key(_) => false
        }
    }

    pub fn is_key(&self) -> bool {
        match *self {
            AttributeName::Index(_) => false,
            AttributeName::Key(_) => true,
        }
    }
}

impl ToString for AttributeName {
    fn to_string(&self) -> String {
        match *self {
            AttributeName::Index(ref i) => format!("{}", i),
            AttributeName::Key(ref s) => s.clone(),
        }
    }
}
