use super::{IntoEntity, IntoHandle};
use crate::physics::wrapper::{self, *};
use bevy::prelude::*;
use rapier::data::{ComponentSet, ComponentSetMut, ComponentSetOption, Index};
use rapier::dynamics;
impl IntoHandle<dynamics::RigidBodyHandle> for Entity {
    #[inline]
    fn handle(self) -> dynamics::RigidBodyHandle {
        dynamics::RigidBodyHandle::from_raw_parts(self.id(), self.generation())
    }
}

impl IntoEntity for dynamics::RigidBodyHandle {
    #[inline]
    fn entity(self) -> Entity {
        self.0.entity()
    }
}

pub type RigidBodyComponentsQueryPayload<'a> = (
    Entity,
    &'a mut RigidBodyPosition,
    &'a mut RigidBodyVelocity,
    &'a mut RigidBodyMassProps,
    &'a mut RigidBodyIds,
    &'a mut RigidBodyForces,
    &'a mut RigidBodyCcd,
    &'a mut RigidBodyColliders,
    &'a mut RigidBodyDamping,
    &'a mut RigidBodyDominance,
    &'a mut RigidBodyType,
    &'a mut RigidBodyChanges,
    &'a mut RigidBodyActivation,
);

pub type RigidBodyChangesQueryPayload<'a> = (
    Entity,
    &'a mut RigidBodyActivation,
    &'a mut RigidBodyChanges,
    Or<(Changed<RigidBodyPosition>, Added<RigidBodyPosition>)>,
    Or<(Changed<RigidBodyType>, Added<RigidBodyType>)>,
    Or<(Changed<RigidBodyColliders>, Added<RigidBodyColliders>)>,
);

pub type RigidBodyChangesQueryFilter = (
    Or<(
        Changed<RigidBodyPosition>,
        Added<RigidBodyPosition>,
        Changed<RigidBodyVelocity>,
        Added<RigidBodyVelocity>,
        Changed<RigidBodyForces>,
        Added<RigidBodyForces>,
        Changed<RigidBodyActivation>,
        Added<RigidBodyActivation>,
        Changed<RigidBodyType>,
        Added<RigidBodyType>,
        Changed<RigidBodyColliders>,
        Added<RigidBodyColliders>,
    )>,
);

pub type RigidBodyComponentsQuerySet<'world, 'state, 'a> = QuerySet<
    'world,
    'state,
    (
        // Components query
        QueryState<RigidBodyComponentsQueryPayload<'a>>,
        // Changes query
        QueryState<RigidBodyChangesQueryPayload<'a>, RigidBodyChangesQueryFilter>,
    ),
>;

pub struct RigidBodyComponentsSet<'world, 'state, 'a>(
    pub Query<'world, 'state, RigidBodyComponentsQueryPayload<'a>>,
);

impl_component_set_mut!(
    RigidBodyComponentsSet,
    dynamics::RigidBodyPosition,
    wrapper::RigidBodyPosition,
    |data| &*data.1
);
impl_component_set_mut!(
    RigidBodyComponentsSet,
    dynamics::RigidBodyVelocity,
    wrapper::RigidBodyVelocity,
    |data| &*data.2
);
impl_component_set_mut!(
    RigidBodyComponentsSet,
    dynamics::RigidBodyMassProps,
    wrapper::RigidBodyMassProps,
    |data| &*data.3
);
impl_component_set_mut!(
    RigidBodyComponentsSet,
    dynamics::RigidBodyIds,
    wrapper::RigidBodyIds,
    |data| &*data.4
);
impl_component_set_mut!(
    RigidBodyComponentsSet,
    dynamics::RigidBodyForces,
    wrapper::RigidBodyForces,
    |data| &*data.5
);
impl_component_set_mut!(
    RigidBodyComponentsSet,
    dynamics::RigidBodyCcd,
    wrapper::RigidBodyCcd,
    |data| &*data.6
);
impl_component_set_mut!(
    RigidBodyComponentsSet,
    dynamics::RigidBodyColliders,
    wrapper::RigidBodyColliders,
    |data| &*data.7
);
impl_component_set_mut!(
    RigidBodyComponentsSet,
    dynamics::RigidBodyDamping,
    wrapper::RigidBodyDamping,
    |data| &*data.8
);
impl_component_set_mut!(
    RigidBodyComponentsSet,
    dynamics::RigidBodyDominance,
    wrapper::RigidBodyDominance,
    |data| &*data.9
);
impl_component_set_mut!(
    RigidBodyComponentsSet,
    dynamics::RigidBodyType,
    wrapper::RigidBodyType,
    |data| &*data.10
);
impl_component_set_mut!(
    RigidBodyComponentsSet,
    dynamics::RigidBodyChanges,
    wrapper::RigidBodyChanges,
    |data| &*data.11
);
impl_component_set_mut!(
    RigidBodyComponentsSet,
    dynamics::RigidBodyActivation,
    wrapper::RigidBodyActivation,
    |data| &*data.12
);

#[derive(Bundle)]
pub struct RigidBodyBundle {
    pub body_type: wrapper::RigidBodyType,
    pub position: wrapper::RigidBodyPosition,
    pub velocity: wrapper::RigidBodyVelocity,
    pub mass_properties: wrapper::RigidBodyMassProps,
    pub forces: wrapper::RigidBodyForces,
    pub activation: wrapper::RigidBodyActivation,
    pub damping: wrapper::RigidBodyDamping,
    pub dominance: wrapper::RigidBodyDominance,
    pub ccd: wrapper::RigidBodyCcd,
    pub changes: wrapper::RigidBodyChanges,
    pub ids: wrapper::RigidBodyIds,
    pub colliders: wrapper::RigidBodyColliders,
}

impl Default for RigidBodyBundle {
    fn default() -> Self {
        Self {
            body_type: wrapper::Comp(rapier::prelude::RigidBodyType::Dynamic),
            position: wrapper::RigidBodyPosition::default(),
            velocity: wrapper::RigidBodyVelocity::default(),
            mass_properties: wrapper::RigidBodyMassProps::default(),
            forces: wrapper::RigidBodyForces::default(),
            activation: wrapper::RigidBodyActivation::default(),
            damping: wrapper::RigidBodyDamping::default(),
            dominance: wrapper::RigidBodyDominance::default(),
            ccd: wrapper::RigidBodyCcd::default(),
            changes: wrapper::RigidBodyChanges::default(),
            ids: wrapper::RigidBodyIds::default(),
            colliders: wrapper::RigidBodyColliders::default(),
        }
    }
}
