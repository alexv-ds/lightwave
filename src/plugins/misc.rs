use crate::app_ext::AppExt;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use crate::motion;

pub struct MiscPlugin;

impl Plugin for MiscPlugin {
    fn build(&self, app: &mut App) {
        app.require_plugin::<InputManagerPlugin<Action>>();
        app.require_plugin::<bevy::time::TimePlugin>();

        app.add_systems(FixedUpdate, lets_move);

        info!("Hello from MiscPlugin!");
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum Action {
    Left,
    Right,
    Up,
    Down,
}


fn lets_move(
    time: Res<Time>,
    action_state: Single<(&ActionState<Action>, &mut motion::Force)>,
) {
    let (action_state, mut force) = action_state.into_inner();
    const SPEED: f32 = 300.0;

    if action_state.pressed(&Action::Left) {
        force.x -= SPEED;
    }
    if action_state.pressed(&Action::Right) {
        force.x += SPEED;
    }
    if action_state.pressed(&Action::Up) {
        force.y += SPEED;
    }
    if action_state.pressed(&Action::Down) {
        force.y -= SPEED;
    }
}

// fn lets_move(
//     time: Res<Time>,
//     action_state: Single<(&ActionState<Action>, &mut motion::Impulse)>,
// ) {
//     let (action_state, mut force) = action_state.into_inner();
//     const SPEED: f32 = 10.0;
//
//     if action_state.just_pressed(&Action::Left) {
//         force.x -= SPEED;
//     }
//     if action_state.just_pressed(&Action::Right) {
//         force.x += SPEED;
//     }
//     if action_state.just_pressed(&Action::Up) {
//         force.y += SPEED;
//     }
//     if action_state.just_pressed(&Action::Down) {
//         force.y -= SPEED;
//     }
// }
