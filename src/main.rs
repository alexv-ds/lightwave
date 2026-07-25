mod app_ext;
mod plugins;

use bevy::prelude::*;
use bevy_transform_interpolation::prelude::*;
use leafwing_input_manager::prelude::*;
use plugins::*;

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Transform::from_xyz(0., 0., 0.),
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: bevy::camera::ScalingMode::FixedVertical {
                viewport_height: 1000.0,
            },
            scale: 0.02,
            ..OrthographicProjection::default_2d()
        }),
    ));

    let input_map = InputMap::new([
        (misc::Action::Left, KeyCode::KeyA),
        (misc::Action::Right, KeyCode::KeyD),
        (misc::Action::Up, KeyCode::KeyW),
        (misc::Action::Down, KeyCode::KeyS),
    ]);

    commands.spawn((
        Transform::from_xyz(0., 0., 0.),
        Sprite::from_color(Color::srgb(0.3, 0.7, 0.9), Vec2::new(0.9, 0.9)),
        // misc::InputMover::default(),
        motion::MotionBundle {
            damping: motion::Damping(11.),
            ..default()
        },
        input_map,
        // NoTranslationEasing
    ));
}

fn main() {
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .set(bevy::log::LogPlugin {
                level: bevy::log::Level::INFO,
                ..default()
            })
            .set(WindowPlugin {
                primary_window: Some(Window {
                    fit_canvas_to_parent: true,
                    resize_constraints: WindowResizeConstraints {
                        min_width: 800.0,
                        min_height: 600.0,
                        ..default()
                    },
                    resizable: false,
                    ..default()
                }),
                ..default()
            }),
    );

    app.add_plugins(bevy::dev_tools::fps_overlay::FpsOverlayPlugin {
        config: bevy::dev_tools::fps_overlay::FpsOverlayConfig {
            enabled: true,
            frame_time_graph_config: bevy::dev_tools::fps_overlay::FrameTimeGraphConfig {
                min_fps: 60.0,
                target_fps: 240.0,
                ..default()
            },
            ..default()
        },
    });

    // app.insert_resource(Time::<Fixed>::from_hz(5.0));
    // app.insert_resource(Time::<Fixed>::from_hz(120.0));

    app.add_plugins(bevy_framepace::FramepacePlugin);
    app.add_plugins(TransformInterpolationPlugin::interpolate_all());
    app.add_plugins(InputManagerPlugin::<misc::Action>::default());
    app.add_plugins(misc::MiscPlugin);
    app.add_plugins(motion::MotionPlugin);
    app.add_systems(Startup, setup);
    app.run();
}
