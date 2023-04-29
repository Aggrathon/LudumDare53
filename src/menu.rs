use crate::colors;
use crate::state::GameState;
use bevy::prelude::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup_menu.in_schedule(OnEnter(GameState::MainMenu)))
            .add_system(button_system.in_set(OnUpdate(GameState::MainMenu)))
            .add_system(key_system.in_set(OnUpdate(GameState::MainMenu)))
            .add_system(exit_menu.in_schedule(OnExit(GameState::MainMenu)));
    }
}

#[derive(Component)]
struct MainMenu;

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                text.sections[0].style.color = colors::bright();
                *color = colors::dark().into();
                state.set(GameState::Level0);
            }
            Interaction::Hovered => {
                text.sections[0].style.color = colors::dark();
                *color = colors::orange().into();
            }
            Interaction::None => {
                text.sections[0].style.color = colors::dark();
                *color = colors::yellow().into();
            }
        }
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
        .spawn((
            MainMenu,
            NodeBundle {
                style: Style {
                    size: Size::all(Val::Percent(100.)),
                    align_items: AlignItems::Center,
                    align_content: AlignContent::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
        ))
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
                    parent.spawn(TextBundle::from_section(
                        "Tilers Trucking Co",
                        TextStyle {
                            font: font.clone(),
                            font_size: 80.,
                            color: colors::dark(),
                        },
                    ));
                    parent
                        .spawn(ButtonBundle {
                            style: Style {
                                size: Size::new(Val::Px(250.), Val::Px(70.)),
                                align_items: AlignItems::Center,
                                align_content: AlignContent::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            background_color: BackgroundColor(colors::yellow()),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Play",
                                TextStyle {
                                    font,
                                    font_size: 40.,
                                    color: colors::dark(),
                                },
                            ));
                        });
                });
        });
}

fn exit_menu(mut commands: Commands, query: Query<Entity, With<MainMenu>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}
