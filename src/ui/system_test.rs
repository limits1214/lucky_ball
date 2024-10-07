use super::{component::TestBtn, event::ButtonClickEvent};
use bevy::prelude::*;
use bevy_color::palettes::css;
use bevy_mod_picking::prelude::*;

pub fn setup_test_ui(mut commands: Commands) {
    let button1 = (
        Name::new("Button1"),
        TestBtn,
        ButtonBundle {
            style: Style {
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: BackgroundColor(css::GRAY.into()),
            ..default()
        },
        On::<Pointer<Click>>::send_event::<ButtonClickEvent>(),
    );
    let text1 = (
        Name::new("Text1"),
        TextBundle::from_section("Text1", TextStyle::default()),
    );

    commands
        .spawn((NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        },))
        .with_children(|parent| {
            parent.spawn(button1).with_children(|parent| {
                parent.spawn(text1);
            });
        });
}

pub fn test_button_interaction(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>, With<TestBtn>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Hovered => {
                *color = css::BEIGE.into();
            }
            Interaction::Pressed => {
                *color = css::RED.into();
            }
            Interaction::None => {
                *color = css::GRAY.into();
            }
        }
    }
}

pub fn test_button_click(mut er: EventReader<ButtonClickEvent>) {
    for ButtonClickEvent(entity, ..) in er.read() {
        info!("click!");
    }
}
