use bevy::prelude::*;

use super::input::PlayerInput;
use super::Player;

const PLAYER_SPEED: f32 = 100.0;

fn move_player(
    mut player: Query<&mut Transform, With<Player>>,
    player_input: Res<PlayerInput>,
    time: Res<Time>,
) {
    let mut player_transform = player.single_mut();
    let new_x = player_transform.translation.x
        + player_input.direction.x * PLAYER_SPEED * time.delta_seconds();
    let new_y = player_transform.translation.y
        + player_input.direction.y * PLAYER_SPEED * time.delta_seconds();
    //TODO: check pos to be inside the screen
    player_transform.translation = Vec3 {
        x: new_x,
        y: new_y,
        z: 1.0,
    };
}

pub struct PlayerMovementPlugin;

impl Plugin for PlayerMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_player);
    }
}
