use bevy::prelude::*;

const MINIMAL_VELOCITY_SQUARED_LENGTH: f32 = 1.0E-10;

// //////////////////////////// //
// //////// COMPONENTS //////// //
// //////////////////////////// //

#[derive(Debug, Component, Deref, DerefMut, Default)]
pub struct Force(pub Vec2);

#[derive(Debug, Component, Deref, DerefMut, Default)]
pub struct Impulse(pub Vec2);

#[derive(Debug, Component, Deref, DerefMut)]
#[require(Force, Impulse, Velocity)]
pub struct Mass(pub f32);

impl Default for Mass {
    fn default() -> Self {
        Self(1.0)
    }
}

#[derive(Debug, Component, Deref, DerefMut, Default)]
#[require(Transform)]
pub struct Velocity(pub Vec2);

// Уменьшение скорости тела
// Текущим плагином только чтение
#[derive(Debug, Component, Deref, DerefMut, Default)]
pub struct Damping(pub f32);

// //////////////////////////// //
// ///////// BUNDLES ////////// //
// //////////////////////////// //

#[derive(Debug, Bundle, Default)]
pub struct MotionBundle {
    pub mass: Mass,
    pub force: Force,
    pub impulse: Impulse,
    pub velocity: Velocity,
    pub damping: Damping,
}

// //////////////////////////// //
// ///////// SYSTEMS ////////// //
// //////////////////////////// //

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub struct MotionSystems;

fn apply_damping(mut query: Query<(&mut Force, &Velocity, &Damping)>, time: Res<Time>) {
    let dt = time.delta_secs();
    for (mut force, velocity, damping) in &mut query {
        if velocity.length_squared() >= MINIMAL_VELOCITY_SQUARED_LENGTH {
            **force += -(**damping) * **velocity;
        }
    }
}

fn update_velocity(
    mut params: ParamSet<(
        Query<(&mut Velocity, &Force, &Mass), Changed<Force>>,
        Query<(&mut Velocity, &Impulse, &Mass), Changed<Impulse>>,
    )>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();

    for (mut velocity, force, mass) in &mut params.p0() {
        let mut new_velocity = **velocity + (**force / **mass) * dt;
        if new_velocity.length_squared() < MINIMAL_VELOCITY_SQUARED_LENGTH {
            new_velocity = Vec2::ZERO;
        }
        if **velocity != new_velocity {
            **velocity = new_velocity;
        }
    }

    for (mut velocity, impulse, mass) in &mut params.p1() {
        let mut new_velocity = **velocity + **impulse / **mass;
        if new_velocity.length_squared() < MINIMAL_VELOCITY_SQUARED_LENGTH {
            new_velocity = Vec2::ZERO;
        }
        if **velocity != new_velocity {
            **velocity = new_velocity;
        }
    }
}

fn clean_forces(
    mut force_query: Query<&mut Force, Changed<Force>>,
    mut impulse_query: Query<&mut Impulse, Changed<Impulse>>,
) {
    for mut force in &mut force_query {
        force.0 = Vec2::ZERO;
    }

    for mut impulse in &mut impulse_query {
        impulse.0 = Vec2::ZERO;
    }
}

fn update_transform(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time<Fixed>>) {
    for (mut transform, velocity) in &mut query {
        let delta = **velocity * time.delta_secs();
        transform.translation += delta.extend(0.0);
    }
}

// //////////////////////////// //
// ////////// PLUGIN ////////// //
// //////////////////////////// //

pub struct MotionPlugin;

impl Plugin for MotionPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(FixedUpdate, MotionSystems);

        app.add_systems(
            FixedUpdate,
            (
                apply_damping,
                update_velocity,
                (clean_forces, update_transform),
            )
                .chain()
                .in_set(MotionSystems),
        );
    }
}
