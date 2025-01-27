use bevy::prelude::*;

use crate::game::constant::BALL_REFS;

pub struct GivenBall(pub u8, pub String);

#[derive(Resource)]
pub struct GameConfig {
    pub is_running: bool,
    // pub is_manual: bool,
    pub is_pool_ball_cnt_sensor: bool,
    pub is_ball_release_sensor: bool,
    pub is_catching: bool,
    pub picked_ball: Vec<u8>,
    pub rule_given_ball: Vec<GivenBall>,
    pub rule_taken_ball: u8,
    pub running_cnt: u32,
    pub show_ad_weight: u8,
}

pub fn make_given_ball(ball_num: Vec<u8>) -> Vec<GivenBall> {
    let mut v: Vec<GivenBall> = vec![];
    for n in ball_num {
        if let Some((num, name)) = BALL_REFS.get((n - 1) as usize) {
            v.push(GivenBall(num.clone(), String::from(*name)));
        }
    }
    v
}

pub fn ball70() -> Vec<u8> {
    let mut v: Vec<u8> = vec![];
    for i in 1..=70 {
        v.push(i);
    }
    v
}

pub fn ball69() -> Vec<u8> {
    let mut v: Vec<u8> = vec![];
    for i in 1..=69 {
        v.push(i);
    }
    v
}

pub fn ball26() -> Vec<u8> {
    let mut v: Vec<u8> = vec![];
    for i in 1..=26 {
        v.push(i);
    }
    v
}
pub fn ball45() -> Vec<u8> {
    let mut v: Vec<u8> = vec![];
    for i in 1..=45 {
        v.push(i);
    }
    v
}
