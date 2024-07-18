use std::process::exit;

use bevy::{prelude::*, time::Stopwatch};
use input::PlayerInputPlugin;
use movement::PlayerMovementPlugin;

use crate::Weapon;

pub mod input;
pub mod movement;

#[derive(Component)]
pub struct Player {
    pub health: i32,
    pub xp: i32,
    pub lvl: i16,
    pub weapons: Vec<Weapon>,
}

fn check_player_death(player: Query<&Player>) {
    if player.single().health <= 0 {
        println!("player dead");
        exit(0);
    }
}

fn spawn_player(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn((
        Player {
            health: 100,
            lvl: 1,
            xp: 0,
            weapons: vec![Weapon {
                speed: 1.0,
                damage: 12,
                range: 400.0,
                timer: Stopwatch::new(),
            }],
        },
        SpriteBundle {
            texture: assets.load("player.png"),
            transform: Transform::from_xyz(0.0, 0.0, -1.0),
            ..default()
        },
    ));
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((PlayerMovementPlugin, PlayerInputPlugin))
            .add_systems(Startup, spawn_player)
            .add_systems(Update, check_player_death);
    }
}
