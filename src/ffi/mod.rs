use bevy::prelude::*;
pub mod ffi_fn;
pub mod ffi_trait;
mod platform;
pub struct FfiPlugin;

impl Plugin for FfiPlugin {
    fn build(&self, app: &mut App) {
        //

        #[cfg(target_os = "ios")]
        {
            use ffi_trait::AdmobInterstitial;
            AdmobInterstitial::ad_init();
        }
    }
}
