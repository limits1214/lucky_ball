use std::slice::Windows;

use bevy::{prelude::*, window::WindowResized};
use bevy_color::palettes::css;
use bevy_mod_picking::prelude::*;
use uuid::Uuid;

use crate::{
    ffi::{
        ffi_fn::{kv_set, quit_app},
        ffi_trait::{AppFfi, AppFfiTrait},
    },
    game::{
        event::{BallClearEvent, BallSpawnEvent, GameEndEvent, GameRunEvent},
        resource::{ball26, ball45, ball69, ball70, make_given_ball, GameConfig},
    },
    ui::utils::paginate_with_total,
};

use super::{
    component::{
        CustomRuleBall, CustomRuleFireCnt, GameBtn, GameRunBtn, NumbersBtn, NumbersContentNode,
        NumbersItem, NumbersPagination, QuitBtn, RootNode, SaveBtn, ShuffleBtn, TextResize,
    },
    event::{
        BackToGameRuleSelectBtnClick, BackToMainMenuBtnClickEvent, ButtonClickEvent,
        CustomGameRuleBtnClick, CustomRuleBallClick, CustomRuleFireCntDownClick,
        CustomRuleFireCntUpClick, CustomRuleRunBtnClick, GameMenuShuffleBtnClick,
        GameResultRetryBtnClick, GameResultSaveBtnClick, GameRuleSelectButtonClickEvent,
        GameRunBtnClick, Load26Fire1BtnClick, Load45Fire6BtnClick, Load69Fire5BtnClick,
        NumbersBtnClick, NumbersItemDeleteBtnClick, NumbersPagingNextBtnClick,
        NumbersPagingPrevBtnClick, QuitBtnClick,
    },
    resource::{BallNumber, SavedCustomRule, UiConfig, VecBallNumberExt},
    utils::make_text,
};

fn spawn_main_menu(root_entity: Entity, mut commands: Commands) {
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
        On::<Pointer<Click>>::send_event::<GameRuleSelectButtonClickEvent>(),
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
        On::<Pointer<Click>>::send_event::<NumbersBtnClick>(),
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
        On::<Pointer<Click>>::send_event::<QuitBtnClick>(),
    );
    let quit_btn_text = (
        Name::new("quit_btn_text"),
        TextBundle::from_section("quit", TextStyle { ..default() }),
        Pickable::IGNORE,
    );

    commands.entity(root_entity).with_children(|parent| {
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

fn spawn_game_rule_select_menu(root_entity: Entity, mut commands: Commands) {
    let buttons_style = Style {
        width: Val::Percent(10.),
        height: Val::Percent(10.),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let loaded69_fire_5_btn = (
        Name::new("loaded_69_fire_5_btn"),
        ButtonBundle {
            style: buttons_style.clone(),
            ..default()
        },
        On::<Pointer<Click>>::send_event::<Load69Fire5BtnClick>(),
    );
    let loaded_69_fire_5_btn_text = (
        Name::new("loaded_69_fire_5_btn_text"),
        TextBundle::from_section("loaded_69_fire_5", TextStyle { ..default() }),
        Pickable::IGNORE,
    );

    let loaded_26_fire_1_btn = (
        Name::new("loaded_26_fire_1_btn"),
        ButtonBundle {
            style: buttons_style.clone(),
            ..default()
        },
        On::<Pointer<Click>>::send_event::<Load26Fire1BtnClick>(),
    );

    let loaded_26_fire_1_btn_text = (
        Name::new("loaded_26_fire_1_btn_text"),
        TextBundle::from_section("loaded_26_fire_1", TextStyle { ..default() }),
        Pickable::IGNORE,
    );

    let loaded_45_fire_5_btn = (
        Name::new("loaded_45_fire_5_btn"),
        ButtonBundle {
            style: buttons_style.clone(),
            ..default()
        },
        On::<Pointer<Click>>::send_event::<Load45Fire6BtnClick>(),
    );

    let loaded_45_fire_5_btn_text = (
        Name::new("loaded_45_fire_5_btn_text"),
        TextBundle::from_section("loaded_45_fire_5", TextStyle { ..default() }),
        Pickable::IGNORE,
    );

    let custom_game_rule_btn = (
        Name::new("custom_game_rule_btn"),
        ButtonBundle {
            style: buttons_style.clone(),
            ..default()
        },
        On::<Pointer<Click>>::send_event::<CustomGameRuleBtnClick>(),
    );

    let custom_game_rule_btn_text = (
        Name::new("custom_game_rule_btn_text"),
        TextBundle::from_section("custom_game_rule", TextStyle { ..default() }),
        Pickable::IGNORE,
    );

    let back_btn = (
        Name::new("back_btn"),
        ButtonBundle {
            style: buttons_style.clone(),
            ..default()
        },
        On::<Pointer<Click>>::send_event::<BackToMainMenuBtnClickEvent>(),
    );

    let back_btn_text = (
        Name::new("back_btn_text"),
        TextBundle::from_section("back", TextStyle { ..default() }),
        Pickable::IGNORE,
    );

    commands.entity(root_entity).with_children(|parent| {
        parent.spawn(loaded69_fire_5_btn).with_children(|parent| {
            parent.spawn(loaded_69_fire_5_btn_text);
        });

        parent.spawn(loaded_26_fire_1_btn).with_children(|parent| {
            parent.spawn(loaded_26_fire_1_btn_text);
        });

        parent.spawn(loaded_45_fire_5_btn).with_children(|parent| {
            parent.spawn(loaded_45_fire_5_btn_text);
        });

        parent.spawn(custom_game_rule_btn).with_children(|parent| {
            parent.spawn(custom_game_rule_btn_text);
        });

        parent.spawn(back_btn).with_children(|parent| {
            parent.spawn(back_btn_text);
        });
    });
}

fn spawn_game_menu(root_entity: Entity, mut commands: Commands) {
    let wrap = (
        Name::new("wrap"),
        NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::End,
                flex_direction: FlexDirection::Row,
                ..default()
            },
            ..default()
        },
    );

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
        ShuffleBtn,
        On::<Pointer<Click>>::send_event::<GameMenuShuffleBtnClick>(),
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
        GameRunBtn,
        On::<Pointer<Click>>::send_event::<GameRunBtnClick>(),
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
        On::<Pointer<Click>>::send_event::<BackToGameRuleSelectBtnClick>(),
    );

    let back_btn_text = (
        Name::new("back_btn_text"),
        TextBundle::from_section("back", TextStyle { ..default() }),
        Pickable::IGNORE,
    );
    commands.entity(root_entity).with_children(|parent| {
        parent.spawn(wrap).with_children(|parent| {
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
    });
}

fn spawn_numbers_menu(root_entity: Entity, mut commands: Commands, ball_numbers: &Vec<BallNumber>) {
    let (paginated_ball_numbers, total_size) = paginate_with_total(&ball_numbers, 0, 5);

    let wrap = (NodeBundle {
        style: Style {
            width: Val::Percent(90.),
            height: Val::Percent(90.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        ..default()
    });

    let title_wrap = (NodeBundle {
        style: Style {
            width: Val::Percent(100.),
            height: Val::Percent(10.),
            ..default()
        },
        background_color: BackgroundColor(css::BISQUE.into()),
        ..default()
    });

    let back_btn = (
        Name::new("back_btn"),
        ButtonBundle {
            style: Style {
                width: Val::Percent(10.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        },
        On::<Pointer<Click>>::send_event::<BackToMainMenuBtnClickEvent>(),
    );

    let back_btn_text = (
        Name::new("back_btn_text"),
        TextBundle::from_section("<", TextStyle { ..default() }),
        Pickable::IGNORE,
    );
    let title_txt_wrap = (NodeBundle {
        style: Style {
            width: Val::Percent(90.),
            height: Val::Percent(100.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        ..default()
    });

    let title = (TextBundle::from_section("numbers", TextStyle::default()));

    let content_wrap = (
        Name::new("NumbersContentNode"),
        NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(80.),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: BackgroundColor(css::BLACK.into()),
            ..default()
        },
        NumbersContentNode,
    );

    let paging_wrap = (NodeBundle {
        style: Style {
            width: Val::Percent(100.),
            height: Val::Percent(10.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        background_color: BackgroundColor(css::YELLOW_GREEN.into()),
        ..default()
    });

    let paging_prev_btn = (
        NodeBundle {
            style: Style {
                width: Val::Percent(20.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        },
        On::<Pointer<Click>>::send_event::<NumbersPagingPrevBtnClick>(),
    );

    let paging_prev_btn_txt = (
        TextBundle::from_section("Prev", TextStyle::default()),
        Pickable::IGNORE,
    );

    let paging_txt_wrap = (NodeBundle {
        style: Style {
            width: Val::Percent(20.),
            height: Val::Percent(100.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        ..default()
    },);

    let paging_txt = (
        TextBundle::from_section(
            format!("1/{}", if total_size < 1 { 1 } else { total_size }),
            TextStyle::default(),
        ),
        NumbersPagination {
            now: 0,
            last: total_size,
        },
    );

    let paging_next_btn = (
        NodeBundle {
            style: Style {
                width: Val::Percent(20.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        },
        On::<Pointer<Click>>::send_event::<NumbersPagingNextBtnClick>(),
    );

    let paging_next_btn_txt = (
        TextBundle::from_section("Next", TextStyle::default()),
        Pickable::IGNORE,
    );

    let mut content_entity = commands.spawn_empty().id();

    commands.entity(root_entity).with_children(|parent| {
        parent.spawn(wrap).with_children(|parent| {
            parent.spawn(title_wrap).with_children(|parent| {
                parent.spawn(back_btn).with_children(|parent| {
                    parent.spawn(back_btn_text);
                });

                parent.spawn(title_txt_wrap).with_children(|parent| {
                    parent.spawn(title);
                });
            });

            content_entity = parent.spawn(content_wrap).id();

            parent.spawn(paging_wrap).with_children(|parent| {
                parent.spawn(paging_prev_btn).with_children(|parent| {
                    parent.spawn(paging_prev_btn_txt);
                });

                parent.spawn(paging_txt_wrap).with_children(|parent| {
                    parent.spawn(paging_txt);
                });

                parent.spawn(paging_next_btn).with_children(|parent| {
                    parent.spawn(paging_next_btn_txt);
                });
            });
        });
    });

    spawn_numbers_contents(content_entity, commands, paginated_ball_numbers);
}

fn spawn_numbers_contents(
    content_entity: Entity,
    mut commands: Commands,
    ball_nums: &[BallNumber],
) {
    let content_item_wrap = (NodeBundle {
        style: Style {
            width: Val::Percent(100.),
            height: Val::Percent(20.),
            ..default()
        },
        ..default()
    });

    let content_item_number_wrap = (NodeBundle {
        style: Style {
            width: Val::Percent(10.),
            height: Val::Percent(100.),
            ..default()
        },
        ..default()
    });

    let content_item_number_txt = (TextBundle::from_section("1", TextStyle::default()));

    let content_item_body_wrap = (NodeBundle {
        style: Style {
            width: Val::Percent(70.),
            height: Val::Percent(100.),
            flex_direction: FlexDirection::Column,
            ..default()
        },
        ..default()
    });
    let contnet_item_body_type_txt =
        (TextBundle::from_section("45load6fire", TextStyle::default()));
    let contnet_item_body_numbers_txt =
        (TextBundle::from_section("1,2,3,4,5,6,11,2,3,4,5,6,", TextStyle::default()));
    let contnet_item_body_time_txt: TextBundle =
        (TextBundle::from_section("2024.12.3", TextStyle::default()));
    let content_item_remove_btn = (ButtonBundle {
        style: Style {
            width: Val::Percent(20.),
            height: Val::Percent(100.),
            ..default()
        },
        ..default()
    },);
    let content_item_remove_btn_txt = (TextBundle::from_section("rm", TextStyle::default()));
    commands.entity(content_entity).with_children(|parent| {
        for (
            i,
            BallNumber {
                numbers,
                game_type,
                time,
                id,
            },
        ) in ball_nums.iter().enumerate()
        {
            parent
                .spawn(content_item_wrap.clone())
                .insert(NumbersItem {
                    game_type: game_type.clone(),
                    id: id.clone(),
                    numbers: numbers.clone(),
                    time: time.clone(),
                })
                .with_children(|parent| {
                    parent
                        .spawn(content_item_number_wrap.clone())
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                i.to_string(),
                                TextStyle::default(),
                            ));
                        });
                    parent
                        .spawn(content_item_body_wrap.clone())
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                format!("{game_type}"),
                                TextStyle::default(),
                            ));
                            parent.spawn(TextBundle::from_section(
                                format!("{numbers:?}"),
                                TextStyle::default(),
                            ));
                            parent.spawn(TextBundle::from_section(
                                format!("{time}"),
                                TextStyle::default(),
                            ));
                        });
                    parent
                        .spawn(content_item_remove_btn.clone())
                        .insert(On::<Pointer<Click>>::send_event::<NumbersItemDeleteBtnClick>())
                        .with_children(|parent| {
                            parent
                                .spawn(TextBundle::from_section("rm", TextStyle::default()))
                                .insert(Pickable::IGNORE);
                        });
                });
        }
    });
}

fn spawn_custom_rule_menu(
    root_entity: Entity,
    mut commands: Commands,
    custom_rule: &SavedCustomRule,
) {
    let buttons_style = Style {
        width: Val::Percent(10.),
        height: Val::Percent(10.),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let wrap = (
        Name::new("wrap"),
        NodeBundle {
            style: Style {
                width: Val::Percent(70.),
                height: Val::Percent(70.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: BackgroundColor(css::WHITE.into()),
            ..default()
        },
    );
    let wrap2 = (NodeBundle {
        style: Style {
            width: Val::Percent(100.),
            height: Val::Percent(80.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Row,
            border: UiRect::all(Val::Px(1.)),
            ..default()
        },
        background_color: BackgroundColor(css::YELLOW.into()),
        ..default()
    },);
    let wrap3 = (NodeBundle {
        style: Style {
            width: Val::Percent(100.),
            height: Val::Percent(20.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Row,
            border: UiRect::all(Val::Px(1.)),
            ..default()
        },
        background_color: BackgroundColor(css::YELLOW.into()),
        ..default()
    },);
    let custom_line_1_wrap = (
        Name::new("custom_line_1"),
        NodeBundle {
            style: Style {
                width: Val::Percent(20.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                border: UiRect::all(Val::Px(1.)),
                ..default()
            },
            background_color: BackgroundColor(css::YELLOW.into()),
            ..default()
        },
    );

    let circle_1 = (NodeBundle {
        style: Style {
            width: Val::Percent(90.),
            height: Val::Percent(100. * 1. / 14.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border: UiRect::all(Val::Px(1.)),
            ..default()
        },
        border_color: BorderColor(css::BLACK.into()),
        background_color: BackgroundColor(css::WHEAT.into()),
        ..default()
    });

    let circle_buttons_style = Style {
        width: Val::Percent(90.),
        height: Val::Percent(90.),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let circle_btn = (ButtonBundle {
        style: circle_buttons_style.clone(),
        ..default()
    });

    let fire_btn_style = Style {
        width: Val::Percent(40.),
        height: Val::Percent(80.),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let left_btn = (ButtonBundle {
        style: fire_btn_style.clone(),
        ..default()
    });

    let left_btn_txt = (make_text("<"));

    let fire_cnt_txt = (TextBundle::from_section(
        custom_rule.fire.to_string(),
        TextStyle {
            font_size: 30.,
            color: css::BLACK.into(),
            ..default()
        },
    ));

    let right_btn = (ButtonBundle {
        style: fire_btn_style.clone(),
        ..default()
    });
    let right_btn_txt = (make_text(">"));

    let back_run_wrap = (NodeBundle {
        style: Style {
            width: Val::Percent(70.),
            height: Val::Percent(10.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Row,
            border: UiRect::all(Val::Px(1.)),
            ..default()
        },
        background_color: BackgroundColor(css::YELLOW.into()),
        ..default()
    });
    let bottom_btn_style = Style {
        width: Val::Percent(90.),
        height: Val::Percent(90.),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let back_btn = (
        Name::new("back_btn"),
        ButtonBundle {
            style: bottom_btn_style.clone(),
            ..default()
        },
        On::<Pointer<Click>>::send_event::<BackToGameRuleSelectBtnClick>(),
    );
    let back_btn_text = (
        Name::new("back_btn_text"),
        TextBundle::from_section("back", TextStyle { ..default() }),
        Pickable::IGNORE,
    );
    let run_btn = (
        Name::new("back_btn"),
        ButtonBundle {
            style: bottom_btn_style.clone(),
            ..default()
        },
        On::<Pointer<Click>>::send_event::<CustomRuleRunBtnClick>(),
    );
    let run_btn_text = (
        Name::new("back_btn_text"),
        TextBundle::from_section("run", TextStyle { ..default() }),
        Pickable::IGNORE,
    );
    commands.entity(root_entity).with_children(|parent| {
        parent.spawn(wrap).with_children(|parent| {
            parent.spawn(make_text("load ball!"));
            parent.spawn(wrap2).with_children(|parent| {
                // 1
                parent
                    .spawn(custom_line_1_wrap.clone())
                    .with_children(|parent| {
                        for i in 1..=14 {
                            let custom_ball = custom_rule.load[i - 1].clone();
                            let ox = if custom_ball.1 { "[O]" } else { "[X]" };
                            parent.spawn(circle_1.clone()).with_children(|parent| {
                                parent
                                    .spawn(circle_btn.clone())
                                    .insert(custom_ball)
                                    .insert(
                                        On::<Pointer<Click>>::send_event::<CustomRuleBallClick>(),
                                    )
                                    .with_children(|parent| {
                                        parent
                                            .spawn(TextBundle::from_sections([
                                                TextSection::new(
                                                    &i.to_string(),
                                                    TextStyle::default(),
                                                ),
                                                TextSection::new(ox, TextStyle::default()),
                                            ]))
                                            .insert(Pickable::IGNORE);
                                    });
                            });
                        }
                    });
                // 2
                parent
                    .spawn(custom_line_1_wrap.clone())
                    .with_children(|parent| {
                        for i in 15..=28 {
                            let custom_ball = custom_rule.load[i - 1].clone();
                            let ox = if custom_ball.1 { "[O]" } else { "[X]" };
                            parent.spawn(circle_1.clone()).with_children(|parent| {
                                parent
                                    .spawn(circle_btn.clone())
                                    .insert(custom_ball)
                                    .insert(
                                        On::<Pointer<Click>>::send_event::<CustomRuleBallClick>(),
                                    )
                                    .with_children(|parent| {
                                        parent
                                            .spawn(TextBundle::from_sections([
                                                TextSection::new(
                                                    &i.to_string(),
                                                    TextStyle::default(),
                                                ),
                                                TextSection::new(ox, TextStyle::default()),
                                            ]))
                                            .insert(Pickable::IGNORE);
                                    });
                            });
                        }
                    });
                // 3
                parent
                    .spawn(custom_line_1_wrap.clone())
                    .with_children(|parent| {
                        for i in 29..=42 {
                            let custom_ball = custom_rule.load[i - 1].clone();
                            let ox = if custom_ball.1 { "[O]" } else { "[X]" };
                            parent.spawn(circle_1.clone()).with_children(|parent| {
                                parent
                                    .spawn(circle_btn.clone())
                                    .insert(custom_ball)
                                    .insert(
                                        On::<Pointer<Click>>::send_event::<CustomRuleBallClick>(),
                                    )
                                    .with_children(|parent| {
                                        parent
                                            .spawn(TextBundle::from_sections([
                                                TextSection::new(
                                                    &i.to_string(),
                                                    TextStyle::default(),
                                                ),
                                                TextSection::new(ox, TextStyle::default()),
                                            ]))
                                            .insert(Pickable::IGNORE);
                                    });
                            });
                        }
                    });
                // 4
                parent
                    .spawn(custom_line_1_wrap.clone())
                    .with_children(|parent| {
                        for i in 43..=56 {
                            let custom_ball = custom_rule.load[i - 1].clone();
                            let ox = if custom_ball.1 { "[O]" } else { "[X]" };
                            parent.spawn(circle_1.clone()).with_children(|parent| {
                                parent
                                    .spawn(circle_btn.clone())
                                    .insert(custom_ball)
                                    .insert(
                                        On::<Pointer<Click>>::send_event::<CustomRuleBallClick>(),
                                    )
                                    .with_children(|parent| {
                                        parent
                                            .spawn(TextBundle::from_sections([
                                                TextSection::new(
                                                    &i.to_string(),
                                                    TextStyle::default(),
                                                ),
                                                TextSection::new(ox, TextStyle::default()),
                                            ]))
                                            .insert(Pickable::IGNORE);
                                    });
                            });
                        }
                    });
                // 5
                parent
                    .spawn(custom_line_1_wrap.clone())
                    .with_children(|parent| {
                        for i in 57..=70 {
                            let custom_ball = custom_rule.load[i - 1].clone();
                            let ox = if custom_ball.1 { "[O]" } else { "[X]" };
                            parent.spawn(circle_1.clone()).with_children(|parent| {
                                parent
                                    .spawn(circle_btn.clone())
                                    .insert(custom_ball)
                                    .insert(
                                        On::<Pointer<Click>>::send_event::<CustomRuleBallClick>(),
                                    )
                                    .with_children(|parent| {
                                        parent
                                            .spawn(TextBundle::from_sections([
                                                TextSection::new(
                                                    &i.to_string(),
                                                    TextStyle::default(),
                                                ),
                                                TextSection::new(ox, TextStyle::default()),
                                            ]))
                                            .insert(Pickable::IGNORE);
                                    });
                            });
                        }
                    });
            });
            parent.spawn(make_text("fire ball!"));
            parent.spawn(wrap3).with_children(|parent| {
                // <
                parent
                    .spawn(left_btn)
                    .insert(On::<Pointer<Click>>::send_event::<CustomRuleFireCntDownClick>())
                    .with_children(|parent| {
                        parent.spawn(left_btn_txt);
                    });
                // fire_cnt
                parent
                    .spawn(fire_cnt_txt)
                    .insert(CustomRuleFireCnt(custom_rule.fire));
                // >
                parent
                    .spawn(right_btn)
                    .insert(On::<Pointer<Click>>::send_event::<CustomRuleFireCntUpClick>())
                    .with_children(|parent| {
                        parent.spawn(right_btn_txt);
                    });
            });
        });
        parent.spawn(back_run_wrap).with_children(|parent| {
            // back
            parent.spawn(back_btn).with_children(|parent| {
                parent.spawn(back_btn_text);
            });
            // run
            parent.spawn(run_btn).with_children(|parent| {
                parent.spawn(run_btn_text);
            });
        });
    });
}

fn spawn_game_result_menu(root_entity: Entity, mut commands: Commands, picked_numbers: &[u8]) {
    // retry_btn <- x
    // back_btn
    // result
    // save btn

    let wrap = (NodeBundle {
        style: Style {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::End,
            align_items: AlignItems::Center,
            ..default()
        },
        ..default()
    });
    let result_wrap = (NodeBundle {
        style: Style {
            width: Val::Percent(100.),
            height: Val::Percent(20.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        ..default()
    });

    let result_txt =
        (TextBundle::from_section(format!("{picked_numbers:?}"), TextStyle::default()));

    let btn_wrap = (NodeBundle {
        style: Style {
            width: Val::Percent(100.),
            height: Val::Percent(20.),
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        ..default()
    });

    let retry_btn = (
        ButtonBundle {
            style: Style {
                width: Val::Percent(20.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        },
        On::<Pointer<Click>>::send_event::<GameResultRetryBtnClick>(),
    );
    let retry_btn_txt = (TextBundle::from_section("retry", TextStyle::default()));
    let save_btn = (
        ButtonBundle {
            style: Style {
                width: Val::Percent(20.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        },
        SaveBtn,
        On::<Pointer<Click>>::send_event::<GameResultSaveBtnClick>(),
    );
    let save_btn_txt = (TextBundle::from_section("save", TextStyle::default()));
    let back_btn = (
        ButtonBundle {
            style: Style {
                width: Val::Percent(20.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        },
        On::<Pointer<Click>>::send_event::<BackToGameRuleSelectBtnClick>(),
    );
    let back_btn_txt = (TextBundle::from_section("back", TextStyle::default()));
    commands.entity(root_entity).with_children(|parent| {
        parent.spawn(wrap).with_children(|parent| {
            parent.spawn(result_wrap).with_children(|parent| {
                parent.spawn(result_txt);
            });
            parent.spawn(btn_wrap).with_children(|parent| {
                // 의미 없어보임
                // parent.spawn(retry_btn).with_children(|parent| {
                //     parent.spawn(retry_btn_txt);
                // });
                parent.spawn(save_btn).with_children(|parent| {
                    parent.spawn(save_btn_txt);
                });
                parent.spawn(back_btn).with_children(|parent| {
                    parent.spawn(back_btn_txt);
                });
            });
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

pub fn button_interaction(
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

pub fn game_rule_select_button_click(
    mut commands: Commands,
    q_root_node: Query<(Entity, &Children), With<RootNode>>,
    mut er: EventReader<GameRuleSelectButtonClickEvent>,
) {
    for _ in er.read() {
        if let Ok((entity, children)) = q_root_node.get_single() {
            for &entity in children.iter() {
                commands.entity(entity).despawn_recursive();
            }
            return spawn_game_rule_select_menu(entity, commands);
        }
    }
}

pub fn back_to_main_menu_btn_click(
    mut commands: Commands,
    mut er: EventReader<BackToMainMenuBtnClickEvent>,
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

pub fn loaded_69_fire_5_btn_click(
    mut commands: Commands,
    mut er: EventReader<Load69Fire5BtnClick>,
    q_root_node: Query<(Entity, &Children), With<RootNode>>,
    mut config: ResMut<GameConfig>,
    mut ew: EventWriter<BallSpawnEvent>,
) {
    for _ in er.read() {
        if let Ok((entity, children)) = q_root_node.get_single() {
            for &entity in children.iter() {
                commands.entity(entity).despawn_recursive();
            }

            config.rule_given_ball = make_given_ball(ball69());
            config.rule_taken_ball = 5;
            ew.send(BallSpawnEvent(false));

            return spawn_game_menu(entity, commands);
        }
    }
}

pub fn loaded_26_fire_1_btn_click(
    mut commands: Commands,
    mut er: EventReader<Load26Fire1BtnClick>,
    q_root_node: Query<(Entity, &Children), With<RootNode>>,
    mut config: ResMut<GameConfig>,
    mut ew: EventWriter<BallSpawnEvent>,
) {
    for _ in er.read() {
        if let Ok((entity, children)) = q_root_node.get_single() {
            for &entity in children.iter() {
                commands.entity(entity).despawn_recursive();
            }

            config.rule_given_ball = make_given_ball(ball26());
            config.rule_taken_ball = 1;
            ew.send(BallSpawnEvent(false));

            return spawn_game_menu(entity, commands);
        }
    }
}

pub fn loaded_45_fire_6_btn_click(
    mut commands: Commands,
    mut er: EventReader<Load45Fire6BtnClick>,
    q_root_node: Query<(Entity, &Children), With<RootNode>>,
    mut config: ResMut<GameConfig>,
    mut ew: EventWriter<BallSpawnEvent>,
) {
    for _ in er.read() {
        if let Ok((entity, children)) = q_root_node.get_single() {
            for &entity in children.iter() {
                commands.entity(entity).despawn_recursive();
            }

            config.rule_given_ball = make_given_ball(ball45());
            config.rule_taken_ball = 6;
            ew.send(BallSpawnEvent(false));

            return spawn_game_menu(entity, commands);
        }
    }
}

pub fn custom_game_rule_btn_click(
    mut commands: Commands,
    mut er: EventReader<CustomGameRuleBtnClick>,
    q_root_node: Query<(Entity, &Children), With<RootNode>>,
    ui_config: Res<UiConfig>,
) {
    for _ in er.read() {
        if let Ok((entity, children)) = q_root_node.get_single() {
            for &entity in children.iter() {
                commands.entity(entity).despawn_recursive();
            }

            // info!(
            //     "ui_config.saved_custom_rule {:?}",
            //     ui_config.saved_custom_rule
            // );

            return spawn_custom_rule_menu(entity, commands, &ui_config.saved_custom_rule);
        }
    }
}

pub fn back_to_game_rule_select_btn_click(
    mut commands: Commands,
    mut er: EventReader<BackToGameRuleSelectBtnClick>,
    q_root_node: Query<(Entity, &Children), With<RootNode>>,
    mut ew: EventWriter<BallClearEvent>,
    mut config: ResMut<GameConfig>,
) {
    for _ in er.read() {
        if let Ok((entity, children)) = q_root_node.get_single() {
            for &entity in children.iter() {
                commands.entity(entity).despawn_recursive();
            }
            config.rule_given_ball = make_given_ball(ball70());
            config.rule_taken_ball = 5;
            config.picked_ball = vec![];
            ew.send(BallClearEvent);
            return spawn_game_rule_select_menu(entity, commands);
        }
    }
}

pub fn game_run_btn_click(
    mut commands: Commands,
    mut er: EventReader<GameRunBtnClick>,
    mut ew: EventWriter<GameRunEvent>,
    q_shuffle_btn: Query<Entity, With<ShuffleBtn>>,
    q_game_run_btn: Query<Entity, With<GameRunBtn>>,
) {
    for _ in er.read() {
        if let Ok(entity) = q_shuffle_btn.get_single() {
            commands.entity(entity).insert(Visibility::Hidden);
        }
        if let Ok(entity) = q_game_run_btn.get_single() {
            commands.entity(entity).insert(Visibility::Hidden);
        }
        ew.send(GameRunEvent);
    }
}

pub fn resize_text_based_on_window(
    mut query: Query<&mut Text, With<TextResize>>,
    mut resize_reader: EventReader<WindowResized>,
) {
    for e in resize_reader.read() {
        return;
        for mut t in &mut query {
            for t2 in t.sections.iter_mut() {
                t2.style.font_size = e.width * 0.05;
            }
        }
    }
}

pub fn custom_rule_run_btn_click(
    mut commands: Commands,
    mut er: EventReader<CustomRuleRunBtnClick>,
    q_root_node: Query<(Entity, &Children), With<RootNode>>,
    mut config: ResMut<GameConfig>,
    mut ew: EventWriter<BallSpawnEvent>,
    q_custom_balls: Query<&CustomRuleBall>,
    q_firecnt: Query<&CustomRuleFireCnt>,
) {
    for _ in er.read() {
        if let Ok((entity, children)) = q_root_node.get_single() {
            for &entity in children.iter() {
                commands.entity(entity).despawn_recursive();
            }

            let mut v = vec![];
            for a in &q_custom_balls {
                if a.1 {
                    v.push(a.0);
                }
            }
            config.rule_given_ball = make_given_ball(v);

            if let Ok(fc) = q_firecnt.get_single() {
                config.rule_taken_ball = fc.0;
            }

            ew.send(BallSpawnEvent(false));
            return spawn_game_menu(entity, commands);
        }
    }
}

pub fn custom_rule_ball_click(
    mut er: EventReader<CustomRuleBallClick>,
    mut q_custom_ball: Query<(&mut CustomRuleBall)>,
    q_child: Query<&Children>,
    mut q_text: Query<&mut Text>,
    mut ui_config: ResMut<UiConfig>,
) {
    for evt in er.read() {
        info!("custom_rule_ball_click");
        if let Ok((mut cb)) = q_custom_ball.get_mut(evt.0) {
            cb.1 = !cb.1;

            ui_config.saved_custom_rule.load[(cb.0 - 1) as usize].1 = cb.1;
            ui_config.saved_custom_rule.save_custom_rule();

            if let Ok(child) = q_child.get(evt.0) {
                if let Ok(mut txt) = q_text.get_mut(child[0]) {
                    if cb.1 {
                        txt.sections[1].value = "[O]".to_string();
                    } else {
                        txt.sections[1].value = "[X]".to_string();
                    }
                }
            }
        }
    }
}

pub fn custom_rule_fire_down_click(
    mut er: EventReader<CustomRuleFireCntDownClick>,
    mut q_firecnt: Query<(&mut CustomRuleFireCnt, &mut Text)>,
    mut ui_config: ResMut<UiConfig>,
) {
    for _ in er.read() {
        info!("custom_rule_fire_down_click");
        if let Ok((mut fc, mut text)) = q_firecnt.get_single_mut() {
            if fc.0 > 1 {
                fc.0 -= 1;

                ui_config.saved_custom_rule.fire = fc.0;
                ui_config.saved_custom_rule.save_custom_rule();

                text.sections[0].value = fc.0.to_string();
            }
        }
    }
}

pub fn custom_rule_fire_up_click(
    mut er: EventReader<CustomRuleFireCntUpClick>,
    mut q_firecnt: Query<(&mut CustomRuleFireCnt, &mut Text)>,
    q_custom_balls: Query<&CustomRuleBall>,
    mut ui_config: ResMut<UiConfig>,
) {
    for _ in er.read() {
        info!("custom_rule_fire_up_click");
        if let Ok((mut fc, mut text)) = q_firecnt.get_single_mut() {
            let mut cnt = 0;
            for cb in &q_custom_balls {
                if cb.1 {
                    cnt += 1;
                }
            }

            if cnt > 30 {
                cnt = 30;
            }

            if fc.0 < cnt {
                fc.0 += 1;

                ui_config.saved_custom_rule.fire = fc.0;
                ui_config.saved_custom_rule.save_custom_rule();

                text.sections[0].value = fc.0.to_string();
            }
        }
    }
}

pub fn quit_btn_click(
    mut er: EventReader<QuitBtnClick>,
    #[cfg(not(any(target_os = "android", target_os = "ios")))] mut exit: EventWriter<AppExit>,
) {
    for _ in er.read() {
        #[cfg(any(target_os = "android", target_os = "ios"))]
        {
            quit_app();
        }
        #[cfg(not(any(target_os = "android", target_os = "ios")))]
        {
            exit.send(AppExit::Success);
        }
    }
}

pub fn numbers_btn_click(
    mut commands: Commands,
    mut er: EventReader<NumbersBtnClick>,
    q_root_node: Query<(Entity, &Children), With<RootNode>>,
    ui_config: Res<UiConfig>,
) {
    for _ in er.read() {
        if let Ok((entity, children)) = q_root_node.get_single() {
            for &entity in children.iter() {
                commands.entity(entity).despawn_recursive();
            }

            return spawn_numbers_menu(entity, commands, &ui_config.saved_ball_numbers);
        }
    }
}

pub fn numbers_paging_prev_click(
    mut commands: Commands,
    mut er: EventReader<NumbersPagingPrevBtnClick>,
    q_content_node: Query<(Entity, &Children), With<NumbersContentNode>>,
    ui_config: Res<UiConfig>,
    mut q_number_pagination: Query<(&mut NumbersPagination, &mut Text)>,
) {
    for _ in er.read() {
        if let Ok((entity, children)) = q_content_node.get_single() {
            for &entity in children.iter() {
                commands.entity(entity).despawn_recursive();
            }

            if let Ok((mut pagination, mut text)) = q_number_pagination.get_single_mut() {
                if pagination.now > 0 {
                    pagination.now -= 1;
                }

                text.sections[0].value = format!("{}/{}", pagination.now + 1, pagination.last);

                let (ball_nums, total_size) =
                    paginate_with_total(&ui_config.saved_ball_numbers, pagination.now, 5);
                return spawn_numbers_contents(entity, commands, ball_nums);
            }
        }
    }
}

pub fn numbers_paging_next_click(
    mut commands: Commands,
    mut er: EventReader<NumbersPagingNextBtnClick>,
    q_content_node: Query<(Entity, &Children), With<NumbersContentNode>>,
    ui_config: Res<UiConfig>,
    mut q_number_pagination: Query<(&mut NumbersPagination, &mut Text)>,
) {
    for _ in er.read() {
        if let Ok((entity, children)) = q_content_node.get_single() {
            for &entity in children.iter() {
                commands.entity(entity).despawn_recursive();
            }

            if let Ok((mut pagination, mut text)) = q_number_pagination.get_single_mut() {
                if pagination.now + 1 < pagination.last {
                    pagination.now += 1;
                }

                text.sections[0].value = format!("{}/{}", pagination.now + 1, pagination.last);
                let (ball_nums, total_size) =
                    paginate_with_total(&ui_config.saved_ball_numbers, pagination.now, 5);
                return spawn_numbers_contents(entity, commands, ball_nums);
            }
        }
    }
}

pub fn numbers_item_delete_btn_click(
    mut commands: Commands,
    mut er: EventReader<NumbersItemDeleteBtnClick>,
    q_parent: Query<&Parent>,
    q_number_item: Query<&NumbersItem>,
    mut ui_config: ResMut<UiConfig>,
    q_content_node: Query<(Entity, &Children), With<NumbersContentNode>>,
    mut q_number_pagination: Query<(&mut NumbersPagination, &mut Text)>,
) {
    for evt in er.read() {
        for anc in q_parent.iter_ancestors(evt.0) {
            if let Ok(number_item) = q_number_item.get(anc) {
                ui_config
                    .saved_ball_numbers
                    .delete_item(number_item.id.clone());

                // commands.entity(anc).despawn_recursive();
            }

            if let Ok((entity, children)) = q_content_node.get_single() {
                for &entity in children.iter() {
                    commands.entity(entity).despawn_recursive();
                }

                if let Ok((mut pagination, mut text)) = q_number_pagination.get_single_mut() {
                    // if pagination.now + 1 < pagination.last {
                    //     pagination.now += 1;
                    // }

                    let (ball_nums, total_size) =
                        paginate_with_total(&ui_config.saved_ball_numbers, pagination.now, 5);

                    pagination.last = total_size;
                    info!("pagination {pagination:?}");
                    if pagination.now + 1 > total_size {
                        pagination.now = total_size - 1;
                    }
                    let (ball_nums, total_size) =
                        paginate_with_total(&ui_config.saved_ball_numbers, pagination.now, 5);

                    text.sections[0].value = format!(
                        "{}/{}",
                        if pagination.now + 1 < 1 {
                            1
                        } else {
                            pagination.now + 1
                        },
                        if pagination.last < 1 {
                            1
                        } else {
                            pagination.last
                        }
                    );
                    return spawn_numbers_contents(entity, commands, ball_nums);
                }
            }
        }
    }
}

pub fn game_menu_shuffle_btn_click(
    mut er: EventReader<GameMenuShuffleBtnClick>,
    mut ew: EventWriter<BallSpawnEvent>,
) {
    for _ in er.read() {
        ew.send(BallSpawnEvent(true));
    }
}

pub fn er_game_end(
    mut commands: Commands,
    mut er: EventReader<GameEndEvent>,
    q_root_node: Query<(Entity, &Children), With<RootNode>>,
    config: Res<GameConfig>,
) {
    for _ in er.read() {
        if let Ok((entity, children)) = q_root_node.get_single() {
            for &entity in children.iter() {
                commands.entity(entity).despawn_recursive();
            }

            return spawn_game_result_menu(entity, commands, &config.picked_ball);
        }
    }
}

pub fn game_result_menu_retry_btn_click(
    mut commands: Commands,
    mut er: EventReader<GameResultRetryBtnClick>,
    q_root_node: Query<(Entity, &Children), With<RootNode>>,
) {
    for _ in er.read() {
        if let Ok((entity, children)) = q_root_node.get_single() {
            for &entity in children.iter() {
                commands.entity(entity).despawn_recursive();
            }

            return spawn_game_menu(entity, commands);
        }
    }
}

pub fn game_result_menu_save_btn_click(
    mut commands: Commands,
    mut er: EventReader<GameResultSaveBtnClick>,
    q_root_node: Query<(Entity, &Children), With<RootNode>>,
    q_save_btn: Query<Entity, With<SaveBtn>>,
    config: Res<GameConfig>,
    mut ui_config: ResMut<UiConfig>,
) {
    for _ in er.read() {
        // if let Ok((entity, children)) = q_root_node.get_single() {
        //     // for &entity in children.iter() {
        //     //     commands.entity(entity).despawn_recursive();
        //     // }
        // }

        let numbers = &config.picked_ball;
        let ball_num = BallNumber {
            game_type: "".to_string(),
            id: Uuid::new_v4().to_string(),
            numbers: numbers.clone(),
            time: AppFfi::get_current_epoch_time(),
        };

        ui_config.saved_ball_numbers.save_item(ball_num);

        //to
        if let Ok(entity) = q_save_btn.get_single() {
            commands.entity(entity).insert(Visibility::Hidden);
        }
    }
}
