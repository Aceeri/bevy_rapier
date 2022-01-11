use bevy::prelude::*;

#[cfg(feature = "dim3")]
pub fn wire_sphere(radius: f32) -> Vec<Vec3> {
    let sphere = bevy::prelude::shape::Icosphere { radius, subdivisions: 1 };
    Vec::new()
}

#[cfg(feature = "dim2")]
pub fn wire_sphere(radius: f32) -> Mesh {
    let mut positions: Vec<[f32; 3]> = Vec::with_capacity(32 * 3);
    let mut indices: Vec<u16> = Vec::with_capacity(32 * 3 * 2);

    for i in 0..32u16 {
        let t = (i as f32) * (2.0 / 32.0);
        let (y, x) = f32::sin_cos(t * std::f32::consts::PI);
        positions.push([x*radius, y*radius, 0.0]);
        if i < 31 {
            indices.push(i);
            indices.push(i + 1);
        }
    }
    indices.push(31);
    indices.push(0);
    let mut mesh = Mesh::new(PrimitiveTopology::LineList);
    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.set_indices(Some(bevy::render::mesh::Indices::U16(indices)));
    mesh
}
