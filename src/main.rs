use bevy::prelude::*;
use enemies::EnemyPlugin;
use player::PlayerPlugin;
use ui::UiPlugin;

mod enemies;
mod player;
mod ui;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PlayerPlugin, EnemyPlugin, UiPlugin))
        .add_systems(Startup, setup)
        .run();
}
