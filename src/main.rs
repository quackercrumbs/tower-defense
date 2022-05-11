use bevy::prelude::*;
use bevy_flycam::PlayerPlugin;
use bevy_inspector_egui::Inspectable;
use bevy_rapier3d::prelude::*;
use std::ops::{Sub, Mul, Add};

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
        .insert_resource(EnemySpawnTimer(Timer::from_seconds(2.0, true)))
        .insert_resource(EnemyConfiguration{ max_count: 2, size: 0.25, speed: 8.0 })
        .add_system(spawn_enemies_interval)
        .add_system(move_enemies)
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
        .with_children(|parent| {
            parent
                .spawn()
                .insert_bundle(PbrBundle {
                    mesh: meshes.add(Mesh::from(bevy::prelude::shape::Plane {
                        size: ground_size * 2.,
                    })),
                    material: materials.add(Color::rgb(0.5, 0.5, 0.).into()),
                    transform: Transform::from_translation(Vec3::new(0.0, ground_height, 0.0)),
                    ..Default::default()
                });
        })
        .insert(ColliderPositionSync::Discrete);


    // Create castle, use cube
    let cube_size = 0.5;
    commands
        .spawn()
        .insert_bundle(RigidBodyBundle {
            position: Vec3::new(0., cube_size, 0.).into(),
            body_type: RigidBodyTypeComponent(RigidBodyType::Dynamic),
            ..RigidBodyBundle::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(cube_size, cube_size, cube_size).into(),
            ..ColliderBundle::default()
        })
        // uncomment to view collider shape (make sure that the dimension and shape matches the colliders shape!)
        // .insert_bundle(PbrBundle {
        //     mesh: meshes.add(Mesh::from(bevy::prelude::shape::Box {
        //         min_x: -cube_size, max_x: cube_size,
        //         min_y: -cube_size, max_y: cube_size,
        //         min_z: -cube_size, max_z: cube_size,
        //     })),
        //     material: materials.add(Color::rgb(0., 0.1, 0.2).into()),
        //     ..Default::default()
        // })
        .with_children(|parent| {
            parent.spawn().insert_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(bevy::prelude::shape::Box {
                    min_x: -cube_size,
                    max_x: cube_size,
                    min_y: -cube_size,
                    max_y: cube_size * 10.,
                    min_z: -cube_size,
                    max_z: cube_size,
                })),
                material: materials.add(Color::rgb(1., 0., 0.).into()),
                ..Default::default()
            });
        })
        .insert(ColliderPositionSync::Discrete);
}

struct EnemySpawnTimer(Timer);

#[derive(Inspectable)]
struct EnemyConfiguration {
    max_count: usize,
    size: f32,
    speed: f32,
}
#[derive(Component)]
struct Enemy;

fn spawn_enemies_interval(
    time: Res<Time>,
    enemy_config: Res<EnemyConfiguration>,
    enemies: Query<&Enemy>,
    mut enemy_timer: ResMut<EnemySpawnTimer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
) {
    if  enemies.iter().len() < enemy_config.max_count {
        if enemy_timer.0.tick(time.delta()).just_finished() {
            info!("Spawn enemy");
            commands.spawn()
                .insert(Enemy)
                .insert_bundle(ColliderBundle {
                    shape: ColliderShape::cuboid(enemy_config.size, enemy_config.size, enemy_config.size).into(),
                    // the location of where they spawn
                    // note: the `y` value matches half the height of the ground collider. might want to use some configuration to drive this?
                    position: Vec3::new(18.0, 0.25,16.0).into(),
                    ..Default::default()
                })
                // uncomment to view collider shape (make sure that the dimension and shape matches the colliders shape!)
                // .insert_bundle(PbrBundle {
                //     mesh: meshes.add(Mesh::from(bevy::prelude::shape::Box {
                //         min_x: -enemy_config.size, max_x: enemy_config.size,
                //         min_y: -enemy_config.size, max_y: enemy_config.size,
                //         min_z: -enemy_config.size, max_z: enemy_config.size,
                //     })),
                //     material: materials.add(Color::rgb(0., 0.1, 0.2).into()),
                //     ..Default::default()
                // })
                .with_children(|parent| {
                    parent.spawn().insert_bundle(PbrBundle {
                        mesh: meshes.add(Mesh::from(bevy::prelude::shape::Cube {
                            size: enemy_config.size * 2.0
                        })),
                        material: materials.add(Color::rgb(0.1, 0.1, 0.4).into()),
                        ..Default::default()
                    });
                })
                .insert(ColliderPositionSync::Discrete);
        }
    }
}

fn move_enemies(
    enemy_config: Res<EnemyConfiguration>,
    time: Res<Time>,
    mut enemies: Query<&mut ColliderPositionComponent, With<Enemy>>,
) {
    let target = nalgebra::Translation3::new(0., 0.25, 0.);
    enemies.for_each_mut(|mut enemy| {
        let distance_vector = target.vector.sub(enemy.translation.vector);
        let len = distance_vector.magnitude();
        if len >= 5. {
            let new_pos = distance_vector.normalize().mul(enemy_config.speed * time.delta_seconds());
            enemy.translation.vector = enemy.translation.vector.add(new_pos);
        }
    })
}