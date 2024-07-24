use bevy::prelude::*;

use crate::enemies::Enemy;

#[derive(Component)]
pub struct EnemyHealthUi {
    pub sprite: SpriteBundle,
}

fn display_enemy_health(mut uis: Query<(&mut Transform), With<EnemyHealthUi>>) {
    for ui in &mut uis {
        println!("found health ui");
    }
}

pub struct EnemyHealtPlugin;

impl Plugin for EnemyHealtPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, display_enemy_health);
    }
}
