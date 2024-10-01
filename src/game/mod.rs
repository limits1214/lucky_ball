use bevy::prelude::*;
use system::{rotate, test_setup};

use crate::app::states::MyStates;

pub mod system;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MyStates::Game), test_setup)
            .add_systems(Update, rotate.run_if(in_state(MyStates::Game)));
    }
}
