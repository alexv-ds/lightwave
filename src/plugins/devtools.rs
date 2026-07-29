use crate::app_ext::AppExt;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;


// //////////////////////////// //
// ///////// SYSTEMS ////////// //
// //////////////////////////// //

// fn update_fps_overlay_config(
//     diagnostics: Res<bevy::diagnostic::DiagnosticsStore>,
//     mut fps_config: ResMut<bevy::dev_tools::fps_overlay::FpsOverlayConfig>,
// ) {
//     let fps = match diagnostics
//         .get(&FrameTimeDiagnosticsPlugin::FPS)
//         .and_then(|fps| fps.average())
//     {
//         Some(fps) => fps,
//         None => {
//             fps_config.enabled = false;
//             return;
//         },
//     };
//
//     // fps_config.enabled = true;
//     fps_config.frame_time_graph_config.target_fps = 240.0;//fps.round() as f32;
//     fps_config.frame_time_graph_config.min_fps = 60.0;
//
//     info!("Updating FPS overlay config - {:?}", fps);
// }




// //////////////////////////// //
// ////////// PLUGIN ////////// //
// //////////////////////////// //

pub struct DevToolsPlugin;

impl Plugin for DevToolsPlugin {
    fn build(&self, app: &mut App) {
        app.require_plugin::<bevy::diagnostic::DiagnosticsPlugin>();

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

        // app.add_systems(
        //     Update,
        //     update_fps_overlay_config.run_if(bevy::time::common_conditions::on_timer(
        //         std::time::Duration::from_secs(1),
        //     )),
        // );
    }
}
