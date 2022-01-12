use crate::prelude::*;
use bevy::prelude::*;
use wgpu_types::PrimitiveTopology;

#[cfg(feature = "dim3")]
pub fn wire_cube(cuboid: &Cuboid, config: &RapierConfiguration) -> Mesh {
    Mesh::from(bevy::prelude::shape::Box::new(
        cuboid.half_extents.x * config.scale * 2.0,
        cuboid.half_extents.y * config.scale * 2.0,
        cuboid.half_extents.z * config.scale * 2.0,
    ))
}

#[cfg(feature = "dim2")]
pub fn wire_cube(cuboid: &Cuboid, config: &RapierConfiguration) -> Mesh {
    Mesh::from(bevy::prelude::shape::Quad::new(Vec2::new(
        cuboid.half_extents.x * config.scale * 2.0,
        cuboid.half_extents.y * config.scale * 2.0,
    )))
}
