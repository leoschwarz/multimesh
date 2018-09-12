use de::Deserializer;

pub struct PlySerializer {}
pub struct PlyDeserializer {}

impl Deserializer for PlyDeserializer {
    type Error = ();

    fn deserialize_into<S, T>(mut source: S, mut target: T) -> Result<(), Self::Error>
        where
            S: Read,
            T: DeserializeMesh
    {

    }
}