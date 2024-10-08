use bevy::{prelude::*, window::WindowMode};
use lucky_ball::AppPlugin;

#[bevy_main]
fn main() {
    App::new()
        .insert_resource(Msaa::Off)
        .add_plugins(
            DefaultPlugins
                // .set(RenderPlugin {
                //     render_creation: RenderCreation::Automatic(WgpuSettings {
                //         backends: Some(Backends::VULKAN),
                //         ..default()
                //     }),
                //     ..default()
                // })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resizable: false,
                        mode: WindowMode::BorderlessFullscreen,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(AppPlugin)
        .run();
}
