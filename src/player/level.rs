use bevy::prelude::*;

use super::Player;

const BASE_XP: f32 = 100.0;

fn level_up(mut player: Query<&mut Player>) {
    let p = player.single();
    if p.xp as f32 >= BASE_XP * (1.0 + (0.1 * (p.lvl as f32 - 1.0))) {
        player.single_mut().lvl += 1;
    }
}

pub struct PlayerLevelPlugin;

impl Plugin for PlayerLevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, level_up);
    }
}
