// TODO: Early work in progress.

use de::Deserializer;
use std::io::Read;
use de::DeserializeMesh;
use util::item_reader::ItemReader;

pub struct PlySerializer {}
pub struct PlyDeserializer {}

#[derive(Debug)]
pub enum DeserializeError {
    Io(::std::io::Error),
    InvalidSyntax(String),
    UnsupportedVersion(String),
}

/*
impl Deserializer for PlyDeserializer {
    type Error = DeserializeError;

    fn deserialize_into<S, T>(mut source: S, mut target: T) -> Result<(), Self::Error>
        where
            S: Read,
            T: DeserializeMesh
    {
        // Read the file into memory.
        let mut data = String::new();
        source
            .read_to_string(&mut data)
            .map_err(|e| DeserializeError::Io(e))?;
        let mut reader = ItemReader::new(data.as_ref());

        if reader.next().unwrap_or("") != "ply" {
            return Err(DeserializeError::InvalidSyntax("Ply document does not begin with keyword ply.".into()));
        }

        while let Some(keyword) = reader.next() {
            match keyword {
                "format" => {
                    // TODO: support binary format
                    if reader.next() != Some("ascii") || reader.next() != Some("1.0") {
                        return Err(DeserializeError::UnsupportedVersion("Only 'ascii 1.0' ply supported as of now.".into()));
                    }
                },
                "comment" => {
                    // Ignore comments. (TODO: might be made more efficient.)
                    while let Some(_) = reader.next_until_eol() {}
                },
                "element" => {
                    let group_name: String = reader.next_parse_until_eol()?;
                    let group_len: usize = reader.next_parse_until_eol()?;

                    // TODO: How to handle ply groups, they might or might not fall into the element/node
                    // distinction which is common for most other formats.
                    // (1) Use this more general notion of elements for other formats.
                    //     → Implement helper methods like position(&self) -> Option<DVector> etc
                    // (2) Fit these into our types where possible, accumulate leftovers somewhere.
                    //     → Attr could handle many cases
                    //     → But how would we detect what can be interpreted like a vertex or element,
                    //       simply by the name. I guess ply relies on convention for this.
                    //       One way to handle this would be to insert this kind of information as
                    //       constructor argument to the PlyDeserializer struct and friends.
                    // (3)
                },
                "property" => {

                },
                "end_header" => break,
                kwd => return Err(DeserializeError::InvalidSyntax(format!("Unknown keyword: {}", kwd))),
            }
        }

        Ok(())
    }
}
*/