use bevy::prelude::*;
use event::{
    BackToGameRuleSelectBtnClick, BackToMainMenuBtnClickEvent, ButtonClickEvent,
    CustomGameRuleBtnClick, CustomRuleBallClick, CustomRuleFireCntDownClick,
    CustomRuleFireCntUpClick, CustomRuleRunBtnClick, GameRuleSelectButtonClickEvent,
    GameRunBtnClick, Load26Fire1BtnClick, Load45Fire6BtnClick, Load69Fire5BtnClick,
};
use system::{
    back_to_game_rule_select_btn_click, back_to_main_menu_btn_click, button_interaction,
    custom_game_rule_btn_click, custom_rule_ball_click, custom_rule_fire_down_click,
    custom_rule_fire_up_click, custom_rule_run_btn_click, game_rule_select_button_click,
    game_run_btn_click, loaded_26_fire_1_btn_click, loaded_45_fire_6_btn_click,
    loaded_69_fire_5_btn_click, resize_text_based_on_window, setup_main_ui,
};

pub mod component;
pub mod event;
pub mod system;
pub mod system_test;
pub mod utils;
pub struct MyUiPlugin;

impl Plugin for MyUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ButtonClickEvent>()
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
            // .add_systems(Startup, setup_test_ui)
            // .add_systems(Update, (test_button_interaction, test_button_click))
            .add_systems(Startup, setup_main_ui)
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
                ),
            );
    }
}
