//! Mesh deserialization types.
//!
//! This module is format agnostic, the actual implementations are found within the `format` module
//! or within custom crates.

use data::attribute::Attr;
use data::mesh::{ReadElement, ReadEntity, ReadNode, ReadVector};
use data::{AttrName, GroupData};
use nalgebra::DVector;
use std::borrow::Cow;
use std::io::Read;

pub trait Deserializer {
    type Error;

    fn deserialize_into<S, T>(source: S, target: T) -> Result<(), Self::Error>
    where
        S: Read,
        T: DeserializeMesh;
}

/// An error which can occur during mesh deserialization.
#[derive(Debug, Fail)]
pub enum DeserializerError {
    #[fail(display = "Deserializing failed due to IO error: {:?}", _0)]
    Io(::std::io::Error),

    #[fail(display = "Implementor of multimesh traits broke invariant, or internal bug: {:?}", _0)]
    BrokenInvariant(String),

    #[fail(display = "Other error: {:?}", _0)]
    Other(Box<::std::error::Error + Send + Sync>),
}

/// Ability to receive a mesh being deserialized.
pub trait DeserializeMesh {
    fn de_dimension(&mut self, dim: u8);

    /// Invoked immediately before deserializing a group of entities.
    fn de_group_begin(&mut self, _group: &GroupData) -> Result<(), DeserializerError> {
        Ok(())
    }

    /// Invoked immediately after deserializing a group of entities.
    fn de_group_end(&mut self, _group: &GroupData) -> Result<(), DeserializerError> {
        Ok(())
    }

    /// Invoked for each entity of a group, unless one of the more specific handlers is invoked.
    fn de_entity<R>(&mut self, entity: &R, group: &GroupData) -> Result<(), DeserializerError>
    where
        R: ReadEntity<Error = DeserializerError>;

    /// Invoked for node/vertex entities instead of `de_entity` if the format metadata defines
    /// the entity as a node entity.
    ///
    /// The default implementation invokes `de_entity` as a fallback.
    fn de_node<R>(&mut self, node: &R, group: &GroupData) -> Result<(), DeserializerError>
    where
        R: ReadNode<Error = DeserializerError>,
    {
        self.de_entity(node, group)
    }

    /// Invoked for element/face/volume entities instead of `de_entity` if the format metadata
    /// defines the entity as a element entity.
    ///
    /// The default implementation invokes `de_entity` as a fallback.
    fn de_element<R>(&mut self, element: &R, group: &GroupData) -> Result<(), DeserializerError>
    where
        R: ReadElement<Error = DeserializerError>,
    {
        self.de_entity(element, group)
    }

    /// Invoked for vector entities instead of `de_entity` if the format metadata
    /// defines the entity as a vector entity.
    ///
    /// The default implementation invokes `de_entity` as a fallback.
    fn de_vector<R>(&mut self, vector: &R, group: &GroupData) -> Result<(), DeserializerError>
    where
        R: ReadVector<Error = DeserializerError>,
    {
        self.de_entity(vector, group)
    }
}

/*
// TODO: Do we need these impls?

impl DeserializeEntity for (DVector<usize>, Attr) {
    fn attr_at(&self, index: usize) -> Option<(AttrName, Cow<str>)> {
        // TODO: This is silly, we format a float as string here so it can be parsed back.
        self.1
            .get_at(index)
            .map(|(n, s)| (n.clone(), Cow::Owned(format!("{}", s))))
    }
}

impl DeserializeElement for (DVector<usize>, Attr) {
    fn indices(&self) -> Result<Option<Cow<DVector<usize>>>, DeserializerError> {
        Ok(Some(Cow::Borrowed(&self.0)))
    }
*/
