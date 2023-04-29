use crate::colors;
use crate::state::GameState;
use crate::ui::{big_button, big_button_text, container_full, title_text};
use bevy::prelude::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup_menu.in_schedule(OnEnter(GameState::MainMenu)))
            .add_system(button_system.in_set(OnUpdate(GameState::MainMenu)))
            .add_system(key_system.in_set(OnUpdate(GameState::MainMenu)))
            .add_system(exit_menu.in_schedule(OnExit(GameState::MainMenu)));
    }
}

#[derive(Component)]
struct MainMenu;

#[derive(Component)]
struct PlayButton;

fn button_system(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<PlayButton>)>,
    mut state: ResMut<NextState<GameState>>,
) {
    for interaction in &mut interaction_query {
        if let Interaction::Clicked = *interaction {
            state.set(GameState::Level0);
        };
    }
}

fn key_system(mut keys: ResMut<Input<KeyCode>>, mut state: ResMut<NextState<GameState>>) {
    if keys.just_pressed(KeyCode::Space) {
        state.set(GameState::Level0);
        keys.reset(KeyCode::Space);
    }
}

fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("Bungee-Regular.ttf");
    commands
        .spawn((MainMenu, container_full()))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::all(Val::Percent(80.)),
                        align_items: AlignItems::Center,
                        align_content: AlignContent::Center,
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        gap: Size::height(Val::Percent(20.)),
                        ..default()
                    },
                    background_color: BackgroundColor(colors::light_green()),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(title_text("Tilers Trucking Co", font.clone()));
                    parent.spawn((PlayButton, big_button())).with_children(|p| {
                        p.spawn(big_button_text("Play", font));
                    });
                });
        });
}

fn exit_menu(mut commands: Commands, query: Query<Entity, With<MainMenu>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}
