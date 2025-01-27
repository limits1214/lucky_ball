use bevy::prelude::*;
use lucky_ball::AppPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(AppPlugin)
        .run();
}
