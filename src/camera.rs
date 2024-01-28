use bevy::{
    math::Vec3,
    pbr::AmbientLight,
    prelude::{Commands, Camera3dBundle},
};
use smooth_bevy_cameras::controllers::orbit::{OrbitCameraBundle, OrbitCameraController};

pub fn camera_system(mut commands: Commands) {
    commands.spawn(Camera3dBundle::default());
    commands.insert_resource(AmbientLight {
        brightness: 2.0,
        ..Default::default()
    });
    commands.spawn(OrbitCameraBundle::new(
        OrbitCameraController::default(),
        Vec3::new(-2.0, 5.0, 5.0),
        Vec3::new(0., 0.0, -0.2),
        Vec3::Y,
    ));
}
