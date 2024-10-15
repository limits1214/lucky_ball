use avian3d::prelude::AngularVelocity;
use bevy::prelude::*;
use bevy_tweening::{component_animator_system, AnimationSystem, Lens};
use event::{
    BallClearEvent, BallSpawnEvent, DrawStickResetEvent, GameEndEvent, GameResetEvent,
    GameRunEvent, GameStepFinishEvent, GameStepStartEvent, PoolBallCntZeroEvent,
};
use resource::{ball70, make_given_ball, GameConfig};
use system::{
    ball_catch, ball_catch_sensor_collding, ball_holder_last_collding, ball_mixer_rotate,
    ball_picked_static, ball_release_sensor, draw_inner_stick_down_event,
    draw_inner_stick_up_event, draw_stick_down_event, draw_stick_reset_event,
    draw_stick_rigid_change, draw_stick_up_event, er_ball_catch, er_ball_clear, er_ball_release,
    er_ball_rigid_change, er_ball_spawn, er_ffi_ad, er_game_reset, er_game_run,
    er_pool_outlet_cover_close, er_pool_outlet_cover_open, game_run_step_finish, play_ball_sound,
    pool_ball_cnt_zero_sensor, spawn_setup, tcb_to_step_convert,
};

use crate::app::states::MyStates;

pub mod component;
pub mod constant;
pub mod event;
pub mod resource;
pub mod system;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GameRunEvent>()
            .add_event::<GameEndEvent>()
            .add_event::<GameResetEvent>()
            .add_event::<GameStepStartEvent>()
            .add_event::<GameStepFinishEvent>()
            .add_event::<PoolBallCntZeroEvent>()
            .add_event::<BallClearEvent>()
            .add_event::<BallSpawnEvent>()
            .add_event::<DrawStickResetEvent>()
            .insert_resource(GameConfig {
                is_ball_release_sensor: false,
                is_running: false,
                is_catching: false,
                is_pool_ball_cnt_sensor: false,
                picked_ball: vec![],
                rule_given_ball: make_given_ball(ball70()),
                rule_taken_ball: 5,
                running_cnt: 0,
                show_ad_weight: 0,
            })
            .add_systems(OnEnter(MyStates::Game), (spawn_setup /*spawn_balls*/,))
            .add_systems(
                Update,
                (
                    ball_mixer_rotate,
                    ball_catch_sensor_collding,
                    er_ball_rigid_change,
                    draw_stick_down_event,
                    draw_stick_up_event,
                    draw_inner_stick_down_event,
                    draw_inner_stick_up_event,
                    draw_stick_rigid_change,
                    er_ball_catch,
                    ball_catch,
                    er_ball_release,
                    ball_holder_last_collding,
                    ball_picked_static,
                    er_pool_outlet_cover_open,
                    er_pool_outlet_cover_close,
                    (
                        er_game_run,
                        er_game_reset,
                        game_run_step_finish,
                        pool_ball_cnt_zero_sensor,
                        tcb_to_step_convert,
                        er_ball_spawn,
                        er_ball_clear,
                        ball_release_sensor,
                        // play_ball_sound,
                        draw_stick_reset_event,
                        er_ffi_ad,
                    ),
                )
                    .run_if(in_state(MyStates::Game)),
            )
            .add_systems(
                Update,
                component_animator_system::<AngularVelocity>
                    .in_set(AnimationSystem::AnimationUpdate),
            );
    }
}

pub struct MyAngularVelocityYLens {
    start: f32,
    end: f32,
}

impl Lens<AngularVelocity> for MyAngularVelocityYLens {
    fn lerp(&mut self, target: &mut dyn bevy_tweening::Targetable<AngularVelocity>, ratio: f32) {
        target.y = self.start + (self.end - self.start) * ratio;
    }
}
