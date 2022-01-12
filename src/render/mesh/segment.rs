use crate::prelude::*;
use bevy::prelude::*;
use bevy::render::mesh::VertexAttributeValues;
use wgpu_types::PrimitiveTopology;

pub fn wire_segment(segment: &Segment, config: &RapierConfiguration) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.set_attribute(
        Mesh::ATTRIBUTE_POSITION,
        VertexAttributeValues::from(vec![[segment.a.x, segment.a.y], [segment.b.x, segment.b.y]]),
    );
    mesh
}
