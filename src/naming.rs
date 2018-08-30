//! This module (partially) solves the problem where the same thing is named differently in
//! between formats.
//!
//! For example what is called `line` in one format could be called `Edge` in another.

// TODO: How could this work for user defined formats?

use std::borrow::Cow;

pub(crate) const NODES_MEDIT: &'static [&'static str] = &[
    "Vertices", // x_i y_i z_i ref_i
    "Normals", // x_i y_i z_i
    "Tangents", // x_i y_i z_i
];
pub(crate) const ELEMENTS_MEDIT: &'static [&'static str] = &[
    "Edges", // e1_i e2_i ref_i
    "Triangles", // v1_i v2_i v3_i ref_i
    "Quadrilaterals", // v1_i v2_i v3_i v4_i ref_i
    "Tetrahedra", // v1_i v2_i v3_i v4_i ref_i
    "Hexahedra", // v1_i v2_i v3_i v4_i v5_i v6_i v7_i v8_i ref_i
];
// TODO: I'm not sure yet what to do about these, technically they could
// all be handled as elements since they only hold integer references, but
// this could lead to confusion with mesh elements.
pub(crate) const OTHER_MEDIT: &'static [&'static str] = &[
    "Ridges", // e_i
    "RequiredEdges", // e_i
    "Corners", // v_i
    "RequiredVertices", // v_i
    "NormalAtVertices", // v_i n_i
    "NormalAtTriangleVertices", // t_i v_j n_i
    "NormalAtQuadrilateralVertices", // q_i v_j n_i
    "TangentAtEdges", // e_i v_j t_i
];

#[derive(Clone, Debug)]
pub struct Name {
    name: String,
    format: Format
}

impl Name {
    pub fn parse_node(s: String, format: Format) -> Option<Self> {
        match format {
            Format::Medit => {
                if NODES_MEDIT.contains(&s.as_str()) {
                    Some(Name {
                        name: s,
                        format: Format::Medit
                    })
                } else {
                    None
                }
            }
        }
    }

    pub fn parse_element(s: String, format: Format) -> Option<Self> {
        match format {
            Format::Medit => {
                if ELEMENTS_MEDIT.contains(&s.as_str()) {
                    Some(Name {
                        name: s,
                        format: Format::Medit
                    })
                } else {
                    None
                }
            }
        }
    }

    pub fn get_original(&self) -> (&str, Format) {
        (self.name.as_ref(), self.format)
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_element_name() {
        let name1 = Name::parse_element("Triangles".into(), Format::Medit);
        let name2 = Name::parse_element("Potato".into(), Format::Medit);
        assert!(name1.is_some());
        assert!(name2.is_none());
        let name = name1.unwrap();
        assert_eq!(name.get_original().0, "Triangles");
        assert_eq!(name.get_original().1, Format::Medit);
    }
}