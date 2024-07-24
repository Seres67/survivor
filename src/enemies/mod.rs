// use ai::EnemyAiPlugin;
use bevy::prelude::*;
use spawn::EnemySpawnPlugin;

use crate::{player::Player, ui::health::EnemyHealthUi};

pub mod ai;
pub mod spawn;

#[derive(Component)]
pub struct Enemy {
    pub health: i32,
    pub xp_drop: i32,
}

fn check_enemy_death(
    mut commands: Commands,
    enemies: Query<(Entity, &Enemy)>,
    mut player: Query<&mut Player>,
) {
    for (entity, enemy) in &enemies {
        if enemy.health <= 0 {
            player.single_mut().xp += enemy.xp_drop;
            commands.entity(entity).despawn();
            println!("enemy dead");
        }
    }
}

#[derive(Bundle)]
struct EnemyBundle {
    sprite: SpriteBundle,
    enemy: Enemy,
    ui: EnemyHealthUi,
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((EnemyAiPlugin, EnemySpawnPlugin))
            .add_systems(Update, check_enemy_death);
    }
}
