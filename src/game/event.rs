use bevy::prelude::*;

#[derive(Event)]
pub struct BallRigidChange(pub bool);

#[derive(Event)]
pub struct BallMixerRotateEvent(pub f32);

#[derive(Event)]
pub struct DrawStickDownEvent;

#[derive(Event)]
pub struct DrawStickUpEvent;
