//! Implementation of MEDIT mesh format support.
//!
//! Defined in [https://www.ljll.math.upmc.fr/frey/publications/RT-0253.pdf](ISSN 0249-0803) .

use data::{attribute::Attr, GroupData, GroupKind};
use data::mesh::{ReadVector,ReadEntity,ReadNode,ReadElement};
use de::Deserializer;
use de::{DeserializeMesh, DeserializerError};
use nalgebra::DVector;
use naming::Format;
use naming::Name;
use ser::{SerializableGroup, SerializableMesh, Serializer};
use std::io::{self, Read, Write};
use util::item_reader::{ItemReader, ItemReaderError};

fn element_nary(element_name: &str) -> Option<usize> {
    match element_name {
        "Edges" => Some(2),
        "Triangles" => Some(3),
        "Quadrilaterals" => Some(4),
        "Tetrahedra" => Some(4),
        "Hexahedra" => Some(8),
        _ => None,
    }
}

#[derive(Debug)]
pub enum DeserializeError {
    /// There was a problem with I/O.
    Io(io::Error),

    /// Unsupported version or format encountered.
    Unsupported(String),

    /// Parsing the data failed for one of many possible reasons.
    Parse(String),

    Deserializer(DeserializerError),

    Reader(ItemReaderError),
}

impl From<DeserializerError> for DeserializeError {
    fn from(e: DeserializerError) -> Self {
        DeserializeError::Deserializer(e)
    }
}

impl From<ItemReaderError> for DeserializeError {
    fn from(e: ItemReaderError) -> Self {
        DeserializeError::Reader(e)
    }
}

#[derive(Debug)]
pub enum SerializeError {
    InvalidElementGroup(String),
    Io(io::Error),
}

impl From<io::Error> for SerializeError {
    fn from(e: io::Error) -> Self {
        SerializeError::Io(e)
    }
}

pub struct MeditSerializer {}

impl MeditSerializer {
    pub fn new() -> Self {
        MeditSerializer {}
    }

    fn serialize_group<G, GS, W, F, FR>(
        &self,
        groups: GS,
        mut target: W,
        serialize_item: F,
    ) -> <Self as Serializer>::Result
    where
        G: SerializableGroup,
        GS: Iterator<Item = G>,
        W: Write,
        F: Fn(&mut W, <G as SerializableGroup>::Item, &str) -> FR,
        FR: Into<<Self as Serializer>::Result>,
    {
        for group in groups {
            let group_metadata = group.metadata();
            // TODO: Rename error to "InvalidGroup".
            let group_name = group_metadata
                .name()
                .get_as(Format::Medit)
                .ok_or_else(|| SerializeError::InvalidElementGroup("No name".into()))?;

            writeln!(target, "{}\n{}", group_name, group.len())?;

            for i in 0..group_metadata.len() {
                // TODO: Remove unwrap, this is an invariant violation error.
                let item = group.item_at(i).unwrap();
                serialize_item(&mut target, item, &group_name).into()?;
            }

            writeln!(target, "")?;
        }

        Ok(())
    }

    fn serialize_node<N, W>(node: N, mut target: W, mesh_dim: u8) -> <Self as Serializer>::Result
    where
        N: SerializableNode,
        W: Write,
    {
        let p = node.position();
        let attr = node.attr().get(0).unwrap_or(0.);
        if mesh_dim == 2 {
            writeln!(target, "{} {} {}", p[0], p[1], attr)?;
        } else if mesh_dim == 3 {
            writeln!(target, "{} {} {} {}", p[0], p[1], p[2], attr)?;
        } else {
            // TODO
            panic!("unsupported");
        }
        Ok(())
    }

    fn serialize_element<E, W>(
        element: E,
        mut target: W,
        nary: usize,
    ) -> <Self as Serializer>::Result
    where
        E: SerializableElement,
        W: Write,
    {
        let is = element.node_indices().unwrap();
        let attr = element.attr().get(0).unwrap_or(0.);

        for j in 0..nary {
            write!(target, "{} ", is[j])?;
        }
        writeln!(target, "{}", attr)?;
        Ok(())
    }
}

impl Serializer for MeditSerializer {
    type Result = Result<(), SerializeError>;

    fn serialize<M, W>(&self, mesh: M, mut target: W) -> Self::Result
    where
        M: SerializableMesh,
        W: Write,
    {
        // TODO: include version information of crate
        writeln!(target, "MeshVersionFormatted 1")?;
        writeln!(target, "# MEDIT mesh file, generated by multimesh")?;

        // Get dimensionality.
        let mesh_dim = mesh.metadata().dimension();
        // TODO: handle other dimensions, remove assert
        assert!(mesh_dim == 2 || mesh_dim == 3);
        writeln!(target, "Dimension {}\n", mesh_dim)?;

        self.serialize_group(mesh.node_groups(), &mut target, |tgt, node, _| {
            Self::serialize_node(node, tgt, mesh_dim)
        })?;
        self.serialize_group(mesh.element_groups(), &mut target, |tgt, element, name| {
            let nary = element_nary(name).unwrap();
            Self::serialize_element(element, tgt, nary)
        })?;

        writeln!(target, "End")?;

        Ok(())
    }
}

pub struct MeditDeserializer {}

impl Deserializer for MeditDeserializer {
    type Error = DeserializeError;

    fn deserialize_into<S, T>(mut source: S, mut target: T) -> Result<(), DeserializeError>
    where
        S: Read,
        T: DeserializeMesh,
    {
        // Read the file into memory.
        let mut data = String::new();
        source
            .read_to_string(&mut data)
            .map_err(|e| DeserializeError::Io(e))?;
        let mut reader = ItemReader::new(data.as_ref());

        // Read data.
        let mut dimension: usize = 0;
        let mut parsing_uid: u64 = 0;

        while let Some(keyword) = reader.next() {
            match keyword {
                "MeshVersionFormatted" => {
                    let version: &str = reader.next_result()?;
                    if version != "1" {
                        return Err(DeserializeError::Parse(format!(
                            "Unsupported version: {}",
                            version
                        )));
                    }
                }
                "Dimension" => {
                    dimension = reader
                        .next_result()?
                        .parse()
                        .map_err(|_| DeserializeError::Parse("Dimension".into()))?;
                    target.de_dimension(dimension as u8);
                }
                "Vertices" | "Normals" | "Tangents" => {
                    if dimension != 3 {
                        return Err(DeserializeError::Parse("Bad dimension.".into()));
                    }

                    let num_nodes: usize = reader.next_parse()?;

                    parsing_uid += 1;
                    let group_name = Name::parse_node(keyword.into(), Format::Medit).unwrap();
                    let group =
                        GroupData::new(parsing_uid, group_name, Some(num_nodes), GroupKind::Node);
                    target.de_group_begin(&group)?;

                    for _ in 0..num_nodes {
                        let mut position = DVector::<f64>::zeros(dimension);
                        for i in 0..dimension {
                            position[i] = reader.next_parse()?;
                        }
                        let mut attr = Attr::new();
                        if keyword == "Vertices" {
                            attr.insert(0, reader.next_parse()?);
                        }

                        target.de_node(position, attr, &group)?;
                    }

                    target.de_group_end(&group)?;
                }
                "Edges" | "Triangles" | "Quadrilaterals" | "Tetrahedra" | "Hexahedra" => {
                    let num_elements: usize = reader.next_parse()?;
                    // Note: Should never fail by definition of `element_nary`.
                    let nary = element_nary(keyword).unwrap();

                    parsing_uid += 1;
                    let group_name = Name::parse_element(keyword.into(), Format::Medit).unwrap();
                    let group = GroupData::new(
                        parsing_uid,
                        group_name,
                        Some(num_elements),
                        GroupKind::Element,
                    );
                    target.de_group_begin(&group)?;

                    for _ in 0..num_elements {
                        let mut indices = DVector::<usize>::from_element(nary, 0);
                        for i_no in 0..nary {
                            indices[i_no] = reader.next_parse()?;
                        }
                        let mut attr = Attr::new();
                        attr.insert(0, reader.next_parse()?);
                        target.de_element((indices, attr), &group)?;
                    }

                    target.de_group_end(&group)?;
                }
                "End" => {
                    // TODO: Maybe it would be better to set a flag and check
                    // if there is more content anyway, the problem with this
                    // is that there might be an obscure convention where someone
                    // puts different data after the end keyword, or if reading
                    // from a stream of multiple medit meshes.
                    return Ok(());
                }
                other => {
                    if other.trim().is_empty() || other.starts_with("#") {
                        // Ignore.
                    } else {
                        return Err(DeserializeError::Parse(format!(
                            "Unsupported keyword: {}",
                            other
                        )));
                    }
                }
            }
        }

        Ok(())
    }
}
