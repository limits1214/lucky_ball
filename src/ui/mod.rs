use bevy::prelude::*;
use event::{
    ButtonClickEvent, GameBackButtonClickEvent, GameButtonClickEvent, Load70Fire5BtnClick,
    RuleSelectBackBtnClick,
};
use system::{
    game_back_btn_click, game_button_click, main_button_click, main_button_interaction,
    setup_main_ui,
};

pub mod component;
pub mod event;
pub mod system;
pub mod system_test;
pub struct MyUiPlugin;

impl Plugin for MyUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ButtonClickEvent>()
            .add_event::<GameButtonClickEvent>()
            .add_event::<GameBackButtonClickEvent>()
            .add_event::<Load70Fire5BtnClick>()
            .add_event::<RuleSelectBackBtnClick>()
            // .add_systems(Startup, setup_test_ui)
            // .add_systems(Update, (test_button_interaction, test_button_click))
            .add_systems(Startup, setup_main_ui)
            .add_systems(
                Update,
                (
                    main_button_interaction,
                    main_button_click,
                    game_button_click,
                    game_back_btn_click,
                ),
            );
    }
}
