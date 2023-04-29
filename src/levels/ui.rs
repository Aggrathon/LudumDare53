use std::f32::consts::PI;
use std::time::Duration;

use bevy::prelude::*;
use bevy_easings::*;

use crate::state::GameState;
use crate::ui::{button, button_image, button_text, container_border, container_column_end, image};

#[derive(Component)]
pub struct GameUI;
#[derive(Component)]
pub struct RestartButton;
#[derive(Component)]
pub struct NextButton;
#[derive(Component)]
pub struct MenuButton;
#[derive(Component)]
pub struct RotateButton;
#[derive(Component)]
pub struct TileImage(f32);

pub fn setup_gui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("Bungee-Regular.ttf");
    commands
        .spawn((GameUI, container_border()))
        .with_children(|p| {
            p.spawn(container_column_end()).with_children(|p| {
                p.spawn((RestartButton, button())).with_children(|p| {
                    p.spawn(button_text("Restart (R)", font.clone()));
                });
                p.spawn((MenuButton, button())).with_children(|p| {
                    p.spawn(button_text("Menu (ESC)", font.clone()));
                });
            });
            p.spawn(container_column_end()).with_children(|p| {
                p.spawn((RotateButton, button_image())).with_children(|p| {
                    p.spawn(button_text("Rotate (SPC)", font));
                    p.spawn((TileImage(0.), image(asset_server.load("tile_tr.png"))));
                });
            });
        });
}

fn restart_level(next_state: &mut ResMut<NextState<GameState>>, state: &Res<State<GameState>>) {
    next_state.set(state.0);
}

fn next_level(next_state: &mut ResMut<NextState<GameState>>, state: &Res<State<GameState>>) {
    next_state.set(state.0.next_level());
}

fn main_menu(next_state: &mut ResMut<NextState<GameState>>) {
    next_state.set(GameState::MainMenu);
}

fn rotate(commands: &mut Commands, query: &mut Query<(&Transform, Entity, &mut TileImage)>) {
    for (tr, e, mut rot) in query {
        rot.0 += PI * 0.5;
        commands.entity(e).insert(tr.ease_to(
            tr.with_rotation(Quat::from_rotation_z(rot.0)),
            EaseFunction::QuadraticInOut,
            EasingType::Once {
                duration: Duration::from_millis(500),
            },
        ));
    }
}

pub fn key_system(
    mut keys: ResMut<Input<KeyCode>>,
    mut commands: Commands,
    mut query: Query<(&Transform, Entity, &mut TileImage)>,
    mut next_state: ResMut<NextState<GameState>>,
    state: Res<State<GameState>>,
) {
    if keys.just_pressed(KeyCode::R) {
        keys.reset(KeyCode::R);
        restart_level(&mut next_state, &state);
    }
    if keys.just_pressed(KeyCode::N) {
        keys.reset(KeyCode::N);
        next_level(&mut next_state, &state);
    }
    if keys.just_pressed(KeyCode::Escape) {
        keys.reset(KeyCode::Escape);
        main_menu(&mut next_state);
    }
    if keys.just_pressed(KeyCode::Space) {
        keys.reset(KeyCode::Space);
        rotate(&mut commands, &mut query);
    }
}

pub fn button_restart(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<RestartButton>)>,
    mut next_state: ResMut<NextState<GameState>>,
    state: Res<State<GameState>>,
) {
    for interaction in &mut interaction_query {
        if let Interaction::Clicked = *interaction {
            restart_level(&mut next_state, &state);
        };
    }
}
pub fn button_next(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<NextButton>)>,
    mut next_state: ResMut<NextState<GameState>>,
    state: Res<State<GameState>>,
) {
    for interaction in &mut interaction_query {
        if let Interaction::Clicked = *interaction {
            next_level(&mut next_state, &state);
        };
    }
}

pub fn button_menu(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<MenuButton>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for interaction in &mut interaction_query {
        if let Interaction::Clicked = *interaction {
            main_menu(&mut next_state);
        };
    }
}

pub fn button_rotate(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<RotateButton>)>,
    mut commands: Commands,
    mut query: Query<(&Transform, Entity, &mut TileImage)>,
) {
    for interaction in &mut interaction_query {
        if let Interaction::Clicked = *interaction {
            rotate(&mut commands, &mut query);
        };
    }
}
