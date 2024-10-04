use bevy::prelude::*;
use event::{
    BallCatchDoneEvent, BallCatchEvent, BallMixerRotateEvent, BallReleaseEvent, BallRigidChange,
    DrawInnerStickDownEvent, DrawInnerStickUpEvent, DrawStickDownEvent, DrawStickRigidChangeEvent,
    DrawStickUpEvent,
};
use resource::GameConfig;
use system::{
    ball_catch, ball_catch_sensor_collding, ball_holder_last_collding, ball_mixer_rotate,
    ball_picked_static, draw_inner_stick_down_event, draw_inner_stick_up_event,
    draw_stick_down_event, draw_stick_rigid_change, draw_stick_up_event, er_ball_catch,
    er_ball_release, er_ball_rigid_change, spawn_balls, spawn_setup,
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
            .insert_resource(GameConfig { is_catching: false })
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
                )
                    .run_if(in_state(MyStates::Game)),
            );
    }
}
