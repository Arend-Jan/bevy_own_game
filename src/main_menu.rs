use super::AppState;
use bevy::app::AppExit;
use bevy::prelude::*;

pub struct MainMenuPlugin;

#[derive(Component)]
enum MenuButton {
    Play,
    Quit,
}

struct MenuMaterials {
    root: Handle<ColorMaterial>,
    border: Handle<ColorMaterial>,
    menu: Handle<ColorMaterial>,
    button: Handle<ColorMaterial>,
    button_hovered: Handle<ColorMaterial>,
    button_pressed: Handle<ColorMaterial>,
    button_text: Color,
}

impl FromWorld for MenuMaterials {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        MenuMaterials {
            root: materials.add(Color::NONE.into()),
            border: materials.add(Color::rgb(0.65, 0.65, 0.65).into()),
            menu: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
            button: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
            button_hovered: materials.add(Color::rgb(0.25, 0.25, 0.25).into()),
            button_pressed: materials.add(Color::rgb(0.35, 0.75, 0.35).into()),
            button_text: Color::WHITE,
        }
    }
}

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(button_system.system())
            .add_system(button_press_system.system())
            .add_system_set(
                SystemSet::on_enter(AppState::MainMenu)
                    .with_system(cleanup.system())
                    .with_system(setup.system()),
            )
            .add_system_set(SystemSet::on_exit(AppState::MainMenu).with_system(cleanup.system()));
    }
}

fn button_system(
    materials: Res<MenuMaterials>,
    mut buttons: Query<
        (&Interaction, &mut Handle<ColorMaterial>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut material) in buttons.iter_mut() {
        match *interaction {
            Interaction::Clicked => *material = materials.button_pressed.clone(),
            Interaction::Hovered => *material = materials.button_hovered.clone(),
            Interaction::None => *material = materials.button.clone(),
        }
    }
}

fn button_press_system(
    buttons: Query<(&Interaction, &MenuButton), (Changed<Interaction>, With<Button>)>,
    mut state: ResMut<State<AppState>>,
    mut exit: EventWriter<AppExit>,
) {
    for (interaction, button) in buttons.iter() {
        if *interaction == Interaction::Clicked {
            match button {
                MenuButton::Play => state
                    .set(AppState::InGame)
                    .expect("Couldn't switch state to InGame"),
                MenuButton::Quit => exit.send(AppExit),
            };
        }
    }
}

fn root(materials: &Res<MenuMaterials>) -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        color: materials.root.clone(),
        ..Default::default()
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, materials: Res<MenuMaterials>) {
    commands.spawn_bundle(UiCameraBundle::default()).id();
    commands.spawn_bundle(root(&materials));
}

fn cleanup(mut commands: Commands, query: Query<Entity>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
