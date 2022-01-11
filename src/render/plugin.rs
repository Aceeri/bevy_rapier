use bevy::prelude::*;
use bevy_polyline::{Polyline, PolylinePlugin};

use crate::render::systems::spawn_debug_colliders;

/// Debug render plugin for rapier.
pub struct RapierDebugPlugin;

impl Plugin for RapierDebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<crate::render::RapierDebugToggleVisibility>();

        app.add_system(spawn_debug_colliders.label("rapier_debug_material_setup"));

        if !app.world.contains_resource::<Assets<Polyline>>() {
            app.add_plugin(PolylinePlugin);
        }
        //.add_asset::<crate::render::render::WireframeMaterial>()
        //.add_asset::<crate::render::render::PositionWireframeMaterial>()
        //.add_event::<crate::render::RapierDebugToggleRenderPass>()
        /*
        .add_startup_system(crate::render::render::setup_material.system().label("material_setup"))
        .add_startup_system(crate::render::render::setup_debug_pass.system().after("material_setup"))
        .add_system(crate::render::systems::spawn_debug_colliders.system())
        .add_system(crate::render::systems::spawn_debug_positions.system())
        .add_system(crate::render::systems::spawn_debug_paths.system())
        .add_system(crate::render::systems::update_path_mesh.system())
        .add_system(crate::render::systems::toggle_visibility.system())
        .add_system(crate::render::systems::toggle_render_pass.system())
        .add_system_to_stage(
            CoreStage::PostUpdate,
            asset_shader_defs_system::<crate::render::render::WireframeMaterial>.system(),
        );
        */
    }
}
