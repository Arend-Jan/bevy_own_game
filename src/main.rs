use bevy::prelude::*;

mod main_menu;
use main_menu::MainMenuPlugin;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    MainMenu,
    InGame,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(WindowDescriptor {
            title: "My Own Game".to_string(),
            vsync: true,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_state(AppState::MainMenu)
        .add_plugin(MainMenuPlugin)
        .run()
}
