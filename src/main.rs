use bevy::prelude::*;
use enemies::EnemyPlugin;
use player::PlayerPlugin;

mod enemies;
mod player;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PlayerPlugin, EnemyPlugin))
        .run();
}
