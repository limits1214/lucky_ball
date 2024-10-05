use bevy::prelude::*;

#[derive(Resource)]
pub struct GameConfig {
    pub is_running: bool,
    // pub is_manual: bool,
    pub is_pool_ball_cnt_sensor: bool,
    pub is_catching: bool,
    pub picked_ball: Vec<u8>,
    pub rule_given_ball: Vec<u8>,
    pub rule_taken_ball: u8,
}
