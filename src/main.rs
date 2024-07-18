use bevy::{prelude::*, time::Stopwatch, transform::components::Transform};
use player::{Player, PlayerPlugin};

mod player;

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
            //TODO: spawn bullets
            // spawn_bullet();
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
        .add_plugins((DefaultPlugins, PlayerPlugin))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            ((get_targets, check_enemy_death).chain(), tick_weapons),
        )
        .run();
}
