use bevy::prelude::*;

/// Represents the size of the position gizmo.
/// **NOTE**: This is intended for internal/advanced use only.
pub struct RapierDebugPositionSize(pub f32);

impl Default for RapierDebugPositionSize {
    fn default() -> RapierDebugPositionSize {
        #[cfg(feature = "dim3")]
        return RapierDebugPositionSize(0.1);
        #[cfg(feature = "dim2")]
        RapierDebugPositionSize(10.0)
    }
}
/*
/// Position Bundle using the correct material to display a position.
/// **NOTE**: This is intended for internal/advanced use only.
#[derive(Bundle)]
pub struct RapierDebugPositionBundle {
    pub polyline: Handle<Polyline>,
    pub material: Handle<PolylineMaterial>,
    pub size: RapierDebugPositionSize,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

impl Default for RapierDebugPositionBundle {
    fn default() -> Self {
        Self {
            polyline: Default::default(),
            size: Default::default(),
            material: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
        }
    }
}
 */
