use bevy::prelude::*;

use crate::player::Player;

use super::Enemy;

const ENEMY_SPEED: f32 = 40.0;

fn move_enemies(
    mut enemies: Query<&mut Transform, With<Enemy>>,
    player_transform: Query<&Transform, (With<Player>, Without<Enemy>)>,
    time: Res<Time>,
) {
    for mut enemy in &mut enemies {
        let direction = (player_transform.single().translation.truncate()
            - enemy.translation.truncate())
        .normalize_or_zero();
        let delta = time.delta_seconds();
        enemy.translation.x += direction.x * ENEMY_SPEED * delta;
        enemy.translation.y += direction.y * ENEMY_SPEED * delta;
    }
}

pub struct EnemyAiPlugin;

impl Plugin for EnemyAiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_enemies);
    }
}
