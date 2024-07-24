use std::process::exit;

use bevy::{prelude::*, time::Stopwatch};
use input::PlayerInputPlugin;
use level::PlayerLevelPlugin;
use movement::PlayerMovementPlugin;

use crate::enemies::Enemy;

pub mod input;
pub mod level;
pub mod movement;

#[derive(Component)]
pub struct Player {
    pub health: i32,
    pub xp: i32,
    pub lvl: i16,
    pub weapons: Vec<Weapon>,
}

pub struct Weapon {
    pub speed: f32,
    pub damage: i32,
    pub range: f32,
    pub timer: Stopwatch,
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
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..default()
        },
    ));
}

fn tick_weapons(mut player: Query<&mut Player>, time: Res<Time>) {
    for weapon in &mut player.single_mut().weapons {
        weapon.timer.tick(time.delta());
    }
}

fn get_targets(
    mut player: Query<(&mut Player, &Transform)>,
    mut enemies: Query<(&mut Enemy, &Transform)>,
) {
    let player_transform = player.single().1;
    let closest_enemy = enemies
        .iter_mut()
        .min_by_key(|a| a.1.translation.distance(player_transform.translation) as u32);
    let distance = closest_enemy
        .as_ref()
        .map(|enemy| enemy.1.translation.distance(player_transform.translation));
    if let Some(d) = distance {
        let w: i32 = player
            .single_mut()
            .0
            .weapons
            .iter_mut()
            .filter(|weapon| d <= weapon.range && weapon.timer.elapsed_secs() >= weapon.speed)
            .map(|w| {
                w.timer.reset();
                w.damage
            })
            .sum();
        if w == 0 {
            return;
        }
        if let Some(mut enemy) = closest_enemy {
            //TODO: spawn bullets
            // spawn_bullet();
            enemy.0.health -= w;
        }
    }
    // if let Some((enemy, distance)) = enemies.iter_mut().map(|enemy|).min_by_key(||)
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((PlayerMovementPlugin, PlayerInputPlugin, PlayerLevelPlugin))
            .add_systems(Startup, spawn_player)
            .add_systems(Update, (check_player_death, tick_weapons, get_targets));
    }
}
