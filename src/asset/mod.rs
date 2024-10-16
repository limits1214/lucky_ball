use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use resources::MyAsstes;

use super::app::states::{Loading, MyStates};

pub mod resources;
pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(MyStates::Load(Loading::Loading))
                .continue_to_state(MyStates::Load(Loading::Gen))
                .load_collection::<MyAsstes>(),
        )
        .add_systems(OnEnter(MyStates::Load(Loading::Gen)), asset_gen);
    }
}

fn asset_gen(mut next_state: ResMut<NextState<MyStates>>) {
    info!("load end");
    next_state.set(MyStates::Game);
}
