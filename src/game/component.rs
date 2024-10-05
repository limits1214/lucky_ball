use bevy::prelude::*;

#[derive(Component)]
pub struct BallMixer;

#[derive(Component)]
pub struct BallDrawStick;

#[derive(Component)]
pub struct BallCatchSensor;

#[derive(Component)]
pub struct PoolBallCntSensor;

#[derive(Component)]
pub struct BallDrawStickIn;

#[derive(Component, Debug)]
pub struct Ball(pub u8);

#[derive(Component)]
pub struct PoolOutletCover;

#[derive(Component)]
pub struct Catched;

#[derive(Component)]
pub struct Picked;

#[derive(Component)]
pub struct BallOutletGuideHolderLast;

#[derive(Component)]
pub struct PickedStatic;
