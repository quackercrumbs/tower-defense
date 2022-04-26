use bevy::prelude::*;
use bevy_flycam::PlayerPlugin;
use bevy_rapier3d::prelude::*;

mod debug;
use debug::DebugPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(DebugPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierRenderPlugin)
        .add_startup_system(setup_world)
        // .add_system(print_collider_altitude)
        // .add_system(print_transform_altitude)
        .run();
}

// setup rapier demo scene
fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    info!("setup world");

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
    let ground_size = 20.1;
    let ground_height = 0.5;
    commands
        .spawn()
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(ground_size, ground_height, ground_size).into(),
            position: Vec3::new(0.0, -ground_height, 0.0).into(),
            ..ColliderBundle::default()
        })
        // .insert_bundle(PbrBundle {
        //     mesh: meshes.add(Mesh::from(bevy::prelude::shape::Cube {
        //         size: ground_size,
        //     })),
        //     material: materials.add(Color::rgb(0.5, 0.5, 0.).into()),
        //     ..Default::default()
        // })
        .insert(ColliderDebugRender::default())
        .insert(ColliderPositionSync::Discrete);

    // Create cube, Build the rigid body.
    let cube_size = 0.5;
    commands
        .spawn()
        .insert_bundle(RigidBodyBundle {
            position: Vec3::new(1., 3., 0.).into(),
            ..RigidBodyBundle::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(cube_size, cube_size, cube_size).into(),
            ..ColliderBundle::default()
        })
        // give it a mesh different from the collider shape
        .insert_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(bevy::prelude::shape::Cube {
                size: cube_size * 2.,
            })),
            material: materials.add(Color::rgb(1., 0., 0.).into()),
            ..Default::default()
        })
        // commented out debug mesh from bevy_rapier
        // .insert(ColliderDebugRender::with_id(color))
        .insert(ColliderPositionSync::Discrete);
}

fn print_collider_altitude(positions: Query<&ColliderPositionComponent>) {
    for rb_pos in positions.iter() {
        println!("Ball altitude: {}", rb_pos.translation.vector.y);
    }
}

fn print_transform_altitude(transforms: Query<&GlobalTransform>) {

    for t in transforms.iter() {
        println!("transform: {}", t.translation.y)
    }

}
