use std::fmt::Debug;
use data::attribute::AttributeContainer;
use data::attribute::AttributeContainerMut;
use data::attribute::AttributeMap;

pub trait Entity: Clone + Debug {
    type Attr: AttributeContainer;

    fn kind(&self) -> EntityKind;
    fn attributes(&self) -> &Self::Attr;
}

pub trait EntityMut<Attr: AttributeContainerMut>: Entity<Attr=Attr> {
    fn attributes_mut(&mut self) -> &mut Attr;
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Copy)]
pub enum EntityKind {
    /// A mesh node/vertex.
    Node,
    /// A mesh element/face/volume.
    Element,
    /// A vector.
    Vector,
    /// Any other entity which does not fit into the other categories.
    Other,
}

#[derive(Clone, Debug)]
pub struct EntityBox {
    kind: EntityKind,
    attr: AttributeMap,
}

impl Entity for EntityBox {
    type Attr = AttributeMap;

    fn kind(&self) -> EntityKind {
        self.kind
    }

    fn attributes(&self) -> &Self::Attr {
        &self.attr
    }
}

impl EntityMut<AttributeMap> for EntityBox
{
    fn attributes_mut(&mut self) -> &mut AttributeMap {
        &mut self.attr
    }
}

impl EntityBox {
    pub fn from_entity<E, A>(e: &E) -> Self
    where
        E: Entity<Attr=A>,
        A: AttributeContainer
    {
        EntityBox {
            kind: e.kind(),
            attr: AttributeMap::from_container(e.attributes())
        }
    }
}