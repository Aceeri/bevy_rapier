use bevy::prelude::*;
use bevy::render::shader::asset_shader_defs_system;

/// Debug render plugin for rapier.
pub struct RapierDebugPlugin;

impl Plugin for RapierDebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<crate::render::render::WireframeMaterial>()
            .add_asset::<crate::render::render::PositionWireframeMaterial>()
            .add_event::<crate::render::RapierDebugToggleVisibility>()
            .add_event::<crate::render::RapierDebugToggleRenderPass>()
            .add_startup_system(crate::render::render::setup_material.label("material_setup"))
            .add_startup_system(crate::render::render::setup_debug_pass.after("material_setup"))
            .add_system(crate::render::systems::spawn_debug_colliders)
            .add_system(crate::render::systems::spawn_debug_positions)
            .add_system(crate::render::systems::spawn_debug_paths)
            .add_system(crate::render::systems::update_path_mesh)
            .add_system(crate::render::systems::toggle_visibility)
            .add_system(crate::render::systems::toggle_render_pass)
            .add_system_to_stage(
                CoreStage::PostUpdate,
                asset_shader_defs_system::<crate::render::render::WireframeMaterial>.system(),
            );

    }
}
