use bevy::prelude::*;

use crate::ui::health::EnemyHealthUi;

use super::{Enemy, EnemyBundle};

#[derive(Default, Resource)]
struct EnemiesData {
    pub count: i32,
}

fn spawn_enemies(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut enemies_data: ResMut<EnemiesData>,
) {
    if enemies_data.count < 10 {
        commands.spawn(EnemyBundle {
            sprite: SpriteBundle {
                texture: assets.load("player.png"),
                transform: Transform::from_xyz(-300.0, -300.0, 0.0),
                ..default()
            },
            enemy: Enemy {
                health: 100,
                xp_drop: 10,
            },
            ui: EnemyHealthUi {
                sprite: SpriteBundle {
                    texture: assets.load("hp.png"),
                    transform: Transform::from_xyz(-300.0, -305.0, 1.0),
                    ..default()
                },
            },
        });
        enemies_data.count += 1;
    }
}

pub struct EnemySpawnPlugin;

impl Plugin for EnemySpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_enemies)
            .init_resource::<EnemiesData>();
    }
}
