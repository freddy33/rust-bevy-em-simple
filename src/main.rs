use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin,
    prelude::{App, Startup, Msaa},
    DefaultPlugins,
};
use camera::camera_system;
use dash::dash_fps_system;
use field::field_system;
use smooth_bevy_cameras::{controllers::orbit::OrbitCameraPlugin, LookTransformPlugin};

mod camera;
mod dash;
mod field;
mod mesh;

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .add_plugins(DefaultPlugins)
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(LookTransformPlugin)
        .add_plugins(OrbitCameraPlugin::default())
        .add_systems(Startup, (camera_system, dash_fps_system, field_system))
        // .add_system(dash_fps_update_system)
        .run();
}
