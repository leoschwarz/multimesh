//! TODO: WIP
//!
//! Definition: http://paulbourke.net/dataformats/ply/

// TODO: Early work in progress.

use data::{attribute::Attr, face_vertex::Entity, AttrName, GroupData, GroupKind};
use de::{DeserializeMesh, Deserializer};
use error::Error;
use naming::{Format, Name};
use std::{io::Read, str::FromStr};
use util::item_reader::ItemReader;

pub struct PlySerializer {}
pub struct PlyDeserializer {}

enum DataType {
    Char,
    Uchar,
    Short,
    Ushort,
    Int,
    Uint,
    Float,
    Double,
}

impl FromStr for DataType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        match s {
            "char" => Ok(DataType::Char),
            "uchar" => Ok(DataType::Uchar),
            "short" => Ok(DataType::Short),
            "ushort" => Ok(DataType::Ushort),
            "int" => Ok(DataType::Int),
            "uint" => Ok(DataType::Uint),
            "float" => Ok(DataType::Float),
            "double" => Ok(DataType::Double),
            _ => Err(Error::Syntax(format!("Unknown data type: {}", s))),
        }
    }
}

struct Group {
    data: GroupData,
    attrs: Vec<(AttrName, DataType)>,
}

impl Deserializer for PlyDeserializer {
    fn deserialize_into<S, T>(mut source: S, mut target: T) -> Result<(), Error>
    where
        S: Read,
        T: DeserializeMesh,
    {
        // Read the file into memory.
        let mut data = String::new();
        source.read_to_string(&mut data)?;
        let mut reader = ItemReader::new(data.as_ref());

        if reader.next().unwrap_or("") != "ply" {
            return Err(Error::Syntax(
                "Ply document does not begin with keyword ply.".into(),
            ));
        }

        // Read the header.
        let mut groups: Vec<Group> = Vec::new();
        let mut parsing_uid: u64 = 0;
        let mut header_ended = false;

        while let Some(keyword) = reader.next() {
            match keyword {
                "format" => {
                    // TODO: support binary format
                    if reader.next() != Some("ascii") || reader.next() != Some("1.0") {
                        return Err(Error::Syntax(
                            "Only 'ascii 1.0' ply supported as of now.".into(),
                        ));
                    }
                }
                "comment" => {
                    // Ignore comments. (TODO: might be made more efficient.)
                    while let Some(_) = reader.next_until_eol() {}
                }
                "element" => {
                    let group_name_str: String = reader.next_parse_until_eol()?;
                    let group_name = Name::parse(group_name_str, Format::Ply, GroupKind::Other)
                        .ok_or_else(|| Error::OtherInternal("Should never happen".into()))?;
                    let group_len: usize = reader.next_parse_until_eol()?;

                    groups.push(Group {
                        data: GroupData::new(
                            parsing_uid,
                            group_name,
                            Some(group_len),
                            GroupKind::Other,
                        ),
                        attrs: Vec::new(),
                    });

                    parsing_uid += 1;
                }
                "property" => {
                    let prop_type: DataType = reader.next_parse()?;
                    let prop_name = AttrName::Key(reader.next_result()?.into());

                    if let Some(ref mut group) = groups.last_mut() {
                        group.attrs.push((prop_name, prop_type));
                    } else {
                        return Err(Error::Syntax("property encountered before element.".into()));
                    }
                }
                "end_header" => {
                    header_ended = true;
                    break;
                }
                kwd => return Err(Error::Syntax(format!("Unknown keyword: {}", kwd))),
            }
        }

        if !header_ended {
            return Err(Error::Syntax(
                "Header not terminated by `end_header` keyword.".into(),
            ));
        }

        // Parse body.
        for ref group_info in groups.iter() {
            target.de_group_begin(&group_info.data)?;

            for _ in 0..group_info.data.size().unwrap() {
                let mut attr = Attr::new();
                for (ref attr_name, ref _attr_type) in &group_info.attrs {
                    let val = reader.next_result()?;
                    attr.insert(attr_name.clone(), val.into());

                    // TODO: respect the attr types?
                    /*
                    match attr_type {
                        DataType::Char | DataType::Short | DataType::Int => {
                        }
                    }
                    */                }
                // TODO: handle the different types of entities.
                target.de_entity(&&Entity { attr }, &group_info.data)?;
            }

            target.de_group_end(&group_info.data)?;
        }

        Ok(())
    }
}
