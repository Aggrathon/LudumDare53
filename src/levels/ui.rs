use std::f32::consts::PI;
use std::time::Duration;

use bevy::prelude::*;
use bevy_easings::*;

use crate::deck::Deck;
use crate::state::GameState;
use crate::tile::TileServer;
use crate::ui::{button, button_image, button_text, container_border, container_column_end, image};
use crate::world::TilePlaced;

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

pub fn update_tile(
    tile_placed: EventReader<TilePlaced>,
    mut deck: ResMut<Deck>,
    mut query: Query<(&mut UiImage, &mut Transform, &mut TileImage)>,
    ts: Res<TileServer>,
) {
    if !tile_placed.is_empty() {
        if let Ok((mut img, mut tr, mut ti)) = query.get_single_mut() {
            let (tile, rot) = deck.get_top();
            let (img2, rot2) = ts.find_texture(&tile);
            img.texture = img2;
            tr.rotation = Quat::from_rotation_z(rot + rot2);
            ti.0 = rot + rot2;
        }
    }
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
    let (tr, e, mut rot) = query.single_mut();
    rot.0 += PI * 0.5;
    commands.entity(e).insert(tr.ease_to(
        tr.with_rotation(Quat::from_rotation_z(rot.0)),
        EaseFunction::QuadraticInOut,
        EasingType::Once {
            duration: Duration::from_millis(500),
        },
    ));
}

fn next_tile(mut deck: ResMut<Deck>, mut tile_placed: EventWriter<TilePlaced>) {
    deck.next();
    tile_placed.send(TilePlaced);
}

pub fn key_system(
    mut keys: ResMut<Input<KeyCode>>,
    mut commands: Commands,
    mut query: Query<(&Transform, Entity, &mut TileImage)>,
    mut next_state: ResMut<NextState<GameState>>,
    state: Res<State<GameState>>,
    deck: ResMut<Deck>,
    tile_placed: EventWriter<TilePlaced>,
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
    if keys.just_pressed(KeyCode::S) {
        keys.reset(KeyCode::S);
        next_tile(deck, tile_placed);
    }
}

pub fn button_restart(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<RestartButton>)>,
    mut next_state: ResMut<NextState<GameState>>,
    state: Res<State<GameState>>,
) {
    if let Ok(Interaction::Clicked) = interaction_query.get_single() {
        restart_level(&mut next_state, &state);
    }
}
pub fn button_next(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<NextButton>)>,
    mut next_state: ResMut<NextState<GameState>>,
    state: Res<State<GameState>>,
) {
    if let Ok(Interaction::Clicked) = interaction_query.get_single() {
        next_level(&mut next_state, &state);
    };
}

pub fn button_menu(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<MenuButton>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if let Ok(Interaction::Clicked) = interaction_query.get_single() {
        main_menu(&mut next_state);
    };
}

pub fn button_rotate(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<RotateButton>)>,
    mut commands: Commands,
    mut query: Query<(&Transform, Entity, &mut TileImage)>,
) {
    if let Ok(Interaction::Clicked) = interaction_query.get_single() {
        rotate(&mut commands, &mut query);
    };
}
