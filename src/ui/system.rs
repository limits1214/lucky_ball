use bevy::{color::palettes::css, prelude::*, window::WindowResized};

use uuid::Uuid;

use crate::{
    asset::resources::MyAsstes,
    ffi::{
        event::{AdmobBannerLaunch, AppInitStartEvent},
        ffi_trait::{AppFfi, AppFfiTrait},
    },
    game::{
        component::{RemixerEndTimer, RemixerJudgeTimer, RemixerTimer},
        constant::{STEP_BALL_MIXER_ROTATE, STEP_INNER_DRAW_STICK_DOWN, STEP_INNER_DRAW_STICK_UP},
        event::{
            BallClearEvent, BallSpawnEvent, DrawStickResetEvent, GameEndEvent, GameRunEvent,
            GameStepData, GameStepStartEvent,
        },
        resource::{ball26, ball45, ball69, ball70, make_given_ball, GameConfig},
    },
    ui::utils::paginate_with_total,
};

use super::{
    component::{
        BtnIndianRedInteract, BtnInteract, CustomRuleBall, CustomRuleFireCnt, GameBtn, GameRunBtn,
        NumbersBtn, NumbersContentNode, NumbersItem, NumbersPagination, QuitBtn, RootNode, SaveBtn,
        ShuffleBtn, TextResize,
    },
    constant::{
        BUTTON_BG_COLOR, BUTTON_BORDER_COLOR, BUTTON_CLICK_COLOR, BUTTON_HOVER_COLOR,
        BUTTON_INDIAN_RED_CLICK_COLOR, BUTTON_INDIAN_RED_HOVER_COLOR,
    },
    event::{
        BackToGameRuleSelectBtnClick, BackToMainMenuBtnClickEvent, CustomGameRuleBtnClick,
        CustomRuleBallClick, CustomRuleFireCntDownClick, CustomRuleFireCntUpClick,
        CustomRuleRunBtnClick, GameMenuShuffleBtnClick, GameResultRetryBtnClick,
        GameResultSaveBtnClick, GameRuleSelectButtonClickEvent, GameRunBtnClick,
        Load26Fire1BtnClick, Load45Fire6BtnClick, Load69Fire5BtnClick, NumbersBtnClick,
        NumbersItemDeleteBtnClick, NumbersPagingNextBtnClick, NumbersPagingPrevBtnClick,
        QuitBtnClick,
    },
    i18n::{
        txt_draw_balls_count, txt_insert_balls, txt_ok, txt_quit, txt_saved_numbers, txt_start,
    },
    resource::{BallNumber, SavedCustomRule, UiConfig, VecBallNumberExt},
    utils::{make_text, time_formatting},
};

fn spawn_main_menu(root_entity: Entity, mut commands: Commands, my_assets: Res<MyAsstes>) {
    let game_btn = (
        Name::new("game_btn"),
        GameBtn,
        BtnInteract,
        Button,
        Node {
            width: Val::Percent(25.),
            height: Val::Percent(10.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::all(Val::Percent(1.)),
            border: UiRect::all(Val::Px(2.)),
            ..default()
        },
        BorderColor(BUTTON_BORDER_COLOR),
        BackgroundColor(BUTTON_BG_COLOR),
        BorderRadius::all(Val::Percent(5.)),
    );
    let game_btn_text = (
        Name::new("game_btn_text"),
        Text::new(txt_start()),
        TextFont {
            font: my_assets.ttf_nanum_gothic_bold.clone(),
            font_size: 20.,
            ..default()
        },
        TextResize,
        PickingBehavior::IGNORE,
    );

    let numbers_btn = (
        Name::new("numbers_btn"),
        NumbersBtn,
        BtnInteract,
        Button,
        Node {
            width: Val::Percent(25.),
            height: Val::Percent(10.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::all(Val::Percent(1.)),
            border: UiRect::all(Val::Px(2.)),
            ..default()
        },
        BorderColor(BUTTON_BORDER_COLOR),
        BackgroundColor(BUTTON_BG_COLOR),
        BorderRadius::all(Val::Percent(5.)),
    );
    let numbers_btn_text = (
        Name::new("numbers_btn_text"),
        Text::new(txt_saved_numbers()),
        TextFont {
            font: my_assets.ttf_nanum_gothic_bold.clone(),
            font_size: 20.,
            ..default()
        },
        PickingBehavior::IGNORE,
    );

    let quit_btn = (
        Name::new("quit_btn"),
        QuitBtn,
        BtnInteract,
        Button,
        Node {
            width: Val::Percent(25.),
            height: Val::Percent(10.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::all(Val::Percent(1.)),
            border: UiRect::all(Val::Px(2.)),
            ..default()
        },
        BorderColor(BUTTON_BORDER_COLOR),
        BackgroundColor(BUTTON_BG_COLOR),
        BorderRadius::all(Val::Percent(5.)),
    );
    let quit_btn_text = (
        Name::new("quit_btn_text"),
        Text::new(txt_quit()),
        TextFont {
            font: my_assets.ttf_nanum_gothic_bold.clone(),
            font_size: 20.,
            ..default()
        },
        PickingBehavior::IGNORE,
    );

    commands.entity(root_entity).with_children(|parent| {
        parent
            .spawn(game_btn)
            .observe(
                |t: Trigger<Pointer<Click>>,
                 mut ew: EventWriter<GameRuleSelectButtonClickEvent>| {
                    //GameRuleSelectButtonClickEvent
                    ew.send(GameRuleSelectButtonClickEvent(t.entity(), t.hit.depth));
                },
            )
            .with_children(|parent| {
                parent.spawn(game_btn_text);
            });

        parent
            .spawn(numbers_btn)
            .observe(
                |t: Trigger<Pointer<Click>>, mut ew: EventWriter<NumbersBtnClick>| {
                    //
                    ew.send(NumbersBtnClick(t.entity(), t.hit.depth));
                },
            )
            .with_children(|parent| {
                parent.spawn(numbers_btn_text);
            });

        // parent.spawn(quit_btn).with_children(|parent| {
        //     parent.spawn(quit_btn_text);
        // });
    });
}

fn spawn_game_rule_select_menu(
    root_entity: Entity,
    mut commands: Commands,
    my_assets: Res<MyAsstes>,
    custom_type: String,
) {
    let loaded69_fire_5_btn = (
        Name::new("loaded_69_fire_5_btn"),
        BtnInteract,
        Button,
        Node {
            width: Val::Percent(25.),
            height: Val::Percent(10.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::all(Val::Percent(1.)),
            border: UiRect::all(Val::Px(2.)),
            ..default()
        },
        BorderColor(BUTTON_BORDER_COLOR),
        BackgroundColor(BUTTON_BG_COLOR),
        BorderRadius::all(Val::Percent(5.)),
    );
    let loaded_69_fire_5_btn_text = (
        Name::new("loaded_69_fire_5_btn_text"),
        Text::new("5/69"),
        TextFont {
            font: my_assets.ttf_nanum_gothic_bold.clone(),
            ..default()
        },
        PickingBehavior::IGNORE,
    );

    let loaded_26_fire_1_btn = (
        Name::new("loaded_26_fire_1_btn"),
        BtnInteract,
        Button,
        Node {
            width: Val::Percent(25.),
            height: Val::Percent(10.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::all(Val::Percent(1.)),
            border: UiRect::all(Val::Px(2.)),
            ..default()
        },
        BorderColor(BUTTON_BORDER_COLOR),
        BackgroundColor(BUTTON_BG_COLOR),
        BorderRadius::all(Val::Percent(5.)),
    );

    let loaded_26_fire_1_btn_text = (
        Name::new("loaded_26_fire_1_btn_text"),
        Text::new("1/26"),
        TextFont {
            font: my_assets.ttf_nanum_gothic_bold.clone(),
            ..default()
        },
        PickingBehavior::IGNORE,
    );

    let loaded_45_fire_5_btn = (
        Name::new("loaded_45_fire_5_btn"),
        BtnInteract,
        Button,
        Node {
            width: Val::Percent(25.),
            height: Val::Percent(10.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::all(Val::Percent(1.)),
            border: UiRect::all(Val::Px(2.)),
            ..default()
        },
        BorderColor(BUTTON_BORDER_COLOR),
        BackgroundColor(BUTTON_BG_COLOR),
        BorderRadius::all(Val::Percent(5.)),
    );

    let loaded_45_fire_5_btn_text = (
        Name::new("loaded_45_fire_5_btn_text"),
        Text::new("6/45"),
        TextFont {
            font: my_assets.ttf_nanum_gothic_bold.clone(),
            ..default()
        },
        PickingBehavior::IGNORE,
    );

    let custom_game_rule_btn = (
        Name::new("custom_game_rule_btn"),
        BtnInteract,
        Button,
        Node {
            width: Val::Percent(25.),
            height: Val::Percent(10.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::all(Val::Percent(1.)),
            border: UiRect::all(Val::Px(2.)),
            ..default()
        },
        BorderColor(BUTTON_BORDER_COLOR),
        BackgroundColor(BUTTON_BG_COLOR),
        BorderRadius::all(Val::Percent(5.)),
    );
    let custom_game_rule_img = (
        Node {
            // width: Val::Percent(100.),
            height: Val::Percent(50.),
            ..default()
        },
        ImageNode::new(my_assets.png_customize.clone()),
    );
    
    let custom_game_rule_btn_text = (
        Name::new("custom_game_rule_btn_text"),
        Text::new(format!("{custom_type}")),
        TextFont {
            font: my_assets.ttf_nanum_gothic_bold.clone(),
            font_size: 18.,
            ..default()
        },
        PickingBehavior::IGNORE,
    );

    let back_btn = (
        Name::new("back_btn"),
        BtnInteract,
        Button,
        Node {
            width: Val::Percent(25.),
            height: Val::Percent(10.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::all(Val::Percent(1.)),
            border: UiRect::all(Val::Px(2.)),
            ..default()
        },
        BorderColor(BUTTON_BORDER_COLOR),
        BackgroundColor(BUTTON_BG_COLOR),
        BorderRadius::all(Val::Percent(5.)),
    );

    let back_btn_img = (
        Node {
            // width: Val::Percent(100.),
            height: Val::Percent(100.),
            ..default()
        },
        ImageNode::new(my_assets.png_back.clone()),
    );

    commands.entity(root_entity).with_children(|parent| {
        parent
            .spawn(loaded_45_fire_5_btn)
            .observe(
                |t: Trigger<Pointer<Click>>, mut ew: EventWriter<Load45Fire6BtnClick>| {
                    ew.send(Load45Fire6BtnClick(t.entity(), t.hit.depth));
                },
            )
            .with_children(|parent| {
                parent.spawn(loaded_45_fire_5_btn_text);
            });

        parent
            .spawn(loaded69_fire_5_btn)
            .observe(
                |t: Trigger<Pointer<Click>>, mut ew: EventWriter<Load69Fire5BtnClick>| {
                    ew.send(Load69Fire5BtnClick(t.entity(), t.hit.depth));
                },
            )
            .with_children(|parent| {
                parent.spawn(loaded_69_fire_5_btn_text);
            });

        parent
            .spawn(loaded_26_fire_1_btn)
            .observe(
                |t: Trigger<Pointer<Click>>, mut ew: EventWriter<Load26Fire1BtnClick>| {
                    ew.send(Load26Fire1BtnClick(t.entity(), t.hit.depth));
                },
            )
            .with_children(|parent| {
                parent.spawn(loaded_26_fire_1_btn_text);
            });

        parent
            .spawn(custom_game_rule_btn)
            .observe(
                |t: Trigger<Pointer<Click>>, mut ew: EventWriter<CustomGameRuleBtnClick>| {
                    ew.send(CustomGameRuleBtnClick(t.entity(), t.hit.depth));
                },
            )
            .with_children(|parent| {
                parent.spawn(custom_game_rule_img);
                parent.spawn(custom_game_rule_btn_text);
            });

        parent
            .spawn(back_btn)
            .observe(
                |t: Trigger<Pointer<Click>>, mut ew: EventWriter<BackToMainMenuBtnClickEvent>| {
                    ew.send(BackToMainMenuBtnClickEvent(t.entity(), t.hit.depth));
                },
            )
            .with_children(|parent| {
                parent.spawn(back_btn_img);
            });
    });
}

fn spawn_game_menu(
    root_entity: Entity,
    mut commands: Commands,
    my_assets: Res<MyAsstes>,
    game_type: String,
) {
    let wrap = (
        Name::new("wrap"),
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(10.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::End,
            flex_direction: FlexDirection::Row,
            ..default()
        },
    );

    let wrapwrap = Node {
        width: Val::Percent(100.),
        height: Val::Percent(100.),
        justify_content: JustifyContent::End,
        align_items: AlignItems::Center,
        flex_direction: FlexDirection::Column,
        // margin: UiRect::bottom(Val::Percent(40.)),
        margin: UiRect::bottom(Val::Px(100.)),
        ..default()
    };

    let game_type_wrap = Node {
        width: Val::Percent(100.),
        height: Val::Percent(10.),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        flex_direction: FlexDirection::Column,
        ..default()
    };

    let game_type_txt = (
        Text::new(game_type),
        TextFont {
            font: my_assets.ttf_nanum_gothic_bold.clone(),
            ..default()
        },
        TextColor(css::BLACK.into()),
    );

    let ball_shuffle_btn = (
        Name::new("ball_shuffle_btn"),
        BtnInteract,
        Button,
        Node {
            width: Val::Percent(25.),
            height: Val::Percent(100.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::all(Val::Percent(1.)),
            border: UiRect::all(Val::Px(2.)),
            ..default()
        },
        BorderColor(BUTTON_BORDER_COLOR),
        BackgroundColor(BUTTON_BG_COLOR),
        BorderRadius::all(Val::Percent(5.)),
        ShuffleBtn,
    );

    let ball_shuffle_btn_img = (
        Node {
            // width: Val::Percent(100.),
            height: Val::Percent(100.),
            ..default()
        },
        ImageNode::new(my_assets.png_shuffle.clone()),
    );

    let game_run_btn = (
        Name::new("game_run_btn"),
        BtnInteract,
        Button,
        Node {
            width: Val::Percent(25.),
            height: Val::Percent(100.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::all(Val::Percent(1.)),
            border: UiRect::all(Val::Px(2.)),
            ..default()
        },
        BorderColor(BUTTON_BORDER_COLOR),
        BackgroundColor(BUTTON_BG_COLOR),
        BorderRadius::all(Val::Percent(5.)),
        GameRunBtn,
    );
    let game_run_btn_img = (
        Node {
            // width: Val::Percent(100.),
            height: Val::Percent(100.),
            ..default()
        },
        ImageNode::new(my_assets.png_play.clone()),
    );

    let back_btn = (
        Name::new("back_btn"),
        BtnInteract,
        Button,
        Node {
            width: Val::Percent(25.),
            height: Val::Percent(100.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::all(Val::Percent(1.)),
            border: UiRect::all(Val::Px(2.)),
            ..default()
        },
        BorderColor(BUTTON_BORDER_COLOR),
        BackgroundColor(BUTTON_BG_COLOR),
        BorderRadius::all(Val::Percent(5.)),
    );
    let back_btn_img = (
        Node {
            // width: Val::Percent(100.),
            height: Val::Percent(100.),
            ..default()
        },
        ImageNode::new(my_assets.png_back.clone()),
    );

    commands.entity(root_entity).with_children(|parent| {
        parent.spawn(wrapwrap).with_children(|parent| {
            parent.spawn(game_type_wrap).with_children(|parent| {
                parent.spawn(game_type_txt);
            });
            parent.spawn(wrap).with_children(|parent| {
                parent.spawn(back_btn).observe(|t: Trigger<Pointer<Click>>, mut ew: EventWriter<BackToGameRuleSelectBtnClick>| {
                    ew.send(BackToGameRuleSelectBtnClick(t.entity(), t.hit.depth));
                }).with_children(|parent| {
                    parent.spawn(back_btn_img);
                });

                parent
                    .spawn(ball_shuffle_btn)
                    .observe(
                        |t: Trigger<Pointer<Click>>,
                         mut ew: EventWriter<GameMenuShuffleBtnClick>| {
                            ew.send(GameMenuShuffleBtnClick(t.entity(), t.hit.depth));
                        },
                    )
                    .with_children(|parent| {
                        parent.spawn(ball_shuffle_btn_img);
                    });

                parent
                    .spawn(game_run_btn)
                    .observe(
                        |t: Trigger<Pointer<Click>>, mut ew: EventWriter<GameRunBtnClick>| {
                            ew.send(GameRunBtnClick(t.entity(), t.hit.depth));
                        },
                    )
                    .with_children(|parent| {
                        parent.spawn(game_run_btn_img);
                    });
            });
        });
    });
}

fn spawn_numbers_menu(
    root_entity: Entity,
    mut commands: Commands,
    ball_numbers: &Vec<BallNumber>,
    my_assets: Res<MyAsstes>,
) {
    let (paginated_ball_numbers, total_size) = paginate_with_total(&ball_numbers, 0, 5);

    let root_wrap = Node {
        width: Val::Percent(100.),
        height: Val::Percent(100.),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        flex_direction: FlexDirection::Column,
        margin: UiRect::bottom(Val::Px(100.)),
        ..default()
    };

    let wrap = Node {
        width: Val::Percent(80.),
        height: Val::Percent(70.),
        justify_content: JustifyContent::Start,
        align_items: AlignItems::Center,
        flex_direction: FlexDirection::Column,
        ..default()
    };

    let title_wrap = (
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(10.),
            justify_content: JustifyContent::Start,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(css::BISQUE.into()),
    ); 

    let back_btn = (
        Name::new("back_btn"),
        BtnInteract,
        Button,
        Node {
            width: Val::Percent(10.),
            height: Val::Percent(100.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border: UiRect::all(Val::Px(2.)),
            ..default()
        },
        BorderColor(BUTTON_BORDER_COLOR),
        BackgroundColor(BUTTON_BG_COLOR),
        BorderRadius::all(Val::Percent(5.)),
    );
    let back_btn_img = (
        Node {
            // width: Val::Percent(100.),
            height: Val::Percent(50.),
            ..default()
        },
        ImageNode::new(my_assets.png_back.clone()),
    ); 

    let title_txt_wrap = Node {
        width: Val::Percent(80.),
        height: Val::Percent(100.),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let title = (
        Text::new(txt_saved_numbers()),
        TextFont {
            font: my_assets.ttf_nanum_gothic_bold.clone(),
            ..default()
        },
        TextColor(css::BLACK.into()),
    );

    let content_wrap = (
        Name::new("NumbersContentNode"),
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(80.),
            flex_direction: FlexDirection::Column,
            ..default()
        },
        BackgroundColor(css::LIGHT_BLUE.into()),
        NumbersContentNode,
    );

    let paging_wrap = (
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(10.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(css::BISQUE.into()),
    );

    let paging_prev_btn = (
        BtnInteract,
        Button,
        Node {
            width: Val::Percent(20.),
            height: Val::Percent(80.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::all(Val::Percent(1.)),
            border: UiRect::all(Val::Px(2.)),
            ..default()
        },
        BorderColor(BUTTON_BORDER_COLOR),
        BackgroundColor(BUTTON_BG_COLOR),
        BorderRadius::all(Val::Percent(5.)),
    );

    let paging_prev_btn_txt = (
        Text::new("<"),
        TextFont {
            font: my_assets.ttf_nanum_gothic_bold.clone(),
            ..default()
        },
        PickingBehavior::IGNORE,
    );

    let paging_txt_wrap = Node {
        width: Val::Percent(20.),
        height: Val::Percent(100.),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let paging_txt = (
        Text::new(format!("1/{}", if total_size < 1 { 1 } else { total_size })),
        TextFont {
            font: my_assets.ttf_nanum_gothic_bold.clone(),
            ..default()
        },
        TextColor(css::BLACK.into()),
        NumbersPagination {
            now: 0,
            last: total_size,
        },
    );

    let paging_next_btn = (
        BtnInteract,
        Button,
        Node {
            width: Val::Percent(20.),
            height: Val::Percent(80.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::all(Val::Percent(1.)),
            border: UiRect::all(Val::Px(2.)),
            ..default()
        },
        BorderColor(BUTTON_BORDER_COLOR),
        BackgroundColor(BUTTON_BG_COLOR),
        BorderRadius::all(Val::Percent(5.)),
    );

    let paging_next_btn_txt = (
        Text::new(">"),
        TextFont {
            font: my_assets.ttf_nanum_gothic_bold.clone(),
            ..default()
        },
        PickingBehavior::IGNORE,
    );

    let mut content_entity = commands.spawn_empty().id();

    commands.entity(root_entity).with_children(|parent| {
        parent.spawn(root_wrap).with_children(|parent| {
            parent.spawn(wrap).with_children(|parent| {
                parent.spawn(title_wrap).with_children(|parent| {
                    parent.spawn(back_btn).observe(|t: Trigger<Pointer<Click>>, mut ew: EventWriter<BackToMainMenuBtnClickEvent>| {
                        ew.send(BackToMainMenuBtnClickEvent(t.entity(), t.hit.depth));
                    }).with_children(|parent| {
                        parent.spawn(back_btn_img);
                    });

                    parent.spawn(title_txt_wrap).with_children(|parent| {
                        parent.spawn(title);
                    });
                });

                content_entity = parent.spawn(content_wrap).id();

                parent.spawn(paging_wrap).with_children(|parent| {
                    parent.spawn(paging_prev_btn).observe(|t: Trigger<Pointer<Click>>, mut ew: EventWriter<NumbersPagingPrevBtnClick>| {
                        ew.send(NumbersPagingPrevBtnClick(t.entity(), t.hit.depth));
                    }).with_children(|parent| {
                        parent.spawn(paging_prev_btn_txt);
                    });

                    parent.spawn(paging_txt_wrap).with_children(|parent| {
                        parent.spawn(paging_txt);
                    });

                    parent.spawn(paging_next_btn).observe(|t: Trigger<Pointer<Click>>, mut ew: EventWriter<NumbersPagingNextBtnClick>| {
                        ew.send(NumbersPagingNextBtnClick(t.entity(), t.hit.depth));
                    }).with_children(|parent| {
                        parent.spawn(paging_next_btn_txt);
                    });
                });
            });
        });
    });

    spawn_numbers_contents(
        content_entity,
        commands,
        0,
        ball_numbers.len(),
        paginated_ball_numbers,
        my_assets,
    );
}

fn spawn_numbers_contents(
    content_entity: Entity,
    mut commands: Commands,
    page: usize,
    total_item_cnt: usize,
    ball_nums: &[BallNumber],
    my_assets: Res<MyAsstes>,
) {
    let content_item_wrap = (
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(20.),
            border: UiRect::all(Val::Px(1.)),
            ..default()
        },
        BorderColor(css::BLACK.into()),
    );

    let content_item_number_wrap = (
        Node {
            width: Val::Percent(10.),
            height: Val::Percent(100.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            // margin: UiRect::right(Val::Px(1.)),
            border: UiRect::right(Val::Px(1.)),
            ..default()
        },
        BorderColor(css::BLACK.into()),
    ); 

    let content_item_body_wrap =Node {
        width: Val::Percent(70.),
        height: Val::Percent(100.),
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Start,
        margin: UiRect::left(Val::Percent(2.)),
        ..default()
    };
    let content_item_remove_btn = (
        Button,
        Node {
            width: Val::Percent(20.),
            height: Val::Percent(100.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(css::INDIAN_RED.into()),
    );
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
                    time: time_formatting(time.clone(), AppFfi::get_time_offset()),
                })
                .with_children(|parent| {
                    parent
                        .spawn(content_item_number_wrap.clone())
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new((total_item_cnt - (page * 5 + i)).to_string()),
                                TextFont::from_font(my_assets.ttf_nanum_gothic_bold.clone()),
                                TextColor(css::BLACK.into()),
                            ));
                        });
                    parent
                        .spawn(content_item_body_wrap.clone())
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new(format!(
                                    "- {}",
                                    time_formatting(time.clone(), AppFfi::get_time_offset())
                                )),
                                TextFont::from_font(my_assets.ttf_nanum_gothic_bold.clone())
                                    .with_font_size(14.),
                                TextColor(css::BLACK.into()),
                            ));
                            parent.spawn((
                                Text::new(format!("- {}", game_type.clone())),
                                TextFont::from_font(my_assets.ttf_nanum_gothic_bold.clone())
                                    .with_font_size(14.),
                                TextColor(css::BLACK.into()),
                            ));

                            let mut numbers = numbers.clone();
                            numbers.sort();

                            let ten = &numbers[..numbers.len().min(10)];
                            parent.spawn((
                                Text::new(format!("{ten:?}")),
                                TextFont::from_font(my_assets.ttf_nanum_gothic_bold.clone())
                                    .with_font_size(14.),
                                TextColor(css::BLACK.into()),
                            ));

                            if numbers.len() > 10 {
                                let ten = &numbers[10..];

                                parent.spawn((
                                    Text::new(format!("{ten:?}")),
                                    TextFont::from_font(my_assets.ttf_nanum_gothic_bold.clone())
                                        .with_font_size(13.),
                                    TextColor(css::BLACK.into()),
                                ));
                            }
                        });
                    parent
                        .spawn(content_item_remove_btn.clone())
                        .insert(BtnIndianRedInteract)
                        .observe(|t: Trigger<Pointer<Click>>, mut ew: EventWriter<NumbersItemDeleteBtnClick>| {
                            ew.send(NumbersItemDeleteBtnClick(t.entity(), t.hit.depth));
                        })
                        .with_children(|parent| {
                            parent
                                .spawn((
                                    Node {
                                        height: Val::Percent(50.),
                                        ..default()
                                    },ImageNode::new(my_assets.png_trash.clone()),
                                    PickingBehavior::IGNORE,
                                ));
                        });
                });
        }
    });
}

fn spawn_custom_rule_menu(
    root_entity: Entity,
    mut commands: Commands,
    custom_rule: &SavedCustomRule,
    my_assets: Res<MyAsstes>,
) {
    let root_wrap = Node {
        width: Val::Percent(100.),
        height: Val::Percent(100.),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        flex_direction: FlexDirection::Column,
        // margin: UiRect::top(Val::Percent(30.)),
        margin: UiRect::bottom(Val::Px(100.)),
        ..default()
    };

    let wrap = (
        Name::new("wrap"),
        Node {
            width: Val::Percent(80.),
            height: Val::Percent(80.),
            justify_content: JustifyContent::Start,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        BorderRadius::all(Val::Percent(1.)),
        BackgroundColor(css::BEIGE.into()),
    );
    let wrap2 = Node {
        width: Val::Percent(100.),
        height: Val::Percent(80.),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        flex_direction: FlexDirection::Row,
        border: UiRect::all(Val::Px(1.)),
        ..default()
    };
    let wrap3 = Node {
        width: Val::Percent(100.),
        height: Val::Percent(10.),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        flex_direction: FlexDirection::Row,
        border: UiRect::all(Val::Px(1.)),
        ..default()
    };
    let custom_line_1_wrap = (
        Name::new("custom_line_1"),
        Node {
            width: Val::Percent(20.),
            height: Val::Percent(100.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            border: UiRect::all(Val::Px(1.)),
            ..default()
        }
    );

    let circle_1 = (
        Node {
            width: Val::Percent(90.),
            height: Val::Percent(100. * 1. / 14.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border: UiRect::all(Val::Px(1.)),
            ..default()
        },
        BorderColor(css::BLACK.into()),
    );

    let circle_buttons_style = Node {
        width: Val::Percent(100.),
        height: Val::Percent(100.),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let circle_btn = (
        Button,
        circle_buttons_style.clone(),
    );

    let left_btn = (
        Node {
            width: Val::Percent(30.),
            height: Val::Percent(70.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::all(Val::Percent(1.)),
            border: UiRect::all(Val::Px(2.)),
            ..default()
        },
        BorderColor(BUTTON_BORDER_COLOR),
        BackgroundColor(BUTTON_BG_COLOR),
        BorderRadius::all(Val::Percent(5.)),
    );

    let left_btn_txt = make_text("<");

    let fire_cnt_txt = (
        Text::new(custom_rule.fire.to_string()),
        TextFont::from_font_size(30.),
        TextColor(css::BLACK.into()),
    );

    let right_btn = (
        Node {
            width: Val::Percent(30.),
            height: Val::Percent(70.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::all(Val::Percent(1.)),
            border: UiRect::all(Val::Px(2.)),
            ..default()
        },
        BorderColor(BUTTON_BORDER_COLOR),
        BackgroundColor(BUTTON_BG_COLOR),
        BorderRadius::all(Val::Percent(5.)),
    ); 
    let right_btn_txt = make_text(">");

    let back_run_wrap =  Node {
        width: Val::Percent(100.),
        height: Val::Percent(10.),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        flex_direction: FlexDirection::Row,
        border: UiRect::all(Val::Px(1.)),
        ..default()
    };

    let back_btn = (
        Name::new("back_btn"),
        Node {
            width: Val::Percent(90.),
            height: Val::Percent(90.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::all(Val::Percent(1.)),
            border: UiRect::all(Val::Px(2.)),
            ..default()
        },
        BorderColor(BUTTON_BORDER_COLOR),
        BackgroundColor(BUTTON_BG_COLOR),
        BorderRadius::all(Val::Percent(5.)),
    );

    let back_btn_img = (
        Node {
            // width: Val::Percent(100.),
            height: Val::Percent(100.),
            ..default()
        },
        ImageNode::new(my_assets.png_back.clone()),
    ); 
    let run_btn = (
        Name::new("back_btn"),
        Button,
        Node {
            width: Val::Percent(90.),
            height: Val::Percent(90.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::all(Val::Percent(1.)),
            border: UiRect::all(Val::Px(2.)),
            ..default()
        },
        BorderColor(BUTTON_BORDER_COLOR),
        BackgroundColor(BUTTON_BG_COLOR),
        BorderRadius::all(Val::Percent(5.)),
    );
    let run_btn_text = (
        Name::new("back_btn_text"),
        Text::new(txt_ok()),
        TextFont::from_font(my_assets.ttf_nanum_gothic_bold.clone()),
        PickingBehavior::IGNORE,
    );
    commands.entity(root_entity).with_children(|parent| {
        parent.spawn(root_wrap).with_children(|parent| {
            parent.spawn(wrap).with_children(|parent| {
                parent.spawn(
                    (
                        Text::new(txt_insert_balls()),
                        TextFont::from_font(my_assets.ttf_nanum_gothic_bold.clone()),
                        TextColor(css::BLACK.into())
                    )
                );
                parent.spawn(wrap2).with_children(|parent| {
                    let ranges = [(1..=14), (15..=28), (29..=42), (43..=56), (57..=70)];
                    for r in ranges {
                        parent
                            .spawn(custom_line_1_wrap.clone())
                            .with_children(|parent| {
                                for i in r {
                                    let custom_ball = custom_rule.load[i - 1].clone();
                                    let ox = if custom_ball.1 { "[v]" } else { "[ ]" };
                                    let bg_col = if custom_ball.1 {
                                        BackgroundColor(css::BLUE.into())
                                    } else {
                                        BackgroundColor(css::LIGHT_BLUE.into())
                                    };
                                    parent.spawn(circle_1.clone()).with_children(|parent| {
                                        parent
                                            .spawn(circle_btn.clone())
                                            .insert(bg_col)
                                            .insert(custom_ball)
                                            .observe(|t: Trigger<Pointer<Click>>, mut ew: EventWriter<CustomRuleBallClick>| {
                                                ew.send(CustomRuleBallClick(t.entity(), t.hit.depth));
                                            }) 
                                            // .insert(On::<Pointer<Down>>::send_event::<
                                            //     CustomRuleBallClick,
                                            // >())
                                            .with_children(|parent| {
                                                parent.spawn((Text::new(i.to_string()), TextFont::from_font(my_assets
                                                    .ttf_nanum_gothic_bold
                                                    .clone()).with_font_size(16.))).insert(PickingBehavior::IGNORE);
                                                parent.spawn((Text::new(ox.to_string()), TextFont::from_font(my_assets
                                                    .ttf_nanum_gothic_bold
                                                    .clone()).with_font_size(16.))).insert(PickingBehavior::IGNORE);
                                           
                                    });
                                });
                            }
                            });
                    }
                });
                parent.spawn(
                    (
                        Text::new(txt_draw_balls_count()),
                        TextFont::from_font(my_assets.ttf_nanum_gothic_bold.clone()),
                        TextColor(css::BLACK.into())
                    )
                );
                parent.spawn(wrap3).with_children(|parent| {
                    // <
                    parent
                        .spawn(left_btn)
                        .observe(|t: Trigger<Pointer<Click>>, mut ew: EventWriter<CustomRuleFireCntDownClick>| {
                            ew.send(CustomRuleFireCntDownClick(t.entity(), t.hit.depth));
                        })
                        // .insert(On::<Pointer<Click>>::send_event::<CustomRuleFireCntDownClick>())
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
                        .observe(|t: Trigger<Pointer<Click>>, mut ew: EventWriter<CustomRuleFireCntUpClick>| {
                            ew.send(CustomRuleFireCntUpClick(t.entity(), t.hit.depth));
                        })
                        // .insert(On::<Pointer<Click>>::send_event::<CustomRuleFireCntUpClick>())
                        .with_children(|parent| {
                            parent.spawn(right_btn_txt);
                        });
                });

                parent.spawn(back_run_wrap).with_children(|parent| {
                    // back
                    parent.spawn(back_btn).observe(|t: Trigger<Pointer<Click>>, mut ew: EventWriter<BackToGameRuleSelectBtnClick>| {
                        ew.send(BackToGameRuleSelectBtnClick(t.entity(), t.hit.depth));
                    }).with_children(|parent| {
                        parent.spawn(back_btn_img);
                    });
                    // run
                    parent.spawn(run_btn).observe(|t: Trigger<Pointer<Click>>, mut ew: EventWriter<CustomRuleRunBtnClick>| {
                        ew.send(CustomRuleRunBtnClick(t.entity(), t.hit.depth));
                    }).with_children(|parent| {
                        parent.spawn(run_btn_text);
                    });
                });
            });
        });
    });
}

fn spawn_game_result_menu(
    root_entity: Entity,
    mut commands: Commands,
    picked_numbers: &[u8],
    my_assets: Res<MyAsstes>,
) {
    // retry_btn <- x
    // back_btn
    // result
    // save btn

    let wrap = Node {
        width: Val::Percent(100.),
        height: Val::Percent(100.),
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::End,
        align_items: AlignItems::Center,
        // margin: UiRect::bottom(Val::Percent(40.)),
        margin: UiRect::bottom(Val::Px(100.)),
        ..default()
    };
    let result_wrap = Node {
        width: Val::Percent(100.),
        height: Val::Percent(10.),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let result_txt = 
    (
        Text::new(format!("{picked_numbers:?}")),
        TextFont::from_font(my_assets.ttf_nanum_gothic_bold.clone()).with_font_size(20.),
        TextColor(css::BLACK.into())
    );
    

    let btn_wrap =  Node {
        width: Val::Percent(100.),
        height: Val::Percent(10.),
        flex_direction: FlexDirection::Row,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    // let retry_btn = (
    //     ButtonBundle {
    //         style: Style {
    //             width: Val::Percent(20.),
    //             height: Val::Percent(100.),
    //             justify_content: JustifyContent::Center,
    //             align_items: AlignItems::Center,
    //             ..default()
    //         },
    //         ..default()
    //     },
    //     On::<Pointer<Click>>::send_event::<GameResultRetryBtnClick>(),
    // );
    // let retry_btn_txt = (TextBundle::from_section("retry", TextStyle::default()));
    let save_btn = (
        BtnInteract,
        Button,
        Node {
            width: Val::Percent(30.),
            height: Val::Percent(100.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::all(Val::Percent(1.)),
            border: UiRect::all(Val::Px(2.)),
            ..default()
        },
        BorderColor(BUTTON_BORDER_COLOR),
        BackgroundColor(BUTTON_BG_COLOR),
        BorderRadius::all(Val::Percent(5.)),
        SaveBtn,
    );
    let save_btn_img = (
        Node {
            // width: Val::Percent(100.),
            height: Val::Percent(100.),
            ..default()
        },
        ImageNode::new(my_assets.png_save.clone()),
    ); 

    let back_btn = (
        BtnInteract,
        Button,
        Node {
            width: Val::Percent(30.),
            height: Val::Percent(100.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::all(Val::Percent(1.)),
            border: UiRect::all(Val::Px(2.)),
            ..default()
        },
        BorderColor(BUTTON_BORDER_COLOR),
        BackgroundColor(BUTTON_BG_COLOR),
        BorderRadius::all(Val::Percent(5.)),
    );

    let back_btn_img = (
        Node {
            // width: Val::Percent(100.),
            height: Val::Percent(100.),
            ..default()
        },
        ImageNode::new(my_assets.png_back.clone()),
    );

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

                parent.spawn(back_btn).observe(|t: Trigger<Pointer<Click>>, mut ew: EventWriter<BackToGameRuleSelectBtnClick>| {
                    ew.send(BackToGameRuleSelectBtnClick(t.entity(), t.hit.depth));
                }).with_children(|parent| {
                    parent.spawn(back_btn_img);
                });

                parent.spawn(save_btn).observe(|t: Trigger<Pointer<Click>>, mut ew: EventWriter<GameResultSaveBtnClick>| {
                    ew.send(GameResultSaveBtnClick(t.entity(), t.hit.depth));
                }).with_children(|parent| {
                    parent.spawn(save_btn_img);
                });
            });
        });
    });
}

pub fn setup_main_ui(mut commands: Commands, my_assets: Res<MyAsstes>) {
    let root_node = (
        Name::new("root_node"),
        RootNode,
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
    );

    let root_entity = commands.spawn(root_node).id();
    spawn_main_menu(root_entity, commands, my_assets);
}

pub fn button_interaction(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<BtnInteract>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Hovered => {
                *color = BUTTON_HOVER_COLOR.into();
            }
            Interaction::Pressed => {
                *color = BUTTON_CLICK_COLOR.into();
            }
            Interaction::None => {
                *color = BUTTON_BG_COLOR.into();
            }
        }
    }
}

pub fn button_indian_red_interaction(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<BtnIndianRedInteract>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Hovered => {
                *color = BUTTON_INDIAN_RED_HOVER_COLOR.into();
            }
            Interaction::Pressed => {
                *color = BUTTON_INDIAN_RED_CLICK_COLOR.into();
            }
            Interaction::None => {
                *color = css::INDIAN_RED.into();
            }
        }
    }
}

pub fn game_rule_select_button_click(
    mut commands: Commands,
    q_root_node: Query<(Entity, &Children), With<RootNode>>,
    mut er: EventReader<GameRuleSelectButtonClickEvent>,
    my_assets: Res<MyAsstes>,
    ui_config: Res<UiConfig>,
) {
    for _ in er.read() {
        if let Ok((entity, children)) = q_root_node.get_single() {
            for &entity in children.iter() {
                commands.entity(entity).despawn_recursive();
            }

            let load = ui_config
                .saved_custom_rule
                .load
                .iter()
                .filter(|f| f.1)
                .count();
            let fire = ui_config.saved_custom_rule.fire;
            let custom_type = format!("{fire}/{load}");
            return spawn_game_rule_select_menu(entity, commands, my_assets, custom_type);
        }
    }
}

pub fn back_to_main_menu_btn_click(
    mut commands: Commands,
    mut er: EventReader<BackToMainMenuBtnClickEvent>,
    q_root_node: Query<(Entity, &Children), With<RootNode>>,
    my_assets: Res<MyAsstes>,
) {
    for _ in er.read() {
        if let Ok((entity, children)) = q_root_node.get_single() {
            for &entity in children.iter() {
                commands.entity(entity).despawn_recursive();
            }

            spawn_main_menu(entity, commands, my_assets);
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
    my_assets: Res<MyAsstes>,
) {
    for _ in er.read() {
        if let Ok((entity, children)) = q_root_node.get_single() {
            for &entity in children.iter() {
                commands.entity(entity).despawn_recursive();
            }

            config.rule_given_ball = make_given_ball(ball69());
            config.rule_taken_ball = 5;
            ew.send(BallSpawnEvent(false));
            let game_type = "5/69".to_owned();

            return spawn_game_menu(entity, commands, my_assets, game_type);
        }
    }
}

pub fn loaded_26_fire_1_btn_click(
    mut commands: Commands,
    mut er: EventReader<Load26Fire1BtnClick>,
    q_root_node: Query<(Entity, &Children), With<RootNode>>,
    mut config: ResMut<GameConfig>,
    mut ew: EventWriter<BallSpawnEvent>,
    my_assets: Res<MyAsstes>,
) {
    for _ in er.read() {
        if let Ok((entity, children)) = q_root_node.get_single() {
            for &entity in children.iter() {
                commands.entity(entity).despawn_recursive();
            }

            config.rule_given_ball = make_given_ball(ball26());
            config.rule_taken_ball = 1;
            ew.send(BallSpawnEvent(false));
            let game_type = "1/26".to_owned();

            return spawn_game_menu(entity, commands, my_assets, game_type);
        }
    }
}

pub fn loaded_45_fire_6_btn_click(
    mut commands: Commands,
    mut er: EventReader<Load45Fire6BtnClick>,
    q_root_node: Query<(Entity, &Children), With<RootNode>>,
    mut config: ResMut<GameConfig>,
    mut ew: EventWriter<BallSpawnEvent>,
    my_assets: Res<MyAsstes>,
) {
    for _ in er.read() {
        if let Ok((entity, children)) = q_root_node.get_single() {
            for &entity in children.iter() {
                commands.entity(entity).despawn_recursive();
            }

            config.rule_given_ball = make_given_ball(ball45());
            config.rule_taken_ball = 6;
            ew.send(BallSpawnEvent(false));
            let game_type = "6/45".to_owned();

            return spawn_game_menu(entity, commands, my_assets, game_type);
        }
    }
}

pub fn custom_game_rule_btn_click(
    mut commands: Commands,
    mut er: EventReader<CustomGameRuleBtnClick>,
    q_root_node: Query<(Entity, &Children), With<RootNode>>,
    ui_config: Res<UiConfig>,
    my_assets: Res<MyAsstes>,
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

            return spawn_custom_rule_menu(
                entity,
                commands,
                &ui_config.saved_custom_rule,
                my_assets,
            );
        }
    }
}

pub fn back_to_game_rule_select_btn_click(
    mut commands: Commands,
    mut er: EventReader<BackToGameRuleSelectBtnClick>,
    q_root_node: Query<(Entity, &Children), With<RootNode>>,
    mut ew: EventWriter<BallClearEvent>,
    mut config: ResMut<GameConfig>,
    my_assets: Res<MyAsstes>,
    ui_config: Res<UiConfig>,
    mut ew_step_start: EventWriter<GameStepStartEvent>,
    mut ew_draw_stick_reset: EventWriter<DrawStickResetEvent>,
    q_rt: Query<Entity, With<RemixerTimer>>,
    q_ret: Query<Entity, With<RemixerEndTimer>>,
    q_rjt: Query<Entity, With<RemixerJudgeTimer>>,
) {
    for _ in er.read() {
        if let Ok((entity, children)) = q_root_node.get_single() {
            for &entity in children.iter() {
                commands.entity(entity).despawn_recursive();
            }

            config.is_running = false;
            config.rule_given_ball = make_given_ball(ball70());
            config.rule_taken_ball = 5;
            config.picked_ball = vec![];
            config.is_ball_release_sensor = false;
            config.is_pool_ball_cnt_sensor = false;
            config.is_catching = false;
            for entity in &q_rt {
                commands.entity(entity).despawn_recursive();
            }

            for entity in &q_ret {
                commands.entity(entity).despawn_recursive();
            }

            for entity in &q_rjt {
                commands.entity(entity).despawn_recursive();
            }
            ew.send(BallClearEvent);

            let load = ui_config
                .saved_custom_rule
                .load
                .iter()
                .filter(|f| f.1)
                .count();
            let fire = ui_config.saved_custom_rule.fire;
            let custom_type = format!("{fire}/{load}");

            ew_draw_stick_reset.send(DrawStickResetEvent);
            ew_step_start.send(GameStepStartEvent::new_with_data(
                STEP_BALL_MIXER_ROTATE,
                GameStepData::Float(1.),
            ));
            return spawn_game_rule_select_menu(entity, commands, my_assets, custom_type);
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

// pub fn resize_text_based_on_window(
//     // mut query: Query<&mut Text, With<TextResize>>,
//     mut resize_reader: EventReader<WindowResized>,
// ) {
//     for _ in resize_reader.read() {
//         // return;
//         // for mut t in &mut query {
//         //     for t2 in t.sections.iter_mut() {
//         //         t2.style.font_size = e.width * 0.03;
//         //     }
//         // }
//     }
// }

pub fn custom_rule_run_btn_click(
    mut commands: Commands,
    mut er: EventReader<CustomRuleRunBtnClick>,
    q_root_node: Query<(Entity, &Children), With<RootNode>>,
    mut config: ResMut<GameConfig>,
    mut ew: EventWriter<BallSpawnEvent>,
    q_custom_balls: Query<&CustomRuleBall>,
    q_firecnt: Query<&CustomRuleFireCnt>,
    my_assets: Res<MyAsstes>,
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

            let game_type = format!(
                "{}/{}",
                config.rule_taken_ball,
                config.rule_given_ball.len()
            );
            return spawn_game_menu(entity, commands, my_assets, game_type);
        }
    }
}

pub fn custom_rule_ball_click(
    mut er: EventReader<CustomRuleBallClick>,
    mut q_custom_ball: Query<(&mut CustomRuleBall, &mut BackgroundColor)>,
    q_firecnt: Query<&CustomRuleFireCnt>,
    q_child: Query<&Children>,
    mut q_text: Query<&mut Text>,
    mut ui_config: ResMut<UiConfig>,
) {
    for evt in er.read() {
        info!("custom_rule_ball_click");

        // 공 넣기는 뽑을공갯수와 같거나 커야만 한다.
        let mut loaded_ball_cnt = 0;
        for (cb, ..) in &q_custom_ball {
            if cb.1 {
                loaded_ball_cnt += 1;
            }
        }
        if let Ok(fc) = q_firecnt.get_single() {
            // todo!: 음 일단 그냥 발리데션은 하지말고 그냥 진행
            if true || loaded_ball_cnt >= fc.0 {
                // is use 꺽고 색, 텍스트 변경후  저장하기
                if let Ok((mut cb, mut bg)) = q_custom_ball.get_mut(evt.0) {
                    cb.1 = !cb.1;

                    ui_config.saved_custom_rule.load[(cb.0 - 1) as usize].1 = cb.1;
                    ui_config.saved_custom_rule.save_custom_rule();

                    if let Ok(child) = q_child.get(evt.0) {
                        if cb.1 {
                            let mut text = q_text.get_mut(child[1]).unwrap();
                            text.0 = "[v]".to_string();
                            
                        } else {
                            let mut text = q_text.get_mut(child[1]).unwrap();
                            text.0 = "[ ]".to_string();
                        }
                        // if let Ok(mut txt) = q_text.get_mut(child[0]) {
                        //     if cb.1 {
                        //         txt.sections[1].value = "[v]".to_string();
                        //     } else {
                        //         txt.sections[1].value = "[ ]".to_string();
                        //     }
                        // }
                    }
                    if cb.1 {
                        *bg = BackgroundColor(css::BLUE.into());
                    } else {
                        *bg = BackgroundColor(css::LIGHT_BLUE.into());
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

                text.0 = fc.0.to_string();
            }
        }
    }
}

pub fn custom_rule_fire_up_click(
    mut er: EventReader<CustomRuleFireCntUpClick>,
    mut q_firecnt: Query<(&mut CustomRuleFireCnt, &mut Text)>,
    // q_custom_balls: Query<&CustomRuleBall>,
    mut ui_config: ResMut<UiConfig>,
) {
    for _ in er.read() {
        info!("custom_rule_fire_up_click");
        if let Ok((mut fc, mut text)) = q_firecnt.get_single_mut() {
            // todo: 일단 발리데이션 하지말자
            // let mut cnt = 0;
            // for cb in &q_custom_balls {
            //     if cb.1 {
            //         cnt += 1;
            //     }
            // }

            // if cnt > 20 {
            //     cnt = 20;
            // }

            // if fc.0 < cnt {
            //     fc.0 += 1;

            //     ui_config.saved_custom_rule.fire = fc.0;
            //     ui_config.saved_custom_rule.save_custom_rule();

            //     text.sections[0].value = fc.0.to_string();
            // }

            if fc.0 < 20 {
                fc.0 += 1;

                ui_config.saved_custom_rule.fire = fc.0;
                ui_config.saved_custom_rule.save_custom_rule();

                text.0 = fc.0.to_string();
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
            // quit_app();
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
    my_assets: Res<MyAsstes>,
) {
    for _ in er.read() {
        if let Ok((entity, children)) = q_root_node.get_single() {
            for &entity in children.iter() {
                commands.entity(entity).despawn_recursive();
            }
            let mut ball_numbers = ui_config.saved_ball_numbers.clone();
            ball_numbers.sort_by(|a, b| b.time.cmp(&a.time));
            return spawn_numbers_menu(entity, commands, &ball_numbers, my_assets);
        }
    }
}

pub fn numbers_paging_prev_click(
    mut commands: Commands,
    mut er: EventReader<NumbersPagingPrevBtnClick>,
    q_content_node: Query<(Entity, &Children), With<NumbersContentNode>>,
    ui_config: Res<UiConfig>,
    mut q_number_pagination: Query<(&mut NumbersPagination, &mut Text)>,
    my_assets: Res<MyAsstes>,
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

                text.0 = format!("{}/{}", pagination.now + 1, pagination.last);

                let mut ball_numbers = ui_config.saved_ball_numbers.clone();
                ball_numbers.sort_by(|a, b| b.time.cmp(&a.time));

                let (ball_nums, ..) = paginate_with_total(&ball_numbers, pagination.now, 5);
                return spawn_numbers_contents(
                    entity,
                    commands,
                    pagination.now,
                    ui_config.saved_ball_numbers.len(),
                    ball_nums,
                    my_assets,
                );
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
    my_assets: Res<MyAsstes>,
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

                text.0 = format!("{}/{}", pagination.now + 1, pagination.last);

                let mut ball_numbers = ui_config.saved_ball_numbers.clone();
                ball_numbers.sort_by(|a, b| b.time.cmp(&a.time));

                let (ball_nums, ..) = paginate_with_total(&ball_numbers, pagination.now, 5);
                return spawn_numbers_contents(
                    entity,
                    commands,
                    pagination.now,
                    ui_config.saved_ball_numbers.len(),
                    ball_nums,
                    my_assets,
                );
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
    my_assets: Res<MyAsstes>,
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
                    let mut ball_numbers = ui_config.saved_ball_numbers.clone();
                    ball_numbers.sort_by(|a, b| b.time.cmp(&a.time));

                    let (_ball_nums, total_size) =
                        paginate_with_total(&ball_numbers, pagination.now, 5);

                    pagination.last = total_size;
                    info!("pagination {pagination:?}");
                    if pagination.now + 1 > total_size {
                        pagination.now = total_size - 1;
                    }
                    let (ball_nums, ..) = paginate_with_total(&ball_numbers, pagination.now, 5);

                    text.0 = format!(
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
                    return spawn_numbers_contents(
                        entity,
                        commands,
                        pagination.now,
                        ui_config.saved_ball_numbers.len(),
                        ball_nums,
                        my_assets,
                    );
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
    my_assets: Res<MyAsstes>,
    // mut ew_step_start: EventWriter<GameStepStartEvent>,
) {
    for _ in er.read() {
        if let Ok((entity, children)) = q_root_node.get_single() {
            for &entity in children.iter() {
                commands.entity(entity).despawn_recursive();
            }

            return spawn_game_result_menu(entity, commands, &config.picked_ball, my_assets);
        }
    }
}

pub fn game_result_menu_retry_btn_click(
    mut commands: Commands,
    mut er: EventReader<GameResultRetryBtnClick>,
    q_root_node: Query<(Entity, &Children), With<RootNode>>,
    my_assets: Res<MyAsstes>,
    config: Res<GameConfig>,
) {
    for _ in er.read() {
        if let Ok((entity, children)) = q_root_node.get_single() {
            for &entity in children.iter() {
                commands.entity(entity).despawn_recursive();
            }
            let game_type = format!(
                "{}/{}",
                config.rule_taken_ball,
                config.rule_given_ball.len()
            );
            return spawn_game_menu(entity, commands, my_assets, game_type);
        }
    }
}

pub fn game_result_menu_save_btn_click(
    mut commands: Commands,
    mut er: EventReader<GameResultSaveBtnClick>,
    q_save_btn: Query<Entity, With<SaveBtn>>,
    config: Res<GameConfig>,
    mut ui_config: ResMut<UiConfig>,
) {
    for _ in er.read() {
        let numbers = &config.picked_ball;
        let game_type = format!(
            "{}/{}",
            config.rule_taken_ball,
            config.rule_given_ball.len()
        );
        let ball_num = BallNumber {
            game_type,
            id: Uuid::new_v4().to_string(),
            numbers: numbers.clone(),
            time: AppFfi::get_current_epoch_time(),
        };

        ui_config.saved_ball_numbers.save_item(ball_num);

        if let Ok(entity) = q_save_btn.get_single() {
            commands.entity(entity).insert(Visibility::Hidden);
        }
    }
}

/// Main State 진입시 배너 광고 오픈
pub fn admob_banner_show(mut ew: EventWriter<AdmobBannerLaunch>) {
    ew.send(AdmobBannerLaunch);
}

pub fn app_init(mut ew: EventWriter<AppInitStartEvent>) {
    ew.send(AppInitStartEvent);
}

pub fn splash_hide() {
    AppFfi::splash_hide();
}
