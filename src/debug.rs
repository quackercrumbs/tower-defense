use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_inspector_egui_rapier::InspectableRapierPlugin;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {

    fn build(&self, app: &mut App) {
        if cfg!(debug_assertions) {
            app.add_plugin(WorldInspectorPlugin::new());
            app.add_plugin(InspectableRapierPlugin);
        }
    }

}