use crate::prelude::*;
use bevy::prelude::*;
use bevy::render::mesh::{Indices, VertexAttributeValues};
use wgpu_types::PrimitiveTopology;

#[cfg(feature = "dim3")]
pub fn wire_polyline(polyline: &Polyline, config: &RapierConfiguration) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.set_attribute(
        Mesh::ATTRIBUTE_POSITION,
        VertexAttributeValues::from(
            polyline
                .vertices()
                .iter()
                .map(|vertex| [vertex.x, vertex.y, vertex.z])
                .collect::<Vec<_>>(),
        ),
    );
    let indices = polyline
        .indices()
        .iter()
        .flat_map(|triangle| [triangle[0], triangle[1], triangle[0]])
        .collect();

    mesh.set_indices(Some(Indices::U32(indices)));
    mesh
}

#[cfg(feature = "dim2")]
pub fn wire_polyline(polyline: &Polyline, config: &RapierConfiguration) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.set_attribute(
        Mesh::ATTRIBUTE_POSITION,
        VertexAttributeValues::from(
            polyline
                .vertices()
                .iter()
                .map(|vertex| [vertex.x, vertex.y])
                .collect::<Vec<_>>(),
        ),
    );
    let indices = polyline
        .indices()
        .iter()
        .flat_map(|triangle| [triangle[0], triangle[1], triangle[0]])
        .collect();

    mesh.set_indices(Some(Indices::U32(indices)));
    mesh
}
