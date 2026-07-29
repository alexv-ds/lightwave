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
            trace!("Despawned: {}", entity);
        }
    }
}

fn apply_force_test_projectile(mut force_query: Query<(&mut motion::Force, &TestProjectile)>) {
    for (mut force, projectile) in &mut force_query {
        **force += projectile.force;
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
        
        // app.spawn_empty();
    }
}
