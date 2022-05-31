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
        .add_system(enemy_check_for_focus_target)
        .add_system(tower_check_for_new_focus_target)
        .add_system(tower_attack_focus_target)
        .add_system(remove_the_dead)
        .run();
}

#[derive(Component, Inspectable)]
struct Health(i64);
#[derive(Component)]
struct Ground;

#[derive(Component)]
struct Tower;

#[derive(Component)]
struct TowerWeaponTimer(Timer);

#[derive(Component, Inspectable)]
struct FocusTarget(Option<Entity>);

#[derive(Component)]
struct Dead;

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
        });

    // Create castle, use cube
    let cube_size = 0.5;
    let sensor_range = cube_size * 10.0;
    commands
        .spawn()
        .insert(Tower)
        .insert_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(bevy::prelude::shape::Box {
                min_x: -cube_size,
                max_x: cube_size,
                min_y: -cube_size,
                max_y: cube_size * 5.,
                min_z: -cube_size,
                max_z: cube_size,
            })),
            material: materials.add(Color::rgb(1., 0., 0.).into()),
            transform: Transform::from_xyz(0.0, cube_size, 0.0),
            ..Default::default()
        })
        .insert(TowerWeaponTimer(Timer::from_seconds(0.5, true)))
        .insert(FocusTarget(None))
        .insert(CollidingEntities::default())
        .insert(Collider::cuboid(sensor_range, sensor_range, sensor_range))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(ActiveCollisionTypes::STATIC_STATIC);
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
                .insert(Health(100))
                .insert(Collider::cuboid(enemy_config.size, enemy_config.size, enemy_config.size))
                .insert(FocusTarget(None))
                .insert(CollidingEntities::default())
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

fn tower_attack_focus_target(
    mut tower: Query<(&FocusTarget, &mut TowerWeaponTimer), (With<Tower>, Without<Dead>)>,
    mut enemy: Query<&mut Health, (With<Enemy>, Without<Dead>)>,
    mut commands: Commands,
    time: Res<Time>,
) {
    tower.iter_mut().for_each(|(focus_target, mut weapon_timer)| {
        // check to shoot focus target
        focus_target.0.iter().for_each(|&target_id| {
            // check if enemy exists and then hit
            enemy.get_mut(target_id).iter_mut().for_each(|health| {
                // check if we can shoot at enemy again
                if weapon_timer.0.tick(time.delta()).just_finished() {
                    health.0 = health.0 - 10;

                    // check if we should despawn enemy
                    if health.0 <= 0 {
                        info!("Enemy ded xD {:?}", target_id);
                        // commands.entity(target_id).despawn();
                        commands.entity(target_id).insert(Dead);
                    }
                }
            });
        })
    });
}

/* I wonder if there is any performance gain for having indivdual systems handle dead entities?
   Wonder if there is any "blocking" this may cause.
*/
fn remove_the_dead(
    mut commands: Commands,
    mut things_to_die: Query<(Entity, &Dead)>,
) {
    // its dead xD
    things_to_die.iter().for_each(|(e, _)| {
        info!("Removing ded");
        commands.entity(e).despawn();
    })
}

/**
 * Tries to find the "highest" priority target. (todo: define highest priority)
 */
fn tower_check_for_new_focus_target(
    mut towers: Query<(&CollidingEntities, &mut FocusTarget), With<Tower>>,
    enemy: Query<&Enemy, Without<Dead>>,
) {
    for (colliding_ents, mut focus_target) in towers.iter_mut() {
        // check if tower is already focusing one enemy that is still in range
        let current_target = focus_target.0.filter(|current_target| {
            colliding_ents.contains(current_target.clone())
        }).or_else(|| {
            // pick a valid focus target
            for entity in colliding_ents.iter() {
                // check if entity is an enemy
                if enemy.contains(entity) {
                    return Some(entity);
                }
            }
            return None;
        });

        focus_target.0 = current_target;
    }
}


fn enemy_check_for_focus_target(
    towers: Query<Entity, (With<Tower>, Without<Dead>)>,
    // idk if colliding entities is the correct component to determine an emeies focus target. (because a tower can have a large collider zone which will mess with an enemies colliding bodies component)
    mut enemies: Query<(&CollidingEntities, &mut FocusTarget), (With<Enemy>, Without<Dead>)>,
) {
    for (colliding_ents, mut focus_target) in enemies.iter_mut() {
        // check if the tower being focused is still valid ((not dead and not despawned))
        let current_target = focus_target.0.filter(|current_target| {
            colliding_ents.contains(current_target.clone())
        }).or_else(|| {
            // pick a valid focus target
            for entity in colliding_ents.iter() {
                // check if colliding ent is a tower
                if towers.contains(entity) {
                    return Some(entity);
                }
            }
            return None;
        });
        focus_target.0 = current_target;
    }
}
