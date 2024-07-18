use bevy::prelude::*;

use crate::player::Player;

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

fn spawn_enemies(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    for _ in 0..1 {
        commands.spawn((
            SpriteBundle {
                texture: assets.load("player.png"),
                ..default()
            },
            Enemy {
                health: 100,
                xp_drop: 10,
            },
        ));
    }
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_enemies)
            .add_systems(Update, check_enemy_death);
    }
}
