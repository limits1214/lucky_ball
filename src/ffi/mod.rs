use bevy::prelude::*;
use bevy_crossbeam_event::{CrossbeamEventApp, CrossbeamEventSender};
use ffi_event::{set_sender, FfiEvents};
use ffi_trait::{AppFfi, AppFfiTrait};

use crate::ui::i18n::set_locale;
pub mod ffi_event;
pub mod ffi_fn;
pub mod ffi_trait;
mod platform;
pub struct FfiPlugin;

impl Plugin for FfiPlugin {
    fn build(&self, app: &mut App) {
        app.add_crossbeam_event::<FfiEvents>()
            .add_systems(Update, ffi_event_test_logging);

        let sender = app
            .world()
            .get_resource::<CrossbeamEventSender<FfiEvents>>()
            .unwrap();
        set_sender(sender.clone());

        #[cfg(target_os = "ios")]
        {
            use ffi_trait::AdmobInterstitial;
            AdmobInterstitial::ad_init();
        }

        AppFfi::init();

        set_locale(AppFfi::get_locale());
    }
}

fn ffi_event_test_logging(mut er: EventReader<FfiEvents>) {
    for evt in er.read() {
        info!("ffi_event_test_logging: {evt:?}");
    }
}
