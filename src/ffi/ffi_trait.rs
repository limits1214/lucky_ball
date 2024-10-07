pub struct Kv;
pub trait KvTrait {
    fn save(key: &str, value: &str);
    fn get(key: &str) -> String;
    fn delete(key: &str);
    fn exists(key: &str) -> bool;
}

pub struct Ad;
pub trait AdTrait {
    fn interstitial_show();
    fn interstitial_load();
}
