
use bevy::prelude::*;
use bevy::render::pipeline::PrimitiveTopology;
use bevy::render::mesh::{Indices, VertexAttributeValues};
use crate::prelude::*;

#[cfg(feature = "dim3")]
pub fn wire_convex_mesh(convex_mesh: &ConvexPolyhedron) -> Mesh {
    let (vertices, indices) = rapier::parry::transformation::convex_hull(convex_mesh.points());
    let mut mesh = Mesh::new(PrimitiveTopology::LineList);
    mesh.set_attribute(
        Mesh::ATTRIBUTE_POSITION,
        VertexAttributeValues::from(
            vertices
                .iter()
                .map(|vertex| [vertex.x, vertex.y, vertex.z])
                .collect::<Vec<_>>(),
        ),
    );
    let indices = indices
        .iter()
        .flat_map(|triangle| [triangle[0], triangle[1], triangle[2]])
        .collect();

    mesh.set_indices(Some(Indices::U32(indices)));
    mesh
}

#[cfg(feature = "dim2")]
pub fn wire_convex_mesh(convex_mesh: &ConvexPolygon) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::LineList);
    mesh.set_attribute(
        Mesh::ATTRIBUTE_POSITION,
        VertexAttributeValues::from(
            convex_mesh
                .points()
                .iter()
                .map(|vertex| [vertex.x, 0.0, vertex.y])
                .collect::<Vec<_>>(),
        ),
    );
    /*
    let indices = convex_mesh
        .indices()
        .iter()
        .flat_map(|triangle| [triangle[0], triangle[1], triangle[0]])
        .collect();
        */

    //mesh.set_indices(Some(Indices::U32(indices)));
    mesh
}