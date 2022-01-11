use crate::prelude::*;
use bevy::prelude::*;

pub fn wire_capsule(capsule: &Capsule) -> Vec<Vec3> {
    let radius = capsule.radius;
    let height = capsule.half_height();

    let mut vertices = Vec::new();
    // Start with rings around the segment ends.
    let segment_a = capsule.segment.a;
    let segment_b = capsule.segment.b;

    vertices.push(Vec3::new(segment_a[0], segment_a[1], segment_a[2]));
    vertices.push(Vec3::new(segment_b[0], segment_b[1], segment_b[2]));
    vertices
}
