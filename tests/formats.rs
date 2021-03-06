extern crate multimesh;

use multimesh::data::face_vertex::Mesh;
use multimesh::de::Deserializer;
use multimesh::format::medit::{MeditDeserializer, MeditSerializer};
use multimesh::ser::Serializer;
use std::fs::File;

#[test]
fn simple_de_medit() {
    let data = include_bytes!("files/blender-monkey.mesh");
    let mut mesh: Mesh = Mesh::default();
    MeditDeserializer::deserialize_into(&data[..], &mut mesh).unwrap();

    assert_eq!(mesh.metadata().dimension(), 3);
}

#[test]
fn simple_ser_medit() {
    let data = include_bytes!("files/blender-monkey.mesh");
    let mut mesh: Mesh = Mesh::default();
    MeditDeserializer::deserialize_into(&data[..], &mut mesh).unwrap();

    let ser = MeditSerializer::new();
    let output = File::create("tests/output1.mesh").unwrap();
    ser.serialize(&mesh, output).unwrap();
}
