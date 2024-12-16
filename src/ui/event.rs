use bevy::prelude::*;

#[derive(Event)]
pub struct ButtonClickEvent(pub Entity, pub f32);

#[derive(Event, Debug)]
pub struct GameRuleSelectButtonClickEvent(pub Entity, pub f32);

#[derive(Event, Debug)]
pub struct BackToMainMenuBtnClickEvent(pub Entity, pub f32);

#[derive(Event, Debug)]
pub struct Load69Fire5BtnClick(pub Entity, pub f32);

#[derive(Event, Debug)]
pub struct Load26Fire1BtnClick(pub Entity, pub f32);

#[derive(Event, Debug)]
pub struct Load45Fire6BtnClick(pub Entity, pub f32);

#[derive(Event, Debug)]
pub struct CustomGameRuleBtnClick(pub Entity, pub f32);

#[derive(Event, Debug)]
pub struct BackToGameRuleSelectBtnClick(pub Entity, pub f32);

#[derive(Event, Debug)]
pub struct GameRunBtnClick(pub Entity, pub f32);

#[derive(Event, Debug)]
pub struct CustomRuleBallClick(pub Entity, pub f32);

#[derive(Event, Debug)]
pub struct CustomRuleFireCntDownClick(pub Entity, pub f32);

#[derive(Event, Debug)]
pub struct CustomRuleFireCntUpClick(pub Entity, pub f32);

#[derive(Event, Debug)]
pub struct CustomRuleRunBtnClick(pub Entity, pub f32);

#[derive(Event, Debug)]
pub struct NumbersBtnClick(pub Entity, pub f32);

#[derive(Event, Debug)]
pub struct QuitBtnClick(pub Entity, pub f32);

#[derive(Event, Debug)]
pub struct NumbersPagingPrevBtnClick(pub Entity, pub f32);

#[derive(Event, Debug)]
pub struct NumbersPagingNextBtnClick(pub Entity, pub f32);

#[derive(Event, Debug)]
pub struct NumbersItemDeleteBtnClick(pub Entity, pub f32);

#[derive(Event, Debug)]
pub struct GameMenuShuffleBtnClick(pub Entity, pub f32);

#[derive(Event, Debug)]
pub struct GameResultRetryBtnClick(pub Entity, pub f32);

#[derive(Event, Debug)]
pub struct GameResultSaveBtnClick(pub Entity, pub f32);
