use bevy::prelude::*;
use bevy_inspector_egui::{WorldInspectorPlugin, widgets::ResourceInspector, Inspectable, InspectorPlugin, RegisterInspectable};
use bevy_inspector_egui_rapier::InspectableRapierPlugin;
use  bevy_rapier3d::render::RapierDebugRenderPlugin;

use crate::{EnemyConfiguration, FocusTarget, Health};

pub struct DebugPlugin;

#[derive(Inspectable, Default)]
struct Data {
    enemy_configuration: ResourceInspector<EnemyConfiguration>,
}

impl Plugin for DebugPlugin {

    fn build(&self, app: &mut App) {
        if cfg!(debug_assertions) {
            app.add_plugin(RapierDebugRenderPlugin::default());
            app.add_plugin(WorldInspectorPlugin::new());
            app.add_plugin(InspectableRapierPlugin);
            app.add_plugin(InspectorPlugin::<Data>::new());
            app.register_inspectable::<FocusTarget>();
            app.register_inspectable::<Health>();
        }
    }

}