use bevy::prelude::*;
use event::{
    BackToGameRuleSelectBtnClick, BackToMainMenuBtnClickEvent, ButtonClickEvent,
    CustomGameRuleBtnClick, CustomRuleBallClick, CustomRuleFireCntDownClick,
    CustomRuleFireCntUpClick, CustomRuleRunBtnClick, GameMenuShuffleBtnClick,
    GameResultRetryBtnClick, GameResultSaveBtnClick, GameRuleSelectButtonClickEvent,
    GameRunBtnClick, Load26Fire1BtnClick, Load45Fire6BtnClick, Load69Fire5BtnClick,
    NumbersBtnClick, NumbersItemDeleteBtnClick, NumbersPagingNextBtnClick,
    NumbersPagingPrevBtnClick, QuitBtnClick,
};
use resource::UiConfig;
use system::{
    back_to_game_rule_select_btn_click, back_to_main_menu_btn_click, button_indian_red_interaction,
    button_interaction, custom_game_rule_btn_click, custom_rule_ball_click,
    custom_rule_fire_down_click, custom_rule_fire_up_click, custom_rule_run_btn_click, er_game_end,
    game_menu_shuffle_btn_click, game_result_menu_retry_btn_click, game_result_menu_save_btn_click,
    game_rule_select_button_click, game_run_btn_click, loaded_26_fire_1_btn_click,
    loaded_45_fire_6_btn_click, loaded_69_fire_5_btn_click, numbers_btn_click,
    numbers_item_delete_btn_click, numbers_paging_next_click, numbers_paging_prev_click,
    quit_btn_click, resize_text_based_on_window, setup_main_ui,
};

use crate::app::states::MyStates;
pub mod component;
pub mod constant;
pub mod event;
pub mod i18n;
pub mod resource;
pub mod system;
pub mod utils;
pub struct MyUiPlugin;

impl Plugin for MyUiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(UiConfig::new())
            .add_event::<ButtonClickEvent>()
            .add_event::<GameRuleSelectButtonClickEvent>()
            .add_event::<BackToMainMenuBtnClickEvent>()
            .add_event::<Load69Fire5BtnClick>()
            .add_event::<Load26Fire1BtnClick>()
            .add_event::<Load45Fire6BtnClick>()
            .add_event::<CustomGameRuleBtnClick>()
            .add_event::<BackToGameRuleSelectBtnClick>()
            .add_event::<GameRunBtnClick>()
            .add_event::<CustomRuleBallClick>()
            .add_event::<CustomRuleFireCntDownClick>()
            .add_event::<CustomRuleFireCntUpClick>()
            .add_event::<CustomRuleRunBtnClick>()
            .add_event::<NumbersBtnClick>()
            .add_event::<QuitBtnClick>()
            .add_event::<NumbersPagingPrevBtnClick>()
            .add_event::<NumbersPagingNextBtnClick>()
            .add_event::<NumbersItemDeleteBtnClick>()
            .add_event::<GameMenuShuffleBtnClick>()
            .add_event::<GameResultRetryBtnClick>()
            .add_event::<GameResultSaveBtnClick>()
            .add_systems(OnEnter(MyStates::Game), setup_main_ui)
            .add_systems(
                Update,
                (
                    button_interaction,
                    game_rule_select_button_click,
                    back_to_main_menu_btn_click,
                    back_to_game_rule_select_btn_click,
                    loaded_69_fire_5_btn_click,
                    loaded_26_fire_1_btn_click,
                    loaded_45_fire_6_btn_click,
                    custom_game_rule_btn_click,
                    game_run_btn_click,
                    resize_text_based_on_window,
                    custom_rule_run_btn_click,
                    custom_rule_ball_click,
                    custom_rule_fire_down_click,
                    custom_rule_fire_up_click,
                    quit_btn_click,
                    numbers_btn_click,
                    numbers_paging_prev_click,
                    numbers_paging_next_click,
                    numbers_item_delete_btn_click,
                    (
                        game_menu_shuffle_btn_click,
                        er_game_end,
                        game_result_menu_retry_btn_click,
                        game_result_menu_save_btn_click,
                        button_indian_red_interaction,
                    ),
                )
                    .run_if(in_state(MyStates::Game)),
            );
    }
}
