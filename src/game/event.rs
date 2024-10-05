use bevy::prelude::*;

/// true: dynamic
/// false: static
#[derive(Event)]
pub struct BallRigidChange(pub bool);

#[derive(Event)]
pub struct BallMixerRotateEvent(pub f32);

#[derive(Event)]
pub struct DrawStickDownEvent;

#[derive(Event)]
pub struct DrawStickUpEvent;

#[derive(Event)]
pub struct DrawInnerStickDownEvent;

#[derive(Event)]
pub struct DrawInnerStickUpEvent;

/// true: static
/// false: remove
#[derive(Event)]
pub struct DrawStickRigidChangeEvent(pub bool);

#[derive(Event)]
pub struct BallCatchEvent;

#[derive(Event)]
pub struct BallCatchDoneEvent;

#[derive(Event)]
pub struct BallReleaseEvent;

#[derive(Event)]
pub struct PoolOutletCoverOpenEvent;

#[derive(Event)]
pub struct PoolOutletCoverCloseEvent;

#[derive(Event)]
pub struct GameResetEvent;

#[derive(Event)]
pub struct GameRunEvent;

#[derive(Event)]
pub struct GameEndEvent;

#[derive(Event)]
pub struct GameStepFinishEvent(pub u64);

#[derive(Event)]
pub struct PoolBallCntZeroEvent;
