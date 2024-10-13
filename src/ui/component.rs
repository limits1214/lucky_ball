use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component)]
pub struct TestBtn;

#[derive(Component)]
pub struct RootNode;

#[derive(Component)]
pub struct GameBtn;

#[derive(Component)]
pub struct QuitBtn;

#[derive(Component)]
pub struct NumbersBtn;

#[derive(Component)]
pub struct TextResize;

/// tuple 0(u8): ball_number
/// tuple 1(bool): is_use
#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct CustomRuleBall(pub u8, pub bool);

#[derive(Component)]
pub struct CustomRuleFireCnt(pub u8);

#[derive(Component)]
pub struct NumbersContentNode;

#[derive(Component, Debug)]
pub struct NumbersPagination {
    pub now: usize,
    pub last: usize,
}
#[derive(Component, Debug)]
pub struct NumbersItem {
    pub id: String,
    pub numbers: Vec<u8>,
    pub game_type: String,
    pub time: String,
}

#[derive(Component)]
pub struct ShuffleBtn;

#[derive(Component)]
pub struct SaveBtn;

#[derive(Component)]
pub struct GameRunBtn;

#[derive(Component)]
pub struct BtnInteract;
#[derive(Component)]
pub struct BtnIndianRedInteract;
