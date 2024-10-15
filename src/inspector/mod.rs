use bevy::{prelude::*, window::PrimaryWindow};
use bevy_inspector_egui::{
    bevy_egui::{EguiContext, EguiPlugin},
    egui,
};
use iyes_perf_ui::{entries::PerfUiBundle, PerfUiPlugin};

use crate::{
    ffi::ffi_fn::{admob_interstitial_is_ready, kv_delete, kv_exists, kv_get, kv_set},
    game::{
        constant::{
            STEP_BALL_CATCH, STEP_BALL_MIXER_ROTATE, STEP_BALL_RELEASE, STEP_BALL_RIGID_TO_DYNAMIC,
            STEP_BALL_RIGID_TO_STATIC, STEP_BALL_STICK_RIGID_TO_EMPTY,
            STEP_BALL_STICK_RIGID_TO_STATIC, STEP_DRAW_STICK_DOWN, STEP_DRAW_STICK_UP,
            STEP_INNER_DRAW_STICK_DOWN, STEP_INNER_DRAW_STICK_UP, STEP_POOL_OUTLET_CLOSE_START,
            STEP_POOL_OUTLET_OPEN_START,
        },
        event::{BallClearEvent, BallSpawnEvent, GameRunEvent, GameStepData, GameStepStartEvent},
    },
};

pub struct InspectorPlugin;

impl Plugin for InspectorPlugin {
    fn build(&self, app: &mut App) {
        // gui plugin
        app.add_plugins(EguiPlugin)
            .add_plugins(bevy_inspector_egui::DefaultInspectorConfigPlugin) // adds default options and `InspectorEguiImpl`s
            .add_systems(Update, inspector_ui);

        // perf ui
        app.add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
            .add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin)
            .add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin)
            .add_plugins(PerfUiPlugin)
            .add_systems(Startup, setup);
    }
}
fn inspector_ui(world: &mut World) {
    let Ok(egui_context) = world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .get_single(world)
    else {
        return;
    };
    let mut egui_context = egui_context.clone();

    egui::Window::new("UI").show(egui_context.get_mut(), |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            // equivalent to `WorldInspectorPlugin`
            bevy_inspector_egui::bevy_inspector::ui_for_world(world, ui);

            // egui::CollapsingHeader::new("Materials").show(ui, |ui| {
            //     bevy_inspector_egui::bevy_inspector::ui_for_assets::<StandardMaterial>(world, ui);
            // });

            // ui.heading("Entities");
            // bevy_inspector_egui::bevy_inspector::ui_for_world_entities(world, ui);

            if ui.button("mixer 0").clicked() {
                world.send_event(GameStepStartEvent::new_with_data(
                    STEP_BALL_MIXER_ROTATE,
                    GameStepData::Float(0.),
                ));
            }

            if ui.button("mixer 1").clicked() {
                world.send_event(GameStepStartEvent::new_with_data(
                    STEP_BALL_MIXER_ROTATE,
                    GameStepData::Float(1.),
                ));
            }

            if ui.button("mixer 10").clicked() {
                world.send_event(GameStepStartEvent::new_with_data(
                    STEP_BALL_MIXER_ROTATE,
                    GameStepData::Float(10.),
                ));
            }

            if ui.button("stick down").clicked() {
                world.send_event(GameStepStartEvent::new(STEP_DRAW_STICK_DOWN));
            }

            if ui.button("stick up").clicked() {
                world.send_event(GameStepStartEvent::new(STEP_DRAW_STICK_UP));
            }

            if ui.button("stick inner down").clicked() {
                world.send_event(GameStepStartEvent::new(STEP_INNER_DRAW_STICK_DOWN));
            }

            if ui.button("stick inner up").clicked() {
                world.send_event(GameStepStartEvent::new(STEP_INNER_DRAW_STICK_UP));
            }

            //

            if ui.button("stick static").clicked() {
                world.send_event(GameStepStartEvent::new(STEP_BALL_STICK_RIGID_TO_STATIC));
            }

            if ui.button("stick static remove").clicked() {
                world.send_event(GameStepStartEvent::new(STEP_BALL_STICK_RIGID_TO_EMPTY));
            }

            //

            if ui.button("ball static").clicked() {
                world.send_event(GameStepStartEvent::new(STEP_BALL_RIGID_TO_STATIC));
            }

            if ui.button("ball dynamci").clicked() {
                world.send_event(GameStepStartEvent::new(STEP_BALL_RIGID_TO_DYNAMIC));
            }

            //

            if ui.button("ball catch").clicked() {
                world.send_event(GameStepStartEvent::new(STEP_BALL_CATCH));
            }

            if ui.button("ball release").clicked() {
                world.send_event(GameStepStartEvent::new(STEP_BALL_RELEASE));
            }

            //

            if ui.button("pool outlet  open").clicked() {
                world.send_event(GameStepStartEvent::new(STEP_POOL_OUTLET_OPEN_START));
            }

            if ui.button("pool outlet  close").clicked() {
                world.send_event(GameStepStartEvent::new(STEP_POOL_OUTLET_CLOSE_START));
            }

            //

            if ui.button("ball spawn normal").clicked() {
                world.send_event(BallSpawnEvent(false));
            }

            if ui.button("ball spawn shuffle").clicked() {
                world.send_event(BallSpawnEvent(true));
            }

            if ui.button("ball clear").clicked() {
                world.send_event(BallClearEvent);
            }

            //

            if ui.button("game run").clicked() {
                world.send_event(GameRunEvent);
            }

            //

            #[cfg(any(target_os = "android", target_os = "ios"))]
            {
                use super::ffi::ffi_fn::{
                    admob_banner_launch, admob_interstitial_is_ready, admob_interstitial_load,
                    admob_interstitial_show,
                };
                if ui.button("admob interstital load").clicked() {
                    admob_interstitial_load();
                }

                if ui.button("admob interstital show").clicked() {
                    admob_interstitial_show();
                }

                if ui.button("admob interstital is ready").clicked() {
                    admob_interstitial_is_ready();
                }

                if ui.button("admob banner launch").clicked() {
                    #[cfg(target_os = "ios")]
                    {
                        use bevy::winit::WinitWindows;
                        use raw_window_handle::HasWindowHandle;
                        let mut q_primary = world.query_filtered::<Entity, With<PrimaryWindow>>();
                        let windows = world.non_send_resource::<WinitWindows>();
                        let e = q_primary.single(&world);
                        let winwrapper = windows.get_window(e).unwrap();
                        let wh = winwrapper.window_handle().unwrap();
                        let rwh = wh.as_raw();
                        admob_banner_launch(rwh);
                    }
                    #[cfg(target_os = "android")]
                    {
                        admob_banner_launch();
                    }
                }
            }

            if ui.button("kvset").clicked() {
                kv_set("test!", "1");
            }

            if ui.button("kvget").clicked() {
                let s = kv_get("test!");
                info!("kvget s: {s:?}");
            }

            if ui.button("kvdelete").clicked() {
                kv_delete("test!");
            }
            if ui.button("kvexists").clicked() {
                let b = kv_exists("test!");
                info!("kvget b: {b:?}");
            }
        });
    });
}
fn setup(mut commands: Commands) {
    commands.spawn(PerfUiBundle::default());
}
