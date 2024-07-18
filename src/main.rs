use std::{borrow::BorrowMut, process::exit, time::Duration};

use bevy::{
    math::NormedVectorSpace, prelude::*, time::Stopwatch, transform::components::Transform,
};

#[derive(Component)]
struct Player {
    pub health: i32,
    pub xp: i32,
    pub lvl: i16,
    pub weapons: Vec<Weapon>,
}

#[derive(Component)]
struct Enemy {
    pub health: i32,
    pub xp_drop: i32,
}

struct Weapon {
    pub speed: f32,
    pub damage: i32,
    pub range: f32,
    pub timer: Stopwatch,
}

const PLAYER_SPEED: f32 = 100.0;

fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let mut horizontal = 0.0;
    let mut vertical = 0.0;
    if keyboard_input.pressed(KeyCode::KeyA) {
        horizontal += -1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        horizontal += 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyW) {
        vertical += 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        vertical += -1.0;
    }

    let mut player_transform = player.single_mut();
    let new_x = player_transform.translation.x + horizontal * PLAYER_SPEED * time.delta_seconds();
    let new_y = player_transform.translation.y + vertical * PLAYER_SPEED * time.delta_seconds();
    //TODO: check pos to be inside the screen
    player_transform.translation = Vec3 {
        x: new_x,
        y: new_y,
        z: -1.0,
    };
}

fn check_player_death(player: Query<&Player>) {
    if player.single().health <= 0 {
        println!("player dead");
        exit(0);
    }
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
            enemy.0.health -= w;
        }
    }
    // if let Some((enemy, distance)) = enemies.iter_mut().map(|enemy|).min_by_key(||)
}

fn tick_weapons(mut player: Query<&mut Player>, time: Res<Time>) {
    for weapon in &mut player.single_mut().weapons {
        weapon.timer.tick(time.delta());
    }
}

fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
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

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                move_player,
                check_player_death,
                (get_targets, check_enemy_death).chain(),
                tick_weapons,
            ),
        )
        .run();
}
