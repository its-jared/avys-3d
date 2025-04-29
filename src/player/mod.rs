use bevy::{core_pipeline::{bloom::Bloom, tonemapping::Tonemapping, Skybox}, pbr::{Atmosphere, AtmosphereSettings}, prelude::*};
use bevy_panorbit_camera::PanOrbitCamera;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player);
    }
}

fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let night_sky = asset_server.load("textures/night_sky.jpg");

    commands.spawn((
        Camera3d::default(),
        Camera {
            hdr: true,
            clear_color: ClearColorConfig::None,
            ..default()
        },
        Skybox {
            image: night_sky,
            brightness: 500.0,
            rotation: Quat::default(),
        },
        Tonemapping::AcesFitted,
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        Bloom::NATURAL,
        Atmosphere::EARTH,
        AtmosphereSettings {
            aerial_view_lut_max_distance: 3.2e5,
            scene_units_to_m: 1e+4,
            ..Default::default()
        },

    ));
}