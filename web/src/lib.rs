use bevy::prelude::*;
use lucky_ball::AppPlugin;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
pub fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    meta_check: bevy::asset::AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        canvas: Some(String::from("#target")),
                        fit_canvas_to_parent: true,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(AppPlugin)
        // .insert_resource(WinitSettings::mobile())
        .run();
}
