use crate::ffi::{
    ffi_event::{AdFfi, FfiEvents, InterstitailAdEvents, SENDER},
    ffi_trait::{
        AdmobBanner, AdmobBannerTrait, AdmobInterstitial, AdmobInterstitialTrait, AppFfi,
        AppFfiTrait, Kv, KvTrait,
    },
};
use raw_window_handle::RawWindowHandle;
use std::{
    ffi::{c_void, CStr, CString},
    os::raw::c_char,
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
    fn ffi_admob_interstitial_load();
    fn ffi_admob_interstitial_show();
    fn ffi_admob_interstitial_is_ready();
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

#[no_mangle]
pub extern "C" fn ffi_callback_admob_interstitial_is_ready(is_ready: bool) {
    SENDER
        .get()
        .unwrap()
        .send(FfiEvents::Ad(AdFfi::AdmobInterstitial(
            InterstitailAdEvents::IsReady(is_ready),
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

    fn interstitial_is_ready() {
        unsafe {
            ffi_admob_interstitial_is_ready();
        };
    }

    fn interstitial_clear() {
        unsafe {
            ffi_admob_interstitial_clear();
        };
    }
}

extern "C" {
    fn ffi_admob_banner_launch(vc: *mut c_void);
}

impl AdmobBannerTrait for AdmobBanner {
    fn banner_launch(rwh: RawWindowHandle) {
        if let RawWindowHandle::UiKit(uikit) = rwh {
            let vc = uikit.ui_view_controller.unwrap();
            unsafe {
                ffi_admob_banner_launch(vc.as_ptr());
            }
        }
    }
}

extern "C" {
    fn ffi_app_init();
    fn ffi_app_exit();
    fn ffi_get_current_epoch_time() -> i64;
    fn ffi_get_locale() -> *const c_char;
    fn ffi_get_time_offset() -> i32;
}

#[no_mangle]
pub extern "C" fn ffi_callback_app_init_end() {
    SENDER
        .get()
        .unwrap()
        .send(FfiEvents::App(crate::ffi::ffi_event::AppFfi::InitEnd));
}

impl AppFfiTrait for AppFfi {
    fn exit() {
        unsafe {
            ffi_app_exit();
        };
    }

    fn init() {
        unsafe {
            ffi_app_init();
        };
    }

    fn get_current_epoch_time() -> i64 {
        unsafe { ffi_get_current_epoch_time() }
    }

    fn get_locale() -> String {
        unsafe {
            let ptr = ffi_get_locale();
            let c_str = CStr::from_ptr(ptr);
            match c_str.to_str() {
                Ok(val) => String::from(val),
                Err(_) => String::new(),
            }
        }
    }

    fn get_time_offset() -> i32 {
        unsafe { ffi_get_time_offset() }
    }
}
