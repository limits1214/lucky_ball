use bevy::{prelude::*, window::WindowMode, winit::WinitSettings};
use lucky_ball::AppPlugin;

#[bevy_main]
fn main() {
    App::new()
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
                        mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(AppPlugin)
        .insert_resource(WinitSettings::mobile())
        .run();
}
