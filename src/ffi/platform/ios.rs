use crate::ffi::{
    ffi_event::{AdFfi, FfiEvents, InterstitailAdEvents, SENDER},
    ffi_trait::{AdmobInterstitial, AdmobInterstitialTrait, Kv, KvTrait},
};
use std::{
    ffi::{CStr, CString},
    os::raw::{c_char, c_void},
};

extern "C" {
    fn ffi_kv_get(key: *const c_char) -> *const c_char;
    fn ffi_kv_set(key: *const c_char, val: *const c_char);
    fn ffi_kv_delete(key: *const c_char);
    fn ffi_kv_exists(key: *const c_char) -> bool;
}

impl KvTrait for Kv {
    fn set(key: &str, value: &str) {
        let key = CString::new(key).unwrap();
        let val = CString::new(value).unwrap();
        unsafe {
            ffi_kv_set(key.into_raw(), val.into_raw());
        };
    }

    fn get(key: &str) -> String {
        let key = CString::new(key).unwrap();
        unsafe {
            let ptr = ffi_kv_get(key.into_raw());
            let c_str = CStr::from_ptr(ptr);
            match c_str.to_str() {
                Ok(val) => String::from(val),
                Err(_) => String::new(),
            }
        }
    }

    fn delete(key: &str) {
        let key = CString::new(key).unwrap();
        unsafe {
            ffi_kv_delete(key.into_raw());
        };
    }

    fn exists(key: &str) -> bool {
        let key = CString::new(key).unwrap();
        unsafe { ffi_kv_exists(key.into_raw()) }
    }
}

extern "C" {
    fn ffi_ad_init();
    fn ffi_admob_interstitial_load();
    fn ffi_admob_interstitial_show();
    fn ffi_admob_interstitial_is_ready() -> bool;
    fn ffi_admob_interstitial_clear();
}

#[no_mangle]
pub extern "C" fn ffi_callback_admob_interstitial_load_success() {
    SENDER
        .get()
        .unwrap()
        .send(FfiEvents::Ad(AdFfi::AdmobInterstitial(
            InterstitailAdEvents::LoadSuccess,
        )));
}

#[no_mangle]
pub extern "C" fn ffi_callback_admob_interstitial_load_fail(err_msg: *const c_char) {
    let err_msg = unsafe {
        let c_str = CStr::from_ptr(err_msg);
        match c_str.to_str() {
            Ok(val) => String::from(val),
            Err(_) => String::new(),
        }
    };
    SENDER
        .get()
        .unwrap()
        .send(FfiEvents::Ad(AdFfi::AdmobInterstitial(
            InterstitailAdEvents::LoadFail(err_msg),
        )));
}

#[no_mangle]
pub extern "C" fn ffi_callback_admob_interstitial_showed() {
    SENDER
        .get()
        .unwrap()
        .send(FfiEvents::Ad(AdFfi::AdmobInterstitial(
            InterstitailAdEvents::Showed,
        )));
}

#[no_mangle]
pub extern "C" fn ffi_callback_admob_interstitial_show_fail(err_msg: *const c_char) {
    let err_msg = unsafe {
        let c_str = CStr::from_ptr(err_msg);
        match c_str.to_str() {
            Ok(val) => String::from(val),
            Err(_) => String::new(),
        }
    };
    SENDER
        .get()
        .unwrap()
        .send(FfiEvents::Ad(AdFfi::AdmobInterstitial(
            InterstitailAdEvents::ShowFail(err_msg),
        )));
}

#[no_mangle]
pub extern "C" fn ffi_callback_admob_interstitial_dismissed() {
    SENDER
        .get()
        .unwrap()
        .send(FfiEvents::Ad(AdFfi::AdmobInterstitial(
            InterstitailAdEvents::Dismissed,
        )));
}

impl AdmobInterstitialTrait for AdmobInterstitial {
    fn interstitial_show() {
        unsafe {
            ffi_admob_interstitial_show();
        };
    }

    fn interstitial_load() {
        unsafe {
            ffi_admob_interstitial_load();
        };
    }

    fn interstitial_is_ready() -> bool {
        unsafe { ffi_admob_interstitial_is_ready() }
    }

    fn interstitial_clear() {
        unsafe {
            ffi_admob_interstitial_clear();
        };
    }
}

impl AdmobInterstitial {
    pub fn ad_init() {
        unsafe {
            ffi_ad_init();
        };
    }
}
