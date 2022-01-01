use super::{IntoEntity, IntoHandle};
use bevy::prelude::*;
use rapier::data::{ComponentSet, ComponentSetMut, ComponentSetOption, Index};
use crate::physics::wrapper::{self, *};
use rapier::geometry;
impl IntoHandle<geometry::ColliderHandle> for Entity {
    #[inline]
    fn handle(self) -> geometry::ColliderHandle {
      geometry::ColliderHandle::from_raw_parts(self.id(), self.generation())
    }
}

impl IntoEntity for geometry::ColliderHandle {
    #[inline]
    fn entity(self) -> Entity {
        self.0.entity()
    }
}

pub type QueryPipelineColliderComponentsQuery<'world, 'state, 'a> = Query<
    'world,
    'state,
    (
        Entity,
        &'a ColliderPosition,
        &'a ColliderShape,
        &'a ColliderFlags,
    ),
>;

pub struct QueryPipelineColliderComponentsSet<'world, 'state, 'a, 'c>(
    pub &'c QueryPipelineColliderComponentsQuery<'world, 'state, 'a>,
);

impl_component_set!(
    QueryPipelineColliderComponentsSet,
    ColliderPosition,
    |data| data.1
);
impl_component_set!(QueryPipelineColliderComponentsSet, ColliderShape, |data| {
    data.2
});
impl_component_set!(QueryPipelineColliderComponentsSet, ColliderFlags, |data| {
    data.3
});

pub struct ColliderComponentsSet<'world, 'state, 'a>(
    pub Query<'world, 'state, ColliderComponentsQueryPayload<'a>>,
);

pub type ColliderComponentsQueryPayload<'a> = (
    Entity,
    &'a mut ColliderChanges,
    &'a mut ColliderPosition,
    &'a mut ColliderBroadPhaseData,
    &'a mut ColliderShape,
    &'a mut ColliderType,
    &'a mut ColliderMaterial,
    &'a mut ColliderFlags,
    Option<&'a ColliderParent>,
);

pub type ColliderChangesQueryPayload<'a> = (
    Entity,
    &'a mut ColliderChanges,
    Or<(Changed<ColliderPosition>, Added<ColliderPosition>)>,
    Or<(Changed<ColliderFlags>, Added<ColliderFlags>)>,
    Or<(Changed<ColliderShape>, Added<ColliderShape>)>,
    Or<(Changed<ColliderType>, Added<ColliderType>)>,
    Option<Or<(Changed<ColliderParent>, Added<ColliderParent>)>>,
);

pub type ColliderChangesQueryFilter = (
    Or<(
        Changed<ColliderPosition>,
        Added<ColliderPosition>,
        Changed<ColliderFlags>,
        Added<ColliderFlags>,
        Changed<ColliderShape>,
        Added<ColliderShape>,
        Changed<ColliderType>,
        Added<ColliderType>,
        Changed<ColliderParent>,
        Added<ColliderParent>,
    )>,
);

pub type ColliderComponentsQuerySet<'world, 'state, 'a> = QuerySet<
    'world,
    'state,
    (
        // Components query
        QueryState<ColliderComponentsQueryPayload<'a>>,
        // Changes query
        QueryState<ColliderChangesQueryPayload<'a>, ColliderChangesQueryFilter>,
    ),
>;

impl_component_set_mut!(ColliderComponentsSet, geometry::ColliderChanges, wrapper::ColliderChanges, |data| &**data.1);
impl_component_set_mut!(ColliderComponentsSet, geometry::ColliderPosition, wrapper::ColliderPosition, |data| &*data.2);
impl_component_set_mut!(ColliderComponentsSet, geometry::ColliderBroadPhaseData, wrapper::ColliderBroadPhaseData,|d| &*d.3);
impl_component_set_mut!(ColliderComponentsSet, geometry::ColliderShape, wrapper::ColliderShape, |data| &*data.4);
impl_component_set_mut!(ColliderComponentsSet, geometry::ColliderType, wrapper::ColliderType, |data| &*data.5);
impl_component_set_mut!(ColliderComponentsSet, geometry::ColliderMaterial, wrapper::ColliderMaterial, |data| &*data.6);
impl_component_set_mut!(ColliderComponentsSet, geometry::ColliderFlags, wrapper::ColliderFlags, |data| &*data.7);
impl_component_set_option!(ColliderComponentsSet, geometry::ColliderParent, wrapper::ColliderParent);

#[derive(Bundle)]
pub struct ColliderBundle {
    pub collider_type: wrapper::ColliderType,
    pub shape: wrapper::ColliderShape,
    pub position: wrapper::ColliderPosition,
    pub material: wrapper::ColliderMaterial,
    pub flags: wrapper::ColliderFlags,
    pub mass_properties: wrapper::ColliderMassProps,
    pub changes: wrapper::ColliderChanges,
    pub bf_data: wrapper::ColliderBroadPhaseData,
}

impl Default for ColliderBundle {
    fn default() -> Self {
        Self {
            collider_type: wrapper::Comp(geometry::ColliderType::Solid),
            shape: wrapper::Comp(geometry::ColliderShape::ball(0.5)),
            position: ColliderPosition::default(),
            material: ColliderMaterial::default(),
            flags: ColliderFlags::default(),
            mass_properties: ColliderMassProps::default(),
            changes: ColliderChanges::default(),
            bf_data: ColliderBroadPhaseData::default(),
        }
    }
}
