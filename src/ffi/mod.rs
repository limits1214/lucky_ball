use bevy::{prelude::*, window::PrimaryWindow, winit::WinitWindows};
use bevy_crossbeam_event::{CrossbeamEventApp, CrossbeamEventSender};
use event::{AdmobBannerLaunch, AppInitStartEvent};
use ffi_event::{set_sender, FfiEvents};
use ffi_trait::{AdmobBanner, AdmobBannerTrait, AppFfi, AppFfiTrait};

use crate::ui::i18n::set_locale;
pub mod event;
pub mod ffi_event;
pub mod ffi_fn;
pub mod ffi_trait;
mod platform;
pub mod resource;
pub struct FfiPlugin;

impl Plugin for FfiPlugin {
    fn build(&self, app: &mut App) {
        app.add_crossbeam_event::<FfiEvents>()
            .add_event::<AdmobBannerLaunch>()
            .add_event::<AppInitStartEvent>()
            .add_systems(
                Update,
                (
                    ffi_event_test_logging,
                    ffi_event_app_init_end,
                    er_admob_banner_launch_with_intersti,
                    er_app_init_start,
                ),
            );

        let sender = app
            .world()
            .get_resource::<CrossbeamEventSender<FfiEvents>>()
            .unwrap();
        set_sender(sender.clone());

        set_locale(AppFfi::get_locale());
    }
}

fn ffi_event_test_logging(mut er: EventReader<FfiEvents>) {
    for evt in er.read() {
        info!("ffi_event_test_logging: {evt:?}");
    }
}

fn ffi_event_app_init_end(
    mut er: EventReader<FfiEvents>,
    mut ew_banner_launch: EventWriter<AdmobBannerLaunch>,
    #[cfg(target_os = "ios")] q_primary: Query<Entity, With<PrimaryWindow>>,
    #[cfg(target_os = "ios")] windows: NonSend<WinitWindows>,
) {
    for evt in er.read() {
        if *evt == FfiEvents::App(ffi_event::AppFfi::InitEnd) {
            #[cfg(target_os = "ios")]
            {
                ew_banner_launch.send(AdmobBannerLaunch);
            }
        }
    }
}

fn er_app_init_start(mut er: EventReader<AppInitStartEvent>) {
    for _ in er.read() {
        AppFfi::init();
    }
}

fn er_admob_banner_launch_with_intersti(
    mut er: EventReader<AdmobBannerLaunch>,
    #[cfg(target_os = "ios")] q_primary: Query<Entity, With<PrimaryWindow>>,
    #[cfg(target_os = "ios")] windows: NonSend<WinitWindows>,
) {
    for _ in er.read() {
        #[cfg(any(target_os = "ios", target_os = "android"))]
        {
            use ffi_fn::admob_interstitial_load;
            #[cfg(target_os = "ios")]
            {
                use raw_window_handle::HasWindowHandle;
                if let Ok(entity) = q_primary.get_single() {
                    if let Some(winwrapper) = windows.get_window(entity) {
                        if let Ok(wh) = winwrapper.window_handle() {
                            let rwh = wh.as_raw();
                            info!("AdmobBanner::banner_launch call");
                            AdmobBanner::banner_launch(rwh);
                        }
                    }
                }
                admob_interstitial_load();
            }
            #[cfg(target_os = "android")]
            {
                AdmobBanner::banner_launch();
                admob_interstitial_load();
            }
        }
    }
}
