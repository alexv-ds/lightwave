use bevy::prelude::*;
// use leafwing_input_manager::prelude::*;

// #[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
// #[actionlike(DualAxis)]
// pub enum DualAxisAction {
//     LookAt,
// }

// //////////////////////////// //
// //////// COMPONENTS //////// //
// //////////////////////////// //

// pub type DualAxisActionState = ActionState<DualAxisAction>;

// In world coords
#[derive(Debug, Component, Deref, DerefMut, Default)]
pub struct LookAt(pub Vec2);

#[derive(Debug, Component, Default)]
#[require(LookAt)]
pub struct LookAtMouse;

// //////////////////////////// //
// ///////// SYSTEMS ////////// //
// //////////////////////////// //

fn update_look_at(
    mut looker_query: Single<(&mut Transform, &mut LookAt), With<LookAtMouse>>,
    window: Single<&Window>,
    camera_query: Single<(&Camera, &GlobalTransform)>,
) {
    let (camera, camera_transform) = camera_query.into_inner();

    let (mut looker_transform, mut looker_look_at) = looker_query.into_inner();

    if let Some(cursor_position) = window.cursor_position()
        && let Ok(cursor_world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_position)
    {

        looker_look_at.0 = cursor_world_pos;

        let z = (cursor_world_pos - looker_transform.translation.xy()).to_angle() - std::f32::consts::FRAC_PI_2;
        looker_transform.rotation = Quat::from_rotation_z(z);
    }
}

// //////////////////////////// //
// ////////// PLUGIN ////////// //
// //////////////////////////// //

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        // app.add_plugins(InputManagerPlugin::<DualAxisAction>::default());
        app.add_systems(FixedUpdate, update_look_at);
    }
}
