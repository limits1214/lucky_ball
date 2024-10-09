use super::ffi_trait::{AdmobInterstitial, AdmobInterstitialTrait, Kv, KvTrait};

pub fn admob_interstitial_load() {
    AdmobInterstitial::interstitial_load();
}

pub fn admob_interstitial_show() {
    AdmobInterstitial::interstitial_show();
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
