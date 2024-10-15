use jni::{
    objects::{JClass, JObject, JString, JValue},
    sys::jboolean,
    JNIEnv, JavaVM,
};
use jni_fn::jni_fn;

use crate::ffi::{
    ffi_event::{FfiEvents, InterstitailAdEvents, SENDER},
    ffi_trait::{
        AdmobBanner, AdmobBannerTrait, AdmobInterstitial, AdmobInterstitialTrait, AppFfi,
        AppFfiTrait, Kv, KvTrait,
    },
};

impl KvTrait for Kv {
    fn set(key: &str, value: &str) {
        let ctx = ndk_context::android_context();
        let vm = unsafe { JavaVM::from_raw(ctx.vm().cast()) }.unwrap();
        let ctx = unsafe { JObject::from_raw(ctx.context().cast()) };
        let mut env = vm.attach_current_thread().unwrap();
        let keyjstr = env.new_string(key).unwrap();
        let keyjobj = JObject::from(keyjstr);
        let keyjval = JValue::from(&keyjobj);
        let valjstr = env.new_string(value).unwrap();
        let valjobj = JObject::from(valjstr);
        let valjval = JValue::from(&valjobj);
        env.call_method(
            ctx,
            "ffiKvSet",
            "(Ljava/lang/String;Ljava/lang/String;)V",
            &[keyjval, valjval],
        )
        .unwrap();
    }

    fn get(key: &str) -> String {
        let ctx = ndk_context::android_context();
        let vm = unsafe { JavaVM::from_raw(ctx.vm().cast()) }.unwrap();
        let ctx = unsafe { JObject::from_raw(ctx.context().cast()) };
        let mut env = vm.attach_current_thread().unwrap();
        let keyjstr = env.new_string(key).unwrap();
        let keyjobj = JObject::from(keyjstr);
        let keyjval = JValue::from(&keyjobj);
        let result = env
            .call_method(
                ctx,
                "ffiKvGet",
                "(Ljava/lang/String;)Ljava/lang/String;",
                &[keyjval],
            )
            .unwrap()
            .l()
            .unwrap();
        let jstr = JString::from(result);
        env.get_string(&jstr).unwrap().into()
    }

    fn delete(key: &str) {
        let ctx = ndk_context::android_context();
        let vm = unsafe { JavaVM::from_raw(ctx.vm().cast()) }.unwrap();
        let ctx = unsafe { JObject::from_raw(ctx.context().cast()) };
        let mut env = vm.attach_current_thread().unwrap();

        let keyjstr = env.new_string(key).unwrap();
        let keyjobj = JObject::from(keyjstr);
        let keyjval = JValue::from(&keyjobj);
        env.call_method(ctx, "ffiKvDelete", "(Ljava/lang/String;)V", &[keyjval])
            .unwrap();
    }

    fn exists(key: &str) -> bool {
        let ctx = ndk_context::android_context();
        let vm = unsafe { JavaVM::from_raw(ctx.vm().cast()) }.unwrap();
        let ctx = unsafe { JObject::from_raw(ctx.context().cast()) };
        let mut env = vm.attach_current_thread().unwrap();
        let keyjstr = env.new_string(key).unwrap();
        let keyjobj = JObject::from(keyjstr);
        let keyjval = JValue::from(&keyjobj);
        let result = env
            .call_method(ctx, "ffiKvExists", "(Ljava/lang/String;)Z", &[keyjval])
            .unwrap()
            .z()
            .unwrap();
        // let jstr = JString::from(result);
        // env.get_string(&jstr).unwrap().into()
        result
    }
}

impl AdmobInterstitialTrait for AdmobInterstitial {
    fn interstitial_show() {
        let ctx = ndk_context::android_context();
        let vm = unsafe { JavaVM::from_raw(ctx.vm().cast()) }.unwrap();
        let ctx = unsafe { JObject::from_raw(ctx.context().cast()) };
        let mut env = vm.attach_current_thread().unwrap();
        env.call_method(ctx, "ffiAdmobInterstitialShow", "()V", &[])
            .unwrap();
    }

    fn interstitial_load() {
        let ctx = ndk_context::android_context();
        let vm = unsafe { JavaVM::from_raw(ctx.vm().cast()) }.unwrap();
        let ctx = unsafe { JObject::from_raw(ctx.context().cast()) };
        let mut env = vm.attach_current_thread().unwrap();
        env.call_method(ctx, "ffiAdmobInterstitialLoad", "()V", &[])
            .unwrap();
    }

    fn interstitial_is_ready() {
        let ctx = ndk_context::android_context();
        let vm = unsafe { JavaVM::from_raw(ctx.vm().cast()) }.unwrap();
        let ctx = unsafe { JObject::from_raw(ctx.context().cast()) };
        let mut env = vm.attach_current_thread().unwrap();
        env.call_method(ctx, "ffiAdmobInterstitialIsReady", "()V", &[])
            .unwrap()
            .v()
            .unwrap()
    }

    fn interstitial_clear() {
        let ctx = ndk_context::android_context();
        let vm = unsafe { JavaVM::from_raw(ctx.vm().cast()) }.unwrap();
        let ctx = unsafe { JObject::from_raw(ctx.context().cast()) };
        let mut env = vm.attach_current_thread().unwrap();
        env.call_method(ctx, "ffiAdmobInterstitialClear", "()V", &[])
            .unwrap();
    }
}

#[jni_fn("xyz.lsy969999.luckyball.RustBinding")]
pub fn ffi_callback_admob_interstitial_load_success(_: JNIEnv, _: JClass) {
    SENDER.get().unwrap().send(FfiEvents::Ad(
        crate::ffi::ffi_event::AdFfi::AdmobInterstitial(InterstitailAdEvents::LoadSuccess),
    ));
}
#[jni_fn("xyz.lsy969999.luckyball.RustBinding")]
pub fn ffi_callback_admob_interstitial_load_fail(mut env: JNIEnv, _: JClass, err_msg: JString) {
    if let Ok(em) = env.get_string(&err_msg) {
        let err_msg: String = em.into();
        SENDER.get().unwrap().send(FfiEvents::Ad(
            crate::ffi::ffi_event::AdFfi::AdmobInterstitial(InterstitailAdEvents::LoadFail(
                err_msg,
            )),
        ));
    }
}
#[jni_fn("xyz.lsy969999.luckyball.RustBinding")]
pub fn ffi_callback_admob_interstitial_showed(_: JNIEnv, _: JClass) {
    SENDER.get().unwrap().send(FfiEvents::Ad(
        crate::ffi::ffi_event::AdFfi::AdmobInterstitial(InterstitailAdEvents::Showed),
    ));
}
#[jni_fn("xyz.lsy969999.luckyball.RustBinding")]
pub fn ffi_callback_admob_interstitial_show_fail(mut env: JNIEnv, _: JClass, err_msg: JString) {
    if let Ok(em) = env.get_string(&err_msg) {
        let err_msg: String = em.into();
        SENDER.get().unwrap().send(FfiEvents::Ad(
            crate::ffi::ffi_event::AdFfi::AdmobInterstitial(InterstitailAdEvents::ShowFail(
                err_msg,
            )),
        ));
    }
}
#[jni_fn("xyz.lsy969999.luckyball.RustBinding")]
pub fn ffi_callback_admob_interstitial_dismissed(_: JNIEnv, _: JClass) {
    SENDER.get().unwrap().send(FfiEvents::Ad(
        crate::ffi::ffi_event::AdFfi::AdmobInterstitial(InterstitailAdEvents::Dismissed),
    ));
}
#[jni_fn("xyz.lsy969999.luckyball.RustBinding")]
pub fn ffi_callback_admob_interstitial_is_ready(_: JNIEnv, _: JClass, is_ready: jboolean) {
    let is_ready_bool = is_ready != 0;
    SENDER.get().unwrap().send(FfiEvents::Ad(
        crate::ffi::ffi_event::AdFfi::AdmobInterstitial(InterstitailAdEvents::IsReady(
            is_ready_bool,
        )),
    ));
}

impl AppFfiTrait for AppFfi {
    fn exit() {
        // todo!()
    }

    fn init() {
        // todo!()
    }

    fn get_current_epoch_time() -> i64 {
        let ctx = ndk_context::android_context();
        let vm = unsafe { JavaVM::from_raw(ctx.vm().cast()) }.unwrap();
        let ctx = unsafe { JObject::from_raw(ctx.context().cast()) };
        let mut env = vm.attach_current_thread().unwrap();

        let result = env
            .call_method(ctx, "ffiGetCurrentEpochTime", "()J", &[])
            .unwrap()
            .j()
            .unwrap();
        result
    }

    fn get_locale() -> String {
        let ctx = ndk_context::android_context();
        let vm = unsafe { JavaVM::from_raw(ctx.vm().cast()) }.unwrap();
        let ctx = unsafe { JObject::from_raw(ctx.context().cast()) };
        let mut env = vm.attach_current_thread().unwrap();

        let result = env
            .call_method(ctx, "fiiGetLocale", "()Ljava/lang/String;", &[])
            .unwrap()
            .l()
            .unwrap();
        let jstr = JString::from(result);
        env.get_string(&jstr).unwrap().into()
    }

    fn get_time_offset() -> i32 {
        let ctx = ndk_context::android_context();
        let vm = unsafe { JavaVM::from_raw(ctx.vm().cast()) }.unwrap();
        let ctx = unsafe { JObject::from_raw(ctx.context().cast()) };
        let mut env = vm.attach_current_thread().unwrap();

        let result = env
            .call_method(ctx, "ffiGetTimeOffset", "()I", &[])
            .unwrap()
            .i()
            .unwrap();
        result
    }
}

#[jni_fn("xyz.lsy969999.luckyball.RustBinding")]
pub fn ffi_callback_app_init_end(_: JNIEnv, _: JClass) {
    SENDER
        .get()
        .unwrap()
        .send(FfiEvents::App(crate::ffi::ffi_event::AppFfi::InitEnd));
}

impl AdmobBannerTrait for AdmobBanner {
    fn banner_launch(#[cfg(target_os = "ios")] rwh: RawWindowHandle) {
        let ctx = ndk_context::android_context();
        let vm = unsafe { JavaVM::from_raw(ctx.vm().cast()) }.unwrap();
        let ctx = unsafe { JObject::from_raw(ctx.context().cast()) };
        let mut env = vm.attach_current_thread().unwrap();
        env.call_method(ctx, "ffiAdmobBannerLaunch", "()V", &[])
            .unwrap();
    }
}
