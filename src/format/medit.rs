use data::{DeserializeMesh, Attr, Group, GroupKind};
use regex::{Regex, RegexBuilder};
use std::collections::VecDeque;
use std::fmt::Display;
use std::io::{self, Read};
use std::str::{FromStr, Lines};
use std::marker::PhantomData;
use nalgebra::DVector;

#[derive(Debug)]
pub enum DeserializeError {
    /// There was a problem with I/O.
    Io(io::Error),

    /// Unsupported version or format encounterd.
    Unsupported(String),

    /// Parsing the data failed for one of many possible reasons.
    Parse(String),
}

pub struct MeditDeserializer {}

// TODO convert to trait
impl MeditDeserializer {
    pub fn read<S, T>(mut source: S, mut target: T) -> Result<(), DeserializeError>
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
        let mut version: Option<&str> = None;
        let mut dimension: usize = 0;
        let mut parsing_uid: u64 = 0;

        while let Some(keyword) = reader.next() {
            match keyword {
                "MeshVersionFormatted" => {
                    version = Some(reader.get_next()?);
                }
                "Dimension" => {
                    dimension = reader
                        .get_next()?
                        .parse()
                        .map_err(|_| DeserializeError::Parse("Dimension".into()))?;
                }
                "Vertices" => {
                    if dimension == 0 {
                        return Err(DeserializeError::Parse("Bad dimension.".into()));
                    }

                    let num_nodes: usize = reader.get_val()?;

                    parsing_uid += 1;
                    let group = Group::new(parsing_uid, "Vertices", Some(num_nodes), GroupKind::Node);
                    target.de_group_begin(&group);

                    for _ in 0..num_nodes {
                        let mut position = DVector::<f64>::zeros(dimension);
                        for i in 0..dimension {
                            position[i] = reader.get_val()?;
                        }
                        let mut attr = Attr::new();
                        attr.insert(0, reader.get_val()?);

                        target.de_node(position, attr, &group);
                    }

                    target.de_group_end(&group);
                }
                "Triangles" => {
                    let num_elements: usize = reader.get_val()?;

                    parsing_uid += 1;
                    let group = Group::new(parsing_uid, "Triangles", Some(num_elements), GroupKind::Element);
                    target.de_group_begin(&group);

                    for _ in 0..num_elements {
                        let mut indices = DVector::<usize>::from_element(3, 0);
                        for i_no in 0..3 {
                            indices[i_no] = reader.get_val()?;
                        }
                        let mut attr = Attr::new();
                        attr.insert(0, reader.get_val()?);
                        target.de_element((indices, attr), &group);
                    }

                    target.de_group_end(&group);
                }
                other => {
                    if other.trim().is_empty() || other.starts_with("#") {
                        // Ignore.
                    } else {
                        return Err(DeserializeError::Parse(format!("Unsupported keyword: {}", other)));
                    }
                }
            }
        }

        Ok(())
    }
}

/*
struct ItemReader<'s> {
    data: &'s str,
    lines: Lines<'s>,
    line_buff
    pointer: usize,
}
*/

struct ItemReader<'s> {
    lines: Lines<'s>,
    line_buf: VecDeque<&'s str>,
}

impl<'s> ItemReader<'s> {
    /*
    fn new(data: &'s str) -> Self {
        ItemReader {
            data: data,
            lines: data.lines(),
            pointer: 0,
        }
    }
    */
    fn new(data: &'s str) -> Self {
        ItemReader {
            lines: data.lines(),
            line_buf: VecDeque::new(),
        }
    }

    fn get_val<T>(&mut self) -> Result<T, DeserializeError>
    where
        T: FromStr,
        <T as FromStr>::Err: Display,
    {
        self.get_next().and_then(|s| {
            s.parse()
                .map_err(|e| DeserializeError::Parse(format!("Parse value failed: {}", e)))
        })
    }

    fn get_next(&mut self) -> Result<&'s str, DeserializeError> {
        self.next()
            .ok_or_else(|| DeserializeError::Parse("Unexpected EOF".into()))
    }
}

impl<'s> Iterator for ItemReader<'s> {
    type Item = &'s str;

    fn next(&mut self) -> Option<Self::Item> {
        let mut probe_buf = |line_buf: &mut VecDeque<&'s str>| {
            while let Some(item) = line_buf.remove(0) {
                if !item.trim().is_empty() {
                    return Some(item);
                }
            }
            None
        };

        if let Some(item) = probe_buf(&mut self.line_buf) {
            return Some(item);
        }

        lazy_static! {
            static ref RE: Regex = Regex::new(r"\s+").unwrap();
        }

        while let Some(line) = self.lines.next() {
            if !line.starts_with("#") && !line.trim().is_empty() {
                self.line_buf.extend(RE.split(line));
                if let Some(item) = probe_buf(&mut self.line_buf) {
                    return Some(item);
                }
            }
        }

        None
    }
}

/*
struct ValueReader<'s, V> {
    reader: &'s mut ItemReader<'s>,
    _value_type: PhantomData<V>,
}

impl<'s, V> Iterator for ValueReader<'s, V>
where
    V: FromStr,
    <V as FromStr>::Err: Display
{
    type Item = V;

    fn next(&mut self) -> Option<Self::Item> {
        self.reader.get_val().ok()
    }
}
*/

mod tests {
    use super::*;

    #[test]
    fn item_reader() {
        let source = " abc   xyz  -1\n\r\t \n  abc xyz";
        let mut reader = ItemReader::new(source);

        assert_eq!(reader.next(), "abc".into());
        assert_eq!(reader.next(), "xyz".into());
        assert_eq!(reader.next(), "-1".into());
        assert_eq!(reader.next(), "abc".into());
        assert_eq!(reader.next(), "xyz".into());
        assert_eq!(reader.next(), None);
    }
}
