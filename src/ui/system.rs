use bevy::{prelude::*, transform::commands};
use bevy_color::palettes::css;
use bevy_mod_picking::prelude::*;

use super::{
    component::{GameBtn, NumbersBtn, QuitBtn, RootNode},
    event::{
        ButtonClickEvent, GameBackButtonClickEvent, GameButtonClickEvent, Load70Fire5BtnClick,
        RuleSelectBackBtnClick,
    },
};

fn spawn_main_menu(root_enttiy: Entity, mut commands: Commands) {
    let buttons_style = Style {
        width: Val::Percent(10.),
        height: Val::Percent(10.),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let game_btn = (
        Name::new("game_btn"),
        GameBtn,
        ButtonBundle {
            style: buttons_style.clone(),
            background_color: BackgroundColor(css::BLACK.into()),
            ..default()
        },
        On::<Pointer<Click>>::send_event::<GameButtonClickEvent>(),
    );
    let game_btn_text = (
        Name::new("game_btn_text"),
        TextBundle::from_section("game", TextStyle { ..default() }),
        Pickable::IGNORE,
    );

    let numbers_btn = (
        Name::new("numbers_btn"),
        NumbersBtn,
        ButtonBundle {
            style: buttons_style.clone(),
            background_color: BackgroundColor(css::BLACK.into()),
            ..default()
        },
        On::<Pointer<Click>>::send_event::<ButtonClickEvent>(),
    );
    let numbers_btn_text = (
        Name::new("numbers_btn_text"),
        TextBundle::from_section("numbers", TextStyle { ..default() }),
        Pickable::IGNORE,
    );

    let quit_btn = (
        Name::new("quit_btn"),
        QuitBtn,
        ButtonBundle {
            style: buttons_style.clone(),
            background_color: BackgroundColor(css::BLACK.into()),
            ..default()
        },
        On::<Pointer<Click>>::send_event::<ButtonClickEvent>(),
    );
    let quit_btn_text = (
        Name::new("quit_btn_text"),
        TextBundle::from_section("quit", TextStyle { ..default() }),
        Pickable::IGNORE,
    );

    commands.entity(root_enttiy).with_children(|parent| {
        parent.spawn(game_btn).with_children(|parent| {
            parent.spawn(game_btn_text);
        });

        parent.spawn(numbers_btn).with_children(|parent| {
            parent.spawn(numbers_btn_text);
        });

        parent.spawn(quit_btn).with_children(|parent| {
            parent.spawn(quit_btn_text);
        });
    });
}

pub fn setup_main_ui(mut commands: Commands) {
    let root_node = (
        Name::new("root_node"),
        RootNode,
        NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        },
    );

    let root_entity = commands.spawn(root_node).id();
    spawn_main_menu(root_entity, commands);
}

pub fn main_button_interaction(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
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
                *color = css::BLACK.into();
            }
        }
    }
}

pub fn main_button_click(
    mut er: EventReader<ButtonClickEvent>,
    q_game_btn: Query<&GameBtn>,
    q_numbers_btn: Query<&NumbersBtn>,
    q_quit_btn: Query<&QuitBtn>,
) {
    for &ButtonClickEvent(entity, ..) in er.read() {
        if let Ok(_) = q_game_btn.get(entity) {
            info!("gmae btn click!");
        } else if let Ok(_) = q_numbers_btn.get(entity) {
            info!("numbers btn click!");
        } else if let Ok(_) = q_quit_btn.get(entity) {
            info!("quit btn click!");
        }
    }
}

pub fn game_button_click(
    mut commands: Commands,
    q_root_node: Query<(Entity, &Children), With<RootNode>>,
    mut er: EventReader<GameButtonClickEvent>,
) {
    for _ in er.read() {
        if let Ok((entity, children)) = q_root_node.get_single() {
            for &entity in children.iter() {
                commands.entity(entity).despawn_recursive();
            }
            let buttons_style = Style {
                width: Val::Percent(10.),
                height: Val::Percent(10.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            };

            let loaded_70_fire_5_btn = (
                Name::new("loaded_70_fire_5_btn"),
                ButtonBundle {
                    style: buttons_style.clone(),
                    ..default()
                },
            );
            let loaded_70_fire_5_btn_text = (
                Name::new("loaded_70_fire_5_btn_text"),
                TextBundle::from_section("loaded_70_fire_5", TextStyle { ..default() }),
                Pickable::IGNORE,
            );

            let loaded_45_fire_5_btn = (
                Name::new("loaded_45_fire_5_btn"),
                ButtonBundle {
                    style: buttons_style.clone(),
                    ..default()
                },
            );

            let loaded_45_fire_5_btn_text = (
                Name::new("loaded_45_fire_5_btn_text"),
                TextBundle::from_section("loaded_45_fire_5", TextStyle { ..default() }),
                Pickable::IGNORE,
            );

            let back_btn = (
                Name::new("back_btn"),
                ButtonBundle {
                    style: buttons_style.clone(),
                    ..default()
                },
                On::<Pointer<Click>>::send_event::<GameBackButtonClickEvent>(),
            );

            let back_btn_text = (
                Name::new("back_btn_text"),
                TextBundle::from_section("back", TextStyle { ..default() }),
                Pickable::IGNORE,
            );

            commands.entity(entity).with_children(|parent| {
                parent.spawn(loaded_70_fire_5_btn).with_children(|parent| {
                    parent.spawn(loaded_70_fire_5_btn_text);
                });

                parent.spawn(loaded_45_fire_5_btn).with_children(|parent| {
                    parent.spawn(loaded_45_fire_5_btn_text);
                });

                parent.spawn(back_btn).with_children(|parent| {
                    parent.spawn(back_btn_text);
                });
            });
        }
    }
}

pub fn game_back_btn_click(
    mut commands: Commands,
    mut er: EventReader<GameBackButtonClickEvent>,
    q_root_node: Query<(Entity, &Children), With<RootNode>>,
) {
    for _ in er.read() {
        if let Ok((entity, children)) = q_root_node.get_single() {
            for &entity in children.iter() {
                commands.entity(entity).despawn_recursive();
            }

            spawn_main_menu(entity, commands);
            return;
        }
    }
}

pub fn loaded_70_fire_5_btn_click(
    mut commands: Commands,
    mut er: EventReader<Load70Fire5BtnClick>,
    q_root_node: Query<(Entity, &Children), With<RootNode>>,
) {
    for _ in er.read() {
        if let Ok((entity, children)) = q_root_node.get_single() {
            for &entity in children.iter() {
                commands.entity(entity).despawn_recursive();
            }
            let buttons_style = Style {
                width: Val::Percent(10.),
                height: Val::Percent(10.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            };

            let ball_shuffle_btn = (
                Name::new("ball_shuffle_btn"),
                ButtonBundle {
                    style: buttons_style.clone(),
                    ..default()
                },
            );

            let ball_shuffle_btn_text = (
                Name::new("ball_shuffle_btn_text"),
                TextBundle::from_section("shuffle", TextStyle { ..default() }),
                Pickable::IGNORE,
            );

            let game_run_btn = (
                Name::new("game_run_btn"),
                ButtonBundle {
                    style: buttons_style.clone(),
                    ..default()
                },
            );

            let game_run_btn_text = (
                Name::new("game_run_btn_text"),
                TextBundle::from_section("run", TextStyle { ..default() }),
                Pickable::IGNORE,
            );

            let back_btn = (
                Name::new("back_btn"),
                ButtonBundle {
                    style: buttons_style.clone(),
                    ..default()
                },
                On::<Pointer<Click>>::send_event::<RuleSelectBackBtnClick>(),
            );

            let back_btn_text = (
                Name::new("back_btn_text"),
                TextBundle::from_section("back", TextStyle { ..default() }),
                Pickable::IGNORE,
            );
            commands.entity(entity).with_children(|parent| {
                parent.spawn(ball_shuffle_btn).with_children(|parent| {
                    parent.spawn(ball_shuffle_btn_text);
                });

                parent.spawn(game_run_btn).with_children(|parent| {
                    parent.spawn(game_run_btn_text);
                });

                parent.spawn(back_btn).with_children(|parent| {
                    parent.spawn(back_btn_text);
                });
            });
        }
    }
}

pub fn game_rule_select_back_btn_click(mut er: EventReader<RuleSelectBackBtnClick>) {
    for _ in er.read() {}
}
