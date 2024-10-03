use bevy::prelude::*;
use event::{BallMixerRotateEvent, BallRigidChange, DrawStickDownEvent, DrawStickUpEvent};
use system::{
    ball_catch_sensor_collding, ball_mixer_rotate, draw_stick_down_event, draw_stick_up_event,
    er_ball_rigid_change, spawn_balls, spawn_setup,
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
            .add_systems(OnEnter(MyStates::Game), (spawn_setup, spawn_balls))
            .add_systems(
                Update,
                (
                    ball_mixer_rotate,
                    ball_catch_sensor_collding,
                    er_ball_rigid_change,
                    draw_stick_down_event,
                    draw_stick_up_event,
                )
                    .run_if(in_state(MyStates::Game)),
            );
    }
}
