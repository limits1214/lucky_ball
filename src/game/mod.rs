use bevy::prelude::*;
use event::{
    BallCatchDoneEvent, BallCatchEvent, BallMixerRotateEvent, BallReleaseEvent, BallRigidChange,
    DrawInnerStickDownEvent, DrawInnerStickUpEvent, DrawStickDownEvent, DrawStickRigidChangeEvent,
    DrawStickUpEvent, GameEndEvent, GameResetEvent, GameRunEvent, GameStepFinishEvent,
    PoolBallCntZeroEvent, PoolOutletCoverCloseEvent, PoolOutletCoverOpenEvent,
};
use resource::GameConfig;
use system::{
    ball_catch, ball_catch_sensor_collding, ball_holder_last_collding, ball_mixer_rotate,
    ball_picked_static, draw_inner_stick_down_event, draw_inner_stick_up_event,
    draw_stick_down_event, draw_stick_rigid_change, draw_stick_up_event, er_ball_catch,
    er_ball_release, er_ball_rigid_change, er_game_end, er_game_reset, er_game_run,
    er_pool_outlet_cover_close, er_pool_outlet_cover_open, game_run, pool_ball_cnt_zero_sensor,
    spawn_balls, spawn_setup, tcb_pool_outlet_open_end,
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
        app.add_event::<BallMixerRotateEvent>()
            .add_event::<BallRigidChange>()
            .add_event::<DrawStickDownEvent>()
            .add_event::<DrawStickUpEvent>()
            .add_event::<DrawInnerStickDownEvent>()
            .add_event::<DrawInnerStickUpEvent>()
            .add_event::<DrawStickRigidChangeEvent>()
            .add_event::<BallCatchEvent>()
            .add_event::<BallCatchDoneEvent>()
            .add_event::<BallReleaseEvent>()
            .add_event::<PoolOutletCoverCloseEvent>()
            .add_event::<PoolOutletCoverOpenEvent>()
            .add_event::<GameRunEvent>()
            .add_event::<GameEndEvent>()
            .add_event::<GameResetEvent>()
            .add_event::<GameStepFinishEvent>()
            .add_event::<PoolBallCntZeroEvent>()
            .insert_resource(GameConfig {
                is_running: false,
                is_catching: false,
                is_pool_ball_cnt_sensor: false,
                picked_ball: vec![],
                rule_given_ball: vec![],
                rule_taken_ball: 5,
            })
            .add_systems(OnEnter(MyStates::Game), (spawn_setup, spawn_balls))
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
                        er_game_end,
                        er_game_reset,
                        game_run,
                        pool_ball_cnt_zero_sensor,
                        tcb_pool_outlet_open_end,
                    ),
                )
                    .run_if(in_state(MyStates::Game)),
            );
    }
}
