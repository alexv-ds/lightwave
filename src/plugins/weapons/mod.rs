mod components;

use crate::app_ext::AppExt;
use crate::plugins::motion;
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;

// //////////////////////////// //
// //////// COMPONENTS //////// //
// //////////////////////////// //

#[derive(Debug, Component, Default, Deref, DerefMut)]
pub struct FiringTarget(pub Vec2);

#[derive(Debug, Component, Default, Deref, DerefMut)]
pub struct NextFireAfter(pub std::time::Duration);

// pub struct FireDelay()

#[derive(Debug, Component, Default, Deref, DerefMut)]
pub struct DespawnAfter(pub std::time::Duration);

#[derive(Debug, Component, Default)]
pub struct TestWeapon {
    pub delay: std::time::Duration,
}

#[derive(Debug, Component, Default)]
pub struct TestProjectile {
    pub force: Vec2,
}

// //////////////////////////// //
// ///////// SYSTEMS ////////// //
// //////////////////////////// //

fn despawn_entities(
    mut commands: Commands,
    time: Res<Time>,
    query: Query<(Entity, &DespawnAfter)>,
) {
    let now = time.elapsed_secs_f64();
    for (entity, despawn_after) in &query {
        if now >= despawn_after.as_secs_f64() {
            commands.entity(entity).despawn_children();
            commands.entity(entity).despawn();
        }
    }
}

fn apply_force_test_projectile(mut force_query: Query<(&mut motion::Force, &TestProjectile)>) {
    for (mut force, projectile) in &mut force_query {
        **force += projectile.force;
    }
}

fn test_weapon_fire(
    mut commands: Commands,
    time: Res<Time>,
    mut weapon_query: Query<(
        &TestWeapon,
        &GlobalTransform,
        &mut NextFireAfter,
        &FiringTarget,
    )>,
) {
    for (weapon, global_transform, mut next_fire, target) in &mut weapon_query {
        if **next_fire > time.elapsed() {
            continue;
        }
        **next_fire = time.elapsed() + weapon.delay;

        let direction =
            (**target - global_transform.translation().xy()).normalize_or(Vec2::new(1.0, 0.0));

        commands.spawn((
            Name::new("TestProjectile"),
            Transform {
                translation: global_transform.translation().xy().extend(0.0),
                rotation: Quat::from_rotation_z(direction.to_angle()),
                ..default()
            },
            Sprite::from_color(Color::srgb(1.0, 0.7, 0.0), Vec2::new(0.5, 0.2)),
            motion::MotionBundle {
                damping: motion::Damping(3.),
                mass: motion::Mass(0.1),
                ..default()
            },
            DespawnAfter(time.elapsed() + std::time::Duration::from_secs(10)),
            TestProjectile {
                force: direction * 20.0,
            },
        ));
    }
}

// //////////////////////////// //
// ////////// PLUGIN ////////// //
// //////////////////////////// //
pub struct WeaponsPlugin;

impl Plugin for WeaponsPlugin {
    fn build(&self, app: &mut App) {
        app.require_plugin::<motion::MotionPlugin>();

        app.add_systems(
            FixedPostUpdate,
            despawn_entities.run_if(on_timer(std::time::Duration::from_millis(500))),
        );

        app.add_systems(
            FixedUpdate,
            apply_force_test_projectile.before(motion::MotionSystems),
        );

        app.add_systems(FixedUpdate, test_weapon_fire);

        // app.spawn_empty();
    }
}
