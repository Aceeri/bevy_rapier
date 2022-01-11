use crate::prelude::*;
use crate::render::prelude::*;
use bevy::prelude::*;
use bevy_polyline::{Polyline, PolylineBundle, PolylineMaterial};

/// Recreate added/changed collider shapes/types/etc.
pub fn spawn_debug_colliders(
    mut commands: Commands,
    mut polyline_materials: ResMut<Assets<PolylineMaterial>>,
    config: Res<RapierConfiguration>,
    mut polylines: ResMut<Assets<Polyline>>,
    query: Query<
        (
            Entity,
            &ColliderShapeComponent,
            &ColliderTypeComponent,
            &RapierDebugCollider,
            &ColliderPositionComponent,
            Option<&RigidBodyPositionSync>,
            Option<&ColliderPositionSync>,
            Option<&RigidBodyPositionComponent>,
        ),
        (Or<(
            Changed<RapierDebugCollider>,
            Changed<ColliderShapeComponent>,
            Changed<ColliderTypeComponent>,
        )>),
    >,
) {
    for (entity, shape, ty, debug, co_pos, rb_sync, co_sync, rb_pos) in query.iter() {
        let transform = {
            if co_sync.is_some() {
                Transform::identity()
            } else if rb_sync.is_some() {
                if let Some(rb_pos) = rb_pos {
                    rigid_body_transform(rb_pos, co_pos)
                } else {
                    Default::default()
                }
            } else {
                collider_transform(co_pos)
            }
        };

        commands.entity(entity).with_children(|parent| {
            if let Some(collider_polylines) = collider_to_polyline(shape) {
                for collider_polyline in collider_polylines {
                    let scaled_vertices: Vec<Vec3> = collider_polyline
                        .vertices
                        .iter()
                        .map(|vertex| {
                            Vec3::new(
                                vertex.x * config.scale,
                                vertex.y * config.scale,
                                vertex.z * config.scale,
                            )
                        })
                        .collect();

                    parent
                        .spawn()
                        .insert(Name::new("Debug Collider"))
                        .insert(RapierDebugRenderCollider)
                        .insert_bundle(PolylineBundle {
                            polyline: polylines.add(Polyline {
                                vertices: scaled_vertices,
                            }),
                            material: polyline_materials.add(PolylineMaterial {
                                width: 100.0,
                                color: Color::RED,
                                perspective: true,
                                ..Default::default()
                            }),
                            ..Default::default()
                        })
                        .insert(transform)
                        .insert(GlobalTransform::from(transform));
                }
            }
        });
    }
}

#[cfg(feature = "dim3")]
fn collider_transform(co_pos: &ColliderPosition) -> Transform {
    let mut transform = Transform::from_translation(co_pos.translation.into());
    transform.rotation = co_pos.rotation.into();
    transform
}

#[cfg(feature = "dim2")]
fn collider_transform(co_pos: &ColliderPosition) -> Transform {
    Transform::from_xyz(co_pos.translation.x, co_pos.translation.y, 1.0)
}

#[cfg(feature = "dim3")]
fn rigid_body_transform(rb_pos: &RigidBodyPosition, co_pos: &ColliderPosition) -> Transform {
    let mut co_transform = Transform::from_translation(
        Vec3::from(co_pos.translation) - Vec3::from(rb_pos.position.translation),
    );
    co_transform.rotation = Quat::from(co_pos.rotation);
    co_transform
}

#[cfg(feature = "dim2")]
fn rigid_body_transform(rb_pos: &RigidBodyPosition, co_pos: &ColliderPosition) -> Transform {
    let pos = Vec2::from(co_pos.translation) - Vec2::from(rb_pos.position.translation);
    Transform::from_xyz(pos.x, pos.y, 1.0)
}

pub struct ColliderFound {
    isometry: Option<Isometry<Real>>,
    vertices: Vec<Vec3>,
}

impl ColliderFound {
    pub fn from(vertices: Vec<Vec3>) -> ColliderFound {
        Self {
            isometry: None,
            vertices: vertices,
        }
    }
}

fn collider_to_polyline(shape: &ColliderShape) -> Option<Vec<ColliderFound>> {
    let mut found = Vec::new();
    match shape.shape_type() {
        ShapeType::Capsule => {
            let capsule = shape.as_capsule().unwrap();
            found.push(ColliderFound::from(crate::render::mesh::wire_capsule(
                capsule,
            )));
        }
        ShapeType::Cuboid => {
            let cuboid = shape.as_cuboid().unwrap();
            found.push(ColliderFound::from(crate::render::mesh::wire_cube(cuboid)));
        }
        ShapeType::Ball => {
            let ball = shape.as_ball().unwrap();
            found.push(ColliderFound::from(crate::render::mesh::wire_sphere(ball.radius)));
        },
        /*
        ShapeType::TriMesh => {
            let trimesh = shape.as_trimesh().unwrap();
            found.push(ColliderFound::from(crate::render::mesh::wire_trimesh(trimesh)));
        },
        ShapeType::Polyline => {
            let polyline = shape.as_polyline().unwrap();
            found.push(ColliderFound::from(crate::render::mesh::wire_polyline(polyline)));
        },
        ShapeType::Segment => {
            let segment = shape.as_segment().unwrap();
            found.push(ColliderFound::from(crate::render::mesh::wire_segment(segment)));
        },
        #[cfg(feature = "dim3")]
        ShapeType::Cylinder => {
            let cylinder = shape.as_cylinder().unwrap();
            found.push(ColliderFound::from(crate::render::mesh::wire_cylinder(cylinder, config)));
        },
        #[cfg(feature = "dim3")]
        ShapeType::ConvexPolyhedron => {
            let convex_mesh = shape.as_convex_polyhedron().unwrap();
            found.push(ColliderFound::from(crate::render::mesh::wire_convex_mesh(convex_mesh)));
        },
        ShapeType::Compound => {
            let compound = shape.as_compound().unwrap();
            for (isometry, shape) in compound.shapes() {
                let recurse_found = collider_to_mesh(shape, config);
                if let Some(mut inner_found) = recurse_found {
                    for recurse in &mut inner_found {
                        recurse.isometry = match recurse.isometry {
                            Some(other) => Some(isometry * other),
                            None => Some(*isometry),
                        };
                    }

                    found.extend(inner_found);
                }
            }
        } */
        ty => {
            warn!("Unable to render collider shape type: {:?}", ty);
        }
    }

    if found.len() > 0 {
        Some(found)
    } else {
        None
    }
}
