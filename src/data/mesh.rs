use data::AttrName;
use nalgebra::DVector;
use std::borrow::Cow;

/*
pub trait ReadAttribute {
    type Error;

    /// Get the attribute at the specified index.
    ///
    /// If the value is `None`, it will be None for all subsequent
    /// indices.
    fn attr_at(&self, index: usize) -> Result<Option<(Cow<AttrName>, Cow<str>)>, Self::Error>;

    /// Get the attribute of the specified name.
    ///
    /// If the value is `None` such an attribute was not found.
    fn attr_get(&self, name: &AttrName) -> Result<Option<Cow<str>>, Self::Error>;
}

pub trait ReadEntity: ReadAttribute {}
*/

pub trait ReadEntity {
    type Error;

    /// Get the attribute at the specified index.
    ///
    /// If the value is `None`, it will be None for all subsequent
    /// indices.
    fn attr_at(&self, index: usize) -> Result<Option<(Cow<AttrName>, Cow<str>)>, Self::Error>;

    /// Get the attribute of the specified name.
    ///
    /// If the value is `None` such an attribute was not found.
    fn attr_get(&self, name: &AttrName) -> Result<Option<Cow<str>>, Self::Error>;
}

pub trait ReadElement: ReadEntity {
    fn node_indices(&self) -> Result<Option<Cow<DVector<usize>>>, Self::Error>;
}

pub trait ReadNode: ReadEntity {
    fn position(&self) -> Result<Cow<DVector<f64>>, Self::Error>;
}

pub trait ReadVector: ReadEntity {
    fn components(&self) -> Result<Cow<DVector<f64>>, Self::Error>;
}

// TODO keep or delete
/*
impl<'a, T> DeserializeEntity for &'a T
where
    T: DeserializeEntity,
{
    fn attr_at(&self, index: usize) -> Option<(AttrName, Cow<str>)> {
        (*self).attr_at(index)
    }
}
*/
