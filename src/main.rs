use bevy::prelude::*;
use bevy_flycam::PlayerPlugin;
fn main() {
   App::new()
       .add_plugins(DefaultPlugins)
       .add_plugin(PlayerPlugin)
       .add_startup_system(setup)
       .run();
}


/// set up a simple 3D scene
fn setup(
   mut commands: Commands,
   mut meshes: ResMut<Assets<Mesh>>,
   mut materials: ResMut<Assets<StandardMaterial>>,
) {
   // plane
   commands.spawn_bundle(PbrBundle {
       mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
       material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
       ..Default::default()
   });
}