use bevy::prelude::*;
use bevy::render::pipeline::PrimitiveTopology;
use bevy::render::mesh::VertexAttributeValues;
use crate::prelude::*;

pub fn wire_segment(segment: &Segment) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::LineList);
    mesh.set_attribute(
        Mesh::ATTRIBUTE_POSITION,
        VertexAttributeValues::from(
            vec![
                [segment.a.x, 0.0, segment.a.y],
                [segment.b.x, 0.0, segment.b.y]
            ],
        ),
    );
    mesh
}
