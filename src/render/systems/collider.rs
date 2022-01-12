use crate::prelude::*;
use crate::render::prelude::*;
use bevy::prelude::*;
use bevy_stylized_wireframe::prelude::*;
//use bevy_mesh::{Polyline, PolylineBundle, PolylineMaterial};

pub struct DebugColliderMaterial {
    pub wireframe: bool,
}

/// Recreate added/changed collider shapes/types/etc.
pub fn spawn_debug_colliders(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    config: Res<RapierConfiguration>,
    mut meshes: ResMut<Assets<Mesh>>,
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
            if let Some(collider_meshs) = collider_to_mesh(shape, &config) {
                for mut found in collider_meshs {
                    // We duplicate vertices and remove indices so there
                    // aren't any shared vertices that would mess with
                    // barycentric coordinates.
                    found.mesh.duplicate_vertices();
                    found.mesh.compute_barycentric();

                    parent
                        .spawn()
                        .insert(Name::new("Debug Collider"))
                        .insert(RapierDebugRenderCollider)
                        .insert(SimpleWireframe::default())
                        //.insert(StylizedWireframe::default())
                        //.insert(ColoredWireframe)
                        .insert(meshes.add(found.mesh))
                        .insert(transform)
                        .insert(GlobalTransform::from(transform))
                        .insert(Visibility::default())
                        .insert(ComputedVisibility::default());
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
    mesh: Mesh,
}

impl ColliderFound {
    pub fn from(mesh: Mesh) -> ColliderFound {
        Self {
            isometry: None,
            mesh: mesh,
        }
    }
}

fn collider_to_mesh(
    shape: &ColliderShape,
    config: &RapierConfiguration,
) -> Option<Vec<ColliderFound>> {
    let mut found = Vec::new();
    match shape.shape_type() {
        ShapeType::Capsule => {
            let capsule = shape.as_capsule().unwrap();
            found.push(ColliderFound::from(crate::render::mesh::wire_capsule(
                capsule, config,
            )));
        }
        ShapeType::Cuboid => {
            let cuboid = shape.as_cuboid().unwrap();
            found.push(ColliderFound::from(crate::render::mesh::wire_cube(
                cuboid, config,
            )));
        }
        ShapeType::Ball => {
            let ball = shape.as_ball().unwrap();
            found.push(ColliderFound::from(crate::render::mesh::wire_sphere(
                ball.radius,
                config,
            )));
        }
        ShapeType::TriMesh => {
            let trimesh = shape.as_trimesh().unwrap();
            found.push(ColliderFound::from(crate::render::mesh::wire_trimesh(
                trimesh, config,
            )));
        }
        ShapeType::Polyline => {
            let polyline = shape.as_polyline().unwrap();
            found.push(ColliderFound::from(crate::render::mesh::wire_polyline(
                polyline, config,
            )));
        }
        ShapeType::Segment => {
            let segment = shape.as_segment().unwrap();
            found.push(ColliderFound::from(crate::render::mesh::wire_segment(
                segment, config,
            )));
        }
        #[cfg(feature = "dim3")]
        ShapeType::Cylinder => {
            let cylinder = shape.as_cylinder().unwrap();
            found.push(ColliderFound::from(crate::render::mesh::wire_cylinder(
                cylinder, config,
            )));
        }
        #[cfg(feature = "dim3")]
        ShapeType::ConvexPolyhedron => {
            let convex_mesh = shape.as_convex_polyhedron().unwrap();
            found.push(ColliderFound::from(crate::render::mesh::wire_convex_mesh(
                convex_mesh,
                config,
            )));
        }
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
        }
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
