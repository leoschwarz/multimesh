use data::{DeserializeMesh, Attr};
use regex::{Regex, RegexBuilder};
use std::fmt::Display;
use std::io::{self, Read};
use std::str::FromStr;
use std::marker::PhantomData;
use nalgebra::DVector;

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
                    target.reserve_nodes(num_nodes, dimension, 1);

                    for _ in 0..num_nodes {
                        let mut position = DVector::<f64>::zeros(dimension);
                        for i in 0..dimension {
                            position[i] = reader.get_val()?;
                        }
                        let mut attr = Attr::new();
                        attr.insert(0, reader.get_val()?);

                        target.de_node(position, attr);
                    }
                }
                _ => unimplemented!(),
            }
        }

        Ok(())
    }
}

struct ItemReader<'s> {
    data: &'s str,
    pointer: usize,
}

impl<'s> ItemReader<'s> {
    fn new(data: &'s str) -> Self {
        ItemReader {
            data: data,
            pointer: 0,
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
        lazy_static! {
            static ref RE: Regex = RegexBuilder::new(r"\S+").multi_line(true).build().unwrap();
        }

        let mat = RE.find(&self.data[self.pointer..])?;
        self.pointer += mat.end();
        Some(mat.as_str())
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
