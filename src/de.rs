//! Mesh deserialization types.
//!
//! This module is format agnostic, the actual implementations are found within the `format` module
//! or within custom crates.

use data::{Attr, Group};
use nalgebra::DVector;

/// An error which can occur during mesh deserialization.
#[derive(Debug, Fail)]
pub enum DeserializerError {
    #[fail(display = "Deserializing failed due to IO error: {:?}", _0)]
    Io(::std::io::Error),

    #[fail(display = "Implementor of multimesh traits broke invariant, or internal bug: {:?}", _0)]
    BrokenInvariant(String),
}

pub trait DeserializeElement {
    fn indices(&mut self) -> Result<Option<DVector<usize>>, DeserializerError>;
    fn attr(&mut self) -> Result<Attr, DeserializerError>;
}

impl DeserializeElement for (DVector<usize>, Attr) {
    fn indices(&mut self) -> Result<Option<DVector<usize>>, DeserializerError> {
        // TODO: this is bad
        Ok(Some(self.0.clone()))
    }

    fn attr(&mut self) -> Result<Attr, DeserializerError> {
        // TODO: this could be even worse
        Ok(self.1.clone())
    }
}

/// Ability to receive a mesh being deserialized.
pub trait DeserializeMesh {
    fn de_dimension(&mut self, dim: u8);

    // TODO: should these methods also result a result?
    fn de_group_begin(&mut self, _group: &Group) {}
    fn de_group_end(&mut self, _group: &Group) {}

    /// Deserialize a node at a position and with attributes.
    ///
    /// TODO: DVector should be const generic size (when supported)
    fn de_node(
        &mut self,
        position: DVector<f64>,
        attr: Attr,
        group: &Group,
    ) -> Result<(), DeserializerError>;

    fn de_element<De: DeserializeElement>(
        &mut self,
        element: De,
        group: &Group,
    ) -> Result<(), DeserializerError>;
}
