pub struct Kv;
pub trait KvTrait {
    fn set(key: &str, value: &str);
    fn get(key: &str) -> String;
    fn delete(key: &str);
    fn exists(key: &str) -> bool;
}

pub struct AdmobInterstitial;
pub trait AdmobInterstitialTrait {
    fn interstitial_show();
    fn interstitial_load();
}
