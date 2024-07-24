use bevy::prelude::*;

pub mod health;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((EnemyHealthPlugin));
    }
}
