use super::AppState;
use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::InGame).with_system(setup_game))
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(movement)
                    .with_system(change_color)
                    .with_system(game_to_menu),
            );
    }
}

fn setup_game(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("images/icon.png"),
        ..Default::default()
    });
}

const SPEED: f32 = 100.0;
fn movement(
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Sprite>>,
) {
    for mut transform in query.iter_mut() {
        let mut direction = Vec3::ZERO;
        if input.pressed(KeyCode::S) {
            direction.x -= 1.0;
        }
        if input.pressed(KeyCode::F) {
            direction.x += 1.0;
        }
        if input.pressed(KeyCode::E) {
            direction.y += 1.0;
        }
        if input.pressed(KeyCode::D) {
            direction.y -= 1.0;
        }

        if direction != Vec3::ZERO {
            transform.translation += direction.normalize() * SPEED * time.delta_seconds();
        }
    }
}

fn change_color(time: Res<Time>, mut query: Query<&mut Sprite>) {
    for mut sprite in query.iter_mut() {
        sprite
            .color
            .set_b((time.seconds_since_startup() * 0.5).sin() as f32 + 2.0);
    }
}

fn game_to_menu(mut state: ResMut<State<AppState>>, input: Res<Input<KeyCode>>) {
    if input.pressed(KeyCode::Escape) {
        state.set(AppState::Menu).unwrap();
    }
}
