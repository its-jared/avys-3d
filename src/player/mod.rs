use bevy::{core_pipeline::{bloom::Bloom, tonemapping::Tonemapping}, pbr::*, prelude::*, render::camera::Exposure};

use crate::data::GameConfig;

#[derive(Component)]
pub struct Player;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_player)
            .add_systems(Update, move_player.run_if(resource_exists::<GameConfig>));
    }
}

fn setup_player(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Camera {
            hdr: true,
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
        Atmosphere::EARTH,
        AtmosphereSettings {
            aerial_view_lut_max_distance: 3.2e5,
            scene_units_to_m: 1e+4,
            ..Default::default()
        },
        Exposure::SUNLIGHT,
        Tonemapping::AcesFitted,
        Bloom::NATURAL,
        Player,
    ));
}

fn move_player(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    config: Res<GameConfig>,
    mut player_q: Query<&mut Transform, With<Player>>,
) -> Result<(), BevyError> {
    let mut player_transform = player_q.single_mut()?;
    let mut move_dir = Vec3::ZERO;
    let mut rotation = 0.0;
    let speed = 10.0;
    let rotation_speed = 0.01;

    if keys.pressed(KeyCode::KeyW) {
        move_dir.x = -speed;
    }
    if keys.pressed(KeyCode::KeyS) {
        move_dir.x = speed;
    }

    if keys.pressed(KeyCode::KeyA) {
        move_dir.z = speed;
    }
    if keys.pressed(KeyCode::KeyD) {
        move_dir.z = -speed;
    }

    if keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight) {
        move_dir.y = -speed;
    }
    if keys.pressed(KeyCode::Space) {
        move_dir.y = speed;
    }

    if keys.pressed(KeyCode::KeyQ) {
        rotation = rotation_speed;
    }
    if keys.pressed(KeyCode::KeyE) {
        rotation = -rotation_speed;
    }

    if move_dir != Vec3::ZERO {
        player_transform.translation += move_dir * time.delta_secs();

        if config.defaults.look_at_origin {
            player_transform.look_at(Vec3::ZERO, Vec3::Y);
        }
    }

    if !config.defaults.look_at_origin && rotation != 0.0 {
        player_transform.rotate_y(rotation);
    }

    Ok(())
}