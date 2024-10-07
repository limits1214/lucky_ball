use crate::ffi::ffi_trait::{Kv, KvTrait};

impl KvTrait for Kv {
    fn save(key: &str, value: &str) {
        todo!()
    }

    fn get(key: &str) -> String {
        todo!()
    }

    fn delete(key: &str) {
        todo!()
    }

    fn exists(key: &str) -> bool {
        todo!()
    }
}
