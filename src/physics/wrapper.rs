use bevy::prelude::*;
use std::ops::{Deref, DerefMut};
use std::hash::{Hash, Hasher};
use rapier::{dynamics, geometry};

#[derive(Component)]
pub struct Comp<T>(pub T);

impl<T> From<T> for Comp<T> {
    fn from(t: T) -> Self {
        Comp(t)
    }
}

impl<T: Default> Default for Comp<T>{
    fn default()->Self{
        Comp::<T>(<T>::default())
    }
}

impl<T> Deref for Comp<T>{
    type Target = T;
    fn deref(&self)->&Self::Target{
        &self.0
    }
}

impl<T> DerefMut for Comp<T>{
    fn deref_mut(&mut self)->&mut Self::Target{
        &mut self.0
    }
}

impl<T: Hash> Hash for Comp<T>{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<T: PartialEq> PartialEq for Comp<T>{
    fn eq(&self,other:&Self) -> bool{
        self.0 == other.0
    }
}

impl<T: Eq> Eq for Comp<T> {}

pub type RigidBodyHandle = Comp<dynamics::RigidBodyHandle>;
pub type RigidBodyType = Comp<dynamics::RigidBodyType>;
pub type RigidBodyChanges = Comp<dynamics::RigidBodyChanges>;
pub type RigidBodyPosition = Comp<dynamics::RigidBodyPosition>;
pub type RigidBodyMassProps = Comp<dynamics::RigidBodyMassProps>;
pub type RigidBodyVelocity = Comp<dynamics::RigidBodyVelocity>;
pub type RigidBodyDamping = Comp<dynamics::RigidBodyDamping>;
pub type RigidBodyForces = Comp<dynamics::RigidBodyForces>;
pub type RigidBodyCcd = Comp<dynamics::RigidBodyCcd>;
pub type RigidBodyIds = Comp<dynamics::RigidBodyIds>;
pub type RigidBodyDominance = Comp<dynamics::RigidBodyDominance>;
pub type RigidBodyActivation = Comp<dynamics::RigidBodyActivation>;
pub type RigidBodyColliders = Comp<dynamics::RigidBodyColliders>;
pub type ColliderHandle = Comp<geometry::ColliderHandle>;
pub type ColliderChanges = Comp<geometry::ColliderChanges>;
pub type ColliderType = Comp<geometry::ColliderType>;
pub type ColliderBroadPhaseData = Comp<geometry::ColliderBroadPhaseData>;
pub type ColliderParent = Comp<geometry::ColliderParent>;
pub type ColliderMassProps = Comp<geometry::ColliderMassProps>;
pub type ColliderPosition = Comp<geometry::ColliderPosition>;
pub type ColliderMaterial = Comp<geometry::ColliderMaterial>;
pub type ColliderFlags = Comp<geometry::ColliderFlags>;
pub type ColliderShape = Comp<geometry::ColliderShape>;

/*
impl_component_wrapper_nd!(RigidBodyHandle, dynamics::RigidBodyHandle);
impl_component_wrapper_hash!(RigidBodyHandle);
impl_component_wrapper_nd!(RigidBodyType, dynamics::RigidBodyType);
impl_component_wrapper!(RigidBodyChanges, dynamics::RigidBodyChanges);
impl_component_wrapper!(RigidBodyPosition, dynamics::RigidBodyPosition);
impl_component_wrapper!(RigidBodyMassProps, dynamics::RigidBodyMassProps);
impl_component_wrapper!(RigidBodyVelocity, dynamics::RigidBodyVelocity);
impl_component_wrapper!(RigidBodyDamping, dynamics::RigidBodyDamping);
impl_component_wrapper!(RigidBodyForces, dynamics::RigidBodyForces);
impl_component_wrapper!(RigidBodyCcd, dynamics::RigidBodyCcd);
impl_component_wrapper!(RigidBodyIds, dynamics::RigidBodyIds);
impl_component_wrapper_hash!(RigidBodyIds);
impl_component_wrapper!(RigidBodyDominance, dynamics::RigidBodyDominance);
impl_component_wrapper_hash!(RigidBodyDominance);
impl_component_wrapper!(RigidBodyActivation, dynamics::RigidBodyActivation);
impl_component_wrapper!(RigidBodyColliders, dynamics::RigidBodyColliders);
impl_component_wrapper_nd!(ColliderHandle, geometry::ColliderHandle);
impl_component_wrapper_hash!(ColliderHandle);
impl_component_wrapper!(ColliderChanges, geometry::ColliderChanges);
impl_component_wrapper_nd!(ColliderType, geometry::ColliderType);
impl_component_wrapper!(ColliderBroadPhaseData, geometry::ColliderBroadPhaseData);
impl_component_wrapper_hash!(ColliderBroadPhaseData);
impl_component_wrapper!(ColliderMassProps, geometry::ColliderMassProps);
impl_component_wrapper_nd!(ColliderParent, geometry::ColliderParent);
impl_component_wrapper!(ColliderPosition, geometry::ColliderPosition);
impl_component_wrapper!(ColliderMaterial, geometry::ColliderMaterial);
impl_component_wrapper!(ColliderFlags, geometry::ColliderFlags);
impl_component_wrapper_hash!(ColliderFlags);
impl_component_wrapper_nd!(ColliderShape, geometry::ColliderShape);
*/