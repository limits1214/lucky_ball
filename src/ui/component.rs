use bevy::prelude::*;

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

#[derive(Component)]
pub struct CustomRuleBall(pub u8, pub bool);

#[derive(Component)]
pub struct CustomRuleFireCnt(pub u8);
