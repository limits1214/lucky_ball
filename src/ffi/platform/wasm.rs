use crate::ffi::ffi_trait::{
    AdmobBanner, AdmobBannerTrait, AdmobInterstitial, AdmobInterstitialTrait, AppFfi, AppFfiTrait,
    Kv, KvTrait,
};
use bevy::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn ffi_get_current_epoch_time() -> i64;
    fn ffi_get_time_offset() -> i32;
}

impl AppFfiTrait for AppFfi {
    fn exit() {}

    fn init() {}

    fn get_current_epoch_time() -> i64 {
        ffi_get_current_epoch_time()
    }

    fn get_locale() -> String {
        String::from("ko_KR")
    }

    fn get_time_offset() -> i32 {
        ffi_get_time_offset()
    }

    fn splash_hide() {}
}

#[wasm_bindgen]
extern "C" {
    fn ffi_kv_get(key: &str) -> String;
    fn ffi_kv_set(key: &str, val: &str);
    fn ffi_kv_delete(key: &str);
    fn ffi_kv_exists(key: &str) -> bool;
}

impl KvTrait for Kv {
    fn set(key: &str, value: &str) {
        ffi_kv_set(key, value);
    }

    fn get(key: &str) -> String {
        ffi_kv_get(key)
    }

    fn delete(key: &str) {
        ffi_kv_delete(key);
    }

    fn exists(key: &str) -> bool {
        ffi_kv_exists(key)
    }
}

impl AdmobInterstitialTrait for AdmobInterstitial {
    fn interstitial_show() {
        info!("not imple");
    }

    fn interstitial_load() {
        info!("not imple");
    }

    fn interstitial_is_ready() {
        info!("not imple");
    }

    fn interstitial_clear() {
        info!("not imple");
    }
}
impl AdmobBannerTrait for AdmobBanner {
    fn banner_launch(#[cfg(target_os = "ios")] rwh: RawWindowHandle) {
        info!("not imple");
    }
}
