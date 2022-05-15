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
        .add_startup_system(setup_world)
        .insert_resource(EnemySpawnTimer(Timer::from_seconds(2.0, true)))
        .insert_resource(EnemyConfiguration{ max_count: 1, size: 0.25, speed: 3.0, distance_from_target: 1.5 })
        .add_system(spawn_enemies_interval)
        .add_system(move_enemies)
        .add_system(test_collider_active_events)
        .run();
}

#[derive(Component)]
struct Ground;

#[derive(Component)]
struct Tower;

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
        .insert(Ground)
        .insert(Transform::from_xyz(0.0, -ground_height, 0.0))
        .insert_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(bevy::prelude::shape::Plane {
                size: ground_size * 2.,
            })),
            material: materials.add(Color::rgb(0.5, 0.5, 0.).into()),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn()
            .insert(Collider::cuboid(ground_size, ground_height, ground_size))
            .insert(Transform::from_xyz(0.0, -ground_height, 0.0));
        });

    // Create castle, use cube
    let cube_size = 0.5;
    commands
        .spawn()
        .insert(Tower)
        .insert_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(bevy::prelude::shape::Box {
                min_x: -cube_size,
                max_x: cube_size,
                min_y: -cube_size,
                max_y: cube_size * 10.,
                min_z: -cube_size,
                max_z: cube_size,
            })),
            material: materials.add(Color::rgb(1., 0., 0.).into()),
            transform: Transform::from_xyz(0.0, cube_size, 0.0),
            ..Default::default()
        })
        // probably don't need this collider
        .insert(Collider::cuboid(cube_size, cube_size, cube_size))
        .with_children(|parent| {
            // sensor range
            let sensor_range = cube_size * 10.0;
            parent
            .spawn()
            // NOTE: there is an issue where the Global Transform isn't being used by rapier (https://github.com/dimforge/bevy_rapier/issues/172)
            // So that means, if we move the parent transform around, the collider won't get syncd because it's only using the Transform (the child one)
            // For now, we'll just hard code it at the origin. If we want to test / make updates, we'll have to update the Transform (the child one)
            .insert_bundle((Transform::from_xyz(0.0, sensor_range, 0.0), GlobalTransform::default()))
            .insert(Collider::cuboid(sensor_range, sensor_range, sensor_range))
            .insert(ActiveEvents::COLLISION_EVENTS)
            .insert(ActiveCollisionTypes::STATIC_STATIC)

            .insert(Sensor(true));
        });
}

struct EnemySpawnTimer(Timer);

#[derive(Inspectable)]
struct EnemyConfiguration {
    max_count: usize,
    size: f32,
    speed: f32,
    distance_from_target: f32,
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
                .insert(Collider::cuboid(enemy_config.size, enemy_config.size, enemy_config.size))
                .insert_bundle(PbrBundle {
                    mesh: meshes.add(Mesh::from(bevy::prelude::shape::Cube {
                        size: enemy_config.size * 2.0
                    })),
                    material: materials.add(Color::rgb(0.1, 0.1, 0.4).into()),
                    transform: Transform::from_xyz(18.0, enemy_config.size, 16.0),
                    ..Default::default()
                });
        }
    }
}

fn move_enemies(
    enemy_config: Res<EnemyConfiguration>,
    time: Res<Time>,
    mut enemies: Query<&mut Transform, With<Enemy>>,
) {
    let target = Vec3::new(0., 0.25, 0.);
    enemies.for_each_mut(|mut enemy| {
        let distance_vector = target.sub(enemy.translation);
        let len = distance_vector.length();
        if len >= enemy_config.distance_from_target {
            let new_pos = distance_vector.normalize().mul(enemy_config.speed * time.delta_seconds());
            enemy.translation = enemy.translation.add(new_pos);
        }
    })
}

fn test_collider_active_events(
    mut collision_events: EventReader<CollisionEvent>,
) {
    for collision_event in collision_events.iter() {
        println!("Recieved collision event: {:?}", collision_event);
    }
}