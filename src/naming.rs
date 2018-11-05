//! This module (partially) solves the problem where the same thing is named differently in
//! between formats.
//!
//! For example what is called `line` in one format could be called `Edge` in another.

// TODO: How could this work for user defined formats?

use data::EntityKind;
use std::borrow::Cow;

pub(crate) const NODES_MEDIT: &'static [&'static str] = &[
    "Vertices", // x_i y_i z_i ref_i
];
pub(crate) const VECTORS_MEDIT: &'static [&'static str] = &[
    "Normals",  // x_i y_i z_i
    "Tangents", // x_i y_i z_i
];
pub(crate) const ELEMENTS_MEDIT: &'static [&'static str] = &[
    "Edges",          // e1_i e2_i ref_i
    "Triangles",      // v1_i v2_i v3_i ref_i
    "Quadrilaterals", // v1_i v2_i v3_i v4_i ref_i
    "Tetrahedra",     // v1_i v2_i v3_i v4_i ref_i
    "Hexahedra",      // v1_i v2_i v3_i v4_i v5_i v6_i v7_i v8_i ref_i
];
pub(crate) const OTHER_MEDIT: &'static [&'static str] = &[
    "Ridges",                        // e_i
    "RequiredEdges",                 // e_i
    "Corners",                       // v_i
    "RequiredVertices",              // v_i
    "NormalAtVertices",              // v_i n_i
    "NormalAtTriangleVertices",      // t_i v_j n_i
    "NormalAtQuadrilateralVertices", // q_i v_j n_i
    "TangentAtEdges",                // e_i v_j t_i
];

#[derive(Clone, Debug)]
pub struct Name {
    name: String,
    format: Format,
    kind: EntityKind,
}

impl Name {
    pub fn parse(s: String, format: Format, kind: EntityKind) -> Option<Self> {
        match format {
            Format::Medit => {
                let ref whitelist = match kind {
                    EntityKind::Node => NODES_MEDIT,
                    EntityKind::Element => ELEMENTS_MEDIT,
                    EntityKind::Vector => VECTORS_MEDIT,
                    EntityKind::Other => OTHER_MEDIT,
                };

                if !whitelist.contains(&s.as_str()) {
                    return None;
                }
            }
            Format::Ply => {
                // No validation.
                // TODO Are there naming conventions for "ply elements"?
            }
        }

        Some(Name {
            name: s,
            format,
            kind: kind.clone(),
        })
    }

    pub fn get_original(&self) -> (&str, Format, EntityKind) {
        (self.name.as_ref(), self.format, self.kind)
    }

    pub fn get_as(&self, f: Format) -> Option<Cow<str>> {
        if f == self.format {
            Some(Cow::Borrowed(&self.name))
        } else {
            // TODO
            unimplemented!()
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Copy)]
pub enum Format {
    Medit,
    Ply,
    // TODO: Allow formats other than the ones implemented together with this crate.
    //Other(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_element_name() {
        let name1 = Name::parse("Triangles".into(), Format::Medit, EntityKind::Element);
        let name2 = Name::parse("Potato".into(), Format::Medit, EntityKind::Element);
        assert!(name1.is_some());
        assert!(name2.is_none());
        let name = name1.unwrap();
        assert_eq!(name.get_original().0, "Triangles");
        assert_eq!(name.get_original().1, Format::Medit);
    }
}
