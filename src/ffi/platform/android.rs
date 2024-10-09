use jni::{
    objects::{JClass, JObject, JString, JValue},
    JNIEnv, JavaVM,
};

use crate::ffi::ffi_trait::{AdmobInterstitial, AdmobInterstitialTrait, Kv, KvTrait};

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
}
