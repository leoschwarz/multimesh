use data::attribute::Attr;
use data::AttrName;
use error::Error;
use nalgebra::DVector;
use std::borrow::Cow;
use std::fmt;
use std::str::FromStr;

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
    // TODO: The API would be a lot more flexible with this type here,
    // however in a first attempt it turned out to complicate the writing
    // of generic code in the format impls significantly unless we end up
    // implementing them only for a specific error, which would not be any
    // benefit against forcing everyone to use our error type.
    //type Error;

    /// Get the attribute of the specified name.
    ///
    /// If the value is `None` such an attribute was not found.
    fn attr(&self, name: &AttrName) -> Result<Option<Cow<str>>, Error>;

    /// Get the attribute at the specified index.
    ///
    /// If the value is `None`, it will be None for all subsequent
    /// indices.
    fn attr_at(&self, index: usize) -> Result<Option<(Cow<AttrName>, Cow<str>)>, Error>;

    fn attr_parse<T>(&self, name: &AttrName) -> Result<Option<T>, Error>
    where
        T: FromStr,
        <T as FromStr>::Err: fmt::Debug + fmt::Display,
    {
        match self.attr(name)? {
            Some(s) => s
                .parse::<T>()
                .map(|t| Some(t))
                .map_err(|e| Error::Syntax(format!("Parsing failed: {} {:?}", e, e))),
            None => Ok(None),
        }
    }
}

// TODO: Using Cow everywhere is actually stupid if there is no way that Cow::Owned
// can be returned as in general it would require a mutable reference or owned self.
pub trait ReadElement: ReadEntity {
    fn node_indices(&self) -> Result<Option<Cow<DVector<usize>>>, Error>;
}

pub trait ReadNode: ReadEntity {
    fn position(&self) -> Result<Cow<DVector<f64>>, Error>;
}

pub trait ReadVector: ReadEntity {
    fn components(&self) -> Result<Cow<DVector<f64>>, Error>;
}

impl ReadEntity for (DVector<usize>, Attr) {
    fn attr(&self, name: &AttrName) -> Result<Option<Cow<str>>, Error> {
        Ok(self.1.get(name).map(|s| Cow::Borrowed(s.as_ref())))
    }

    fn attr_at(&self, index: usize) -> Result<Option<(Cow<AttrName>, Cow<str>)>, Error> {
        Ok(self
            .1
            .get_at(index)
            .map(|(n, v)| (Cow::Borrowed(n), Cow::Borrowed(v.as_ref()))))
    }
}

impl ReadElement for (DVector<usize>, Attr) {
    fn node_indices(&self) -> Result<Option<Cow<DVector<usize>>>, Error> {
        Ok(Some(Cow::Borrowed(&self.0)))
    }
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
