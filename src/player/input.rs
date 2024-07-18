use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct PlayerInput {
    pub direction: Vec2,
}

fn handle_input(keyboard_input: Res<ButtonInput<KeyCode>>, mut player_input: ResMut<PlayerInput>) {
    let mut direction = Vec2::default();
    if keyboard_input.pressed(KeyCode::KeyA) {
        direction += Vec2::new(-1.0, 0.0);
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        direction += Vec2::new(1.0, 0.0);
    }
    if keyboard_input.pressed(KeyCode::KeyW) {
        direction += Vec2::new(0.0, 1.0);
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        direction += Vec2::new(0.0, -1.0);
    }

    player_input.direction = direction.normalize_or_zero();
}

pub struct PlayerInputPlugin;

impl Plugin for PlayerInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_input)
            .init_resource::<PlayerInput>();
    }
}
