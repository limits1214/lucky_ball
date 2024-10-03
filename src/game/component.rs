use bevy::prelude::*;

#[derive(Component)]
pub struct BallMixer;

#[derive(Component)]
pub struct BallDrawStick;

#[derive(Component)]
pub struct BallCatchSensor;

#[derive(Component)]
pub struct BallDrawStickIn;

#[derive(Component, Debug)]
pub struct Ball(pub u8);
