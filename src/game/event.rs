use bevy::prelude::*;

#[derive(Event)]
pub struct GameResetEvent;

#[derive(Event)]
pub struct GameRunEvent;

#[derive(Event)]
pub struct GameEndEvent;

#[derive(Event)]
pub struct GameStepFinishEvent {
    pub event_id: u64,
    pub data: Option<GameStepData>,
}
impl GameStepFinishEvent {
    pub fn new(event_id: u64) -> Self {
        Self {
            event_id,
            data: None,
        }
    }
    pub fn new_with_data(event_id: u64, data: GameStepData) -> Self {
        Self {
            event_id,
            data: Some(data),
        }
    }
}
#[derive(Debug)]
pub enum GameStepData {
    Int(u32),
    Float(f32),
}
#[derive(Event, Debug)]
pub struct GameStepStartEvent {
    pub event_id: u64,
    pub data: Option<GameStepData>,
}

#[derive(Event)]
pub struct DrawStickResetEvent;

impl GameStepStartEvent {
    pub fn new(event_id: u64) -> Self {
        Self {
            event_id,
            data: None,
        }
    }
    pub fn new_with_data(event_id: u64, data: GameStepData) -> Self {
        Self {
            event_id,
            data: Some(data),
        }
    }
}

#[derive(Event)]
pub struct PoolBallCntZeroEvent;

#[derive(Event)]
pub struct BallClearEvent;

/// tuple 0 (bool): is_shuffle
#[derive(Event)]
pub struct BallSpawnEvent(pub bool);
