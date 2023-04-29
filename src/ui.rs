use bevy::prelude::*;

use crate::colors;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(button_system);
    }
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                text.sections[0].style.color = colors::bright();
                *color = colors::dark().into();
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

pub fn button() -> ButtonBundle {
    ButtonBundle {
        style: Style {
            size: Size::new(Val::Px(250.), Val::Px(50.)),
            align_items: AlignItems::Center,
            align_content: AlignContent::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },

        background_color: BackgroundColor(colors::yellow()),
        ..default()
    }
}

pub fn button_text(text: &str, font: Handle<Font>) -> TextBundle {
    TextBundle::from_section(
        text,
        TextStyle {
            font,
            font_size: 32.,
            color: colors::dark(),
        },
    )
}

pub fn button_image() -> ButtonBundle {
    ButtonBundle {
        style: Style {
            size: Size::new(Val::Px(250.), Val::Px(260.)),
            align_items: AlignItems::Center,
            align_content: AlignContent::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::ColumnReverse,
            gap: Size::all(Val::Px(5.)),
            ..default()
        },

        background_color: BackgroundColor(colors::yellow()),
        ..default()
    }
}

pub fn big_button() -> ButtonBundle {
    ButtonBundle {
        style: Style {
            size: Size::new(Val::Px(250.), Val::Px(70.)),
            align_items: AlignItems::Center,
            align_content: AlignContent::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        background_color: BackgroundColor(colors::yellow()),
        ..default()
    }
}

pub fn big_button_text(text: &str, font: Handle<Font>) -> TextBundle {
    TextBundle::from_section(
        text,
        TextStyle {
            font,
            font_size: 40.,
            color: colors::dark(),
        },
    )
}

pub fn title_text(text: &str, font: Handle<Font>) -> TextBundle {
    TextBundle::from_section(
        text,
        TextStyle {
            font,
            font_size: 80.,
            color: colors::dark(),
        },
    )
}

pub fn container_full() -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::all(Val::Percent(100.)),
            align_items: AlignItems::Center,
            align_content: AlignContent::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        ..default()
    }
}

pub fn container_border() -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::all(Val::Percent(100.)),
            align_items: AlignItems::Center,
            align_content: AlignContent::SpaceBetween,
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceBetween,
            ..default()
        },
        ..default()
    }
}

pub fn container_column_end() -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::height(Val::Percent(100.)),
            align_items: AlignItems::End,
            align_content: AlignContent::End,
            justify_content: JustifyContent::End,
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(10.)),
            gap: Size::all(Val::Px(10.)),
            ..default()
        },
        ..default()
    }
}

pub fn image(texture: Handle<Image>) -> ImageBundle {
    ImageBundle {
        style: Style {
            size: Size::all(Val::Px(200.)),
            border: UiRect::all(Val::Px(5.)),
            ..default()
        },
        // background_color: colors::dark().into(),
        image: UiImage {
            texture,
            ..default()
        },
        ..default()
    }
}
