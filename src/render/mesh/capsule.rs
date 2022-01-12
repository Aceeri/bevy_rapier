use crate::prelude::*;
use bevy::prelude::*;

pub fn wire_capsule(capsule: &Capsule, config: &RapierConfiguration) -> Mesh {
    Mesh::from(bevy::prelude::shape::Capsule {
        radius: capsule.radius * config.scale,
        rings: 0,
        depth: capsule.half_height() / 4.0 * config.scale,
        latitudes: 8,
        longitudes: 18,
        uv_profile: bevy::prelude::shape::CapsuleUvProfile::Aspect,
    })
}
