use bevy::prelude::*;



fn hello_world() {
    // println!("hello world!");
    // info!("HI DUDDLE");
    // trace!("HI DUDDLE");
}

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

    commands.spawn((
        Transform::from_xyz(0., 0., 0.),
        Sprite::from_color(Color::srgb(0.3, 0.7, 0.9), Vec2::new(0.9, 0.9)),
    ));

    // let keks = commands.spawn((Transform::from_xyz(0., 0., 0.)));
    // keks.add;
}

fn main() {
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .set(bevy::log::LogPlugin {
                level: bevy::log::Level::TRACE,
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
                    ..default()
                }),
                ..default()
            }),
    );

    app.add_systems(Startup, setup);

    app.add_systems(Update, hello_world);

    app.run();
}

// fn button(label: &str) -> impl Scene {
//     bsn! {
//         Button
//         Node {
//             width: px(150),
//             height: px(65),
//             border: px(5),
//             border_radius: BorderRadius::MAX,
//             justify_content: JustifyContent::Center,
//             align_items: AlignItems::Center,
//         }
//         BorderColor::from(Color::BLACK)
//         BackgroundColor(Color::srgb(0.15, 0.15, 0.15))
//         Children [(
//             Text(label)
//             TextFont {
//                 font: FontSourceTemplate::Handle("fonts/FiraSans-Bold.ttf"),
//                 font_size: px(33.0),
//             }
//             TextColor(Color::srgb(0.9, 0.9, 0.9))
//             TextShadow
//         )]
//     }
// }
