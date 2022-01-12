use crate::prelude::*;
use bevy::prelude::*;
use bevy::render::mesh::{Indices, VertexAttributeValues};
use wgpu_types::PrimitiveTopology;

#[cfg(feature = "dim3")]
pub fn wire_trimesh(trimesh: &TriMesh, config: &RapierConfiguration) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.set_attribute(
        Mesh::ATTRIBUTE_POSITION,
        VertexAttributeValues::from(
            trimesh
                .vertices()
                .iter()
                .map(|vertex| [vertex.x, vertex.y, vertex.z])
                .collect::<Vec<_>>(),
        ),
    );
    let indices = trimesh
        .indices()
        .iter()
        .flat_map(|triangle| [triangle[0], triangle[1], triangle[2]])
        .collect();

    mesh.set_indices(Some(Indices::U32(indices)));
    mesh
}

#[cfg(feature = "dim2")]
pub fn wire_trimesh(trimesh: &TriMesh, config: &RapierConfiguration) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.set_attribute(
        Mesh::ATTRIBUTE_POSITION,
        VertexAttributeValues::from(
            trimesh
                .vertices()
                .iter()
                .map(|vertex| [vertex.x, vertex.y])
                .collect::<Vec<_>>(),
        ),
    );
    let indicies = trimesh
        .indices()
        .iter()
        .flat_map(|triangle| [triangle[0], triangle[1]])
        .collect();

    mesh.set_indices(Some(Indices::U32(indicies)));
    mesh
}
