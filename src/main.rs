use bevy::prelude::*;

mod game;
mod menu;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    Menu,
    InGame,
}

/// This example illustrates how to use [`States`] to control transitioning from a `Menu` state to
/// an `InGame` state.
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state(AppState::Menu)
        .add_plugin(menu::MenuPlugin)
        .add_plugin(game::GamePlugin)
        .run();
}
