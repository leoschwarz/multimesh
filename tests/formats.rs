extern crate multimesh;

use multimesh::format::medit::MeditDeserializer;
use multimesh::data::face_vertex::Mesh;

#[test]
fn simple_medit() {
    let data = include_bytes!("files/blender-monkey.mesh");
    let mut mesh: Mesh = Mesh::default();
    MeditDeserializer::read(&data[..], &mut mesh).unwrap();
    //MeditDeserializer::read(&data[..], &mut mesh).expect("reading failed");
}

