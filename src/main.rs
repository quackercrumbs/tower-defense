use bevy::prelude::*;
use bevy_flycam::PlayerPlugin;
use bevy_rapier3d::prelude::*;

fn main() {
   App::new()
       .add_plugins(DefaultPlugins)
       .add_plugin(PlayerPlugin)
       .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
       .add_plugin(RapierRenderPlugin)
       .add_startup_system(setup_physics_demo)
       .add_system(print_ball_altitude)
       .run();
}

// setup rapier demo scene
fn setup_physics_demo(
   mut commands: Commands,
   mut meshes: ResMut<Assets<Mesh>>,
   mut materials: ResMut<Assets<StandardMaterial>>,
) {
   info!("setup physics");

   const HALF_SIZE: f32 = 100.0;

   commands.spawn_bundle(DirectionalLightBundle {
       directional_light: DirectionalLight {
           illuminance: 10000.0,
           // Configure the projection to better fit the scene
           shadow_projection: OrthographicProjection {
               left: -HALF_SIZE,
               right: HALF_SIZE,
               bottom: -HALF_SIZE,
               top: HALF_SIZE,
               near: -10.0 * HALF_SIZE,
               far: 100.0 * HALF_SIZE,
               ..Default::default()
           },
           shadows_enabled: true,
           ..Default::default()
       },
       transform: Transform {
           translation: Vec3::new(10.0, 2.0, 10.0),
           rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4),
           ..Default::default()
       },
       ..Default::default()
   });

   /* Create the ground. */
   /*
    * Ground
   */
   let ground_size = 200.1;
   let ground_height = 0.1;
   let collider = ColliderBundle {
       shape: ColliderShape::cuboid(ground_size, ground_height, ground_size).into(),
       position: Vec3::new(0.0, -ground_height, 0.0).into(),
       ..ColliderBundle::default()
   };
   commands
       .spawn_bundle(collider)
       .insert(ColliderDebugRender::default())
       .insert(ColliderPositionSync::Discrete);

   /* Create the bouncing ball. */
   let rigid_body = RigidBodyBundle {
       position: Vec3::new(2.0, 2.0, 0.0).into(),
       ..Default::default()
   };
   let collider = ColliderBundle {
       shape: ColliderShape::ball(0.5).into(),
       material: ColliderMaterial {
           restitution: 0.7,
           ..Default::default()
       }.into(),
       ..Default::default()
   };
   commands
       .spawn()
       .insert_bundle(rigid_body)
       .insert_bundle(collider)
       .insert(ColliderPositionSync::Discrete)
       .insert(ColliderDebugRender::with_id(2));

   // Create cube, Build the rigid body.
   let rad = 1.0;
   let color = 0;
   let rigid_body = RigidBodyBundle {
       position: Vec3::new(1., 1., 0.).into(),
       ..RigidBodyBundle::default()
   };
   let collider = ColliderBundle {
       shape: ColliderShape::cuboid(rad, rad, rad).into(),
       ..ColliderBundle::default()
   };
   commands
       .spawn()
       .insert_bundle(rigid_body)
       .insert_bundle(collider)
       // give it a mesh different from the collider shape
       .insert_bundle(PbrBundle {
           mesh: meshes.add(Mesh::from(bevy::prelude::shape::Icosphere {radius: 2., subdivisions: 10})),
           material: materials.add(Color::rgb(1.,0.,0.).into()),
           transform: Transform::from_xyz(1., 1., 1.),
           ..Default::default()
       })
       // commented out debug mesh from bevy_rapier
       .insert(ColliderDebugRender::with_id(color))
       .insert(ColliderPositionSync::Discrete);
}

fn print_ball_altitude(positions: Query<&RigidBodyPositionComponent>) {
   for rb_pos in positions.iter() {
       println!("Ball altitude: {}", rb_pos.position.translation.vector.y);
   }
}