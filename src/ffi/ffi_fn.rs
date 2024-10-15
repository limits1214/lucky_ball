#[cfg(target_os = "ios")]
use raw_window_handle::RawWindowHandle;

use super::ffi_trait::{
    AdmobBanner, AdmobBannerTrait, AdmobInterstitial, AdmobInterstitialTrait, Kv, KvTrait,
};

pub fn admob_interstitial_load() {
    AdmobInterstitial::interstitial_load();
}

pub fn admob_interstitial_show() {
    AdmobInterstitial::interstitial_show();
}

pub fn admob_interstitial_is_ready() {
    AdmobInterstitial::interstitial_is_ready()
}

pub fn admob_banner_launch(#[cfg(target_os = "ios")] rwh: RawWindowHandle) {
    AdmobBanner::banner_launch(
        #[cfg(target_os = "ios")]
        rwh,
    );
}

pub fn kv_get(key: &str) -> String {
    Kv::get(key)
}

pub fn kv_set(key: &str, value: &str) {
    Kv::set(key, value);
}

pub fn kv_delete(key: &str) {
    Kv::delete(key);
}

pub fn kv_exists(key: &str) -> bool {
    Kv::exists(key)
}

pub fn quit_app() {}
