pub mod objects;

use ::jni::{objects::JObject, JNIEnv, JavaVM, NativeMethod};
use jni::{objects::JString, sys::jboolean};
use once_cell::sync::OnceCell;
use std::ffi::c_void;

static GLOBAL_JVM: OnceCell<JavaVM> = OnceCell::new();

pub fn init(env: &JNIEnv) -> crate::Result<()> {
    if let Ok(()) = GLOBAL_JVM.set(env.get_java_vm()?) {
        env.register_native_methods(
            "com/bleat/bleat_core/android/impl/Adapter",
            &[
                NativeMethod {
                    name: "reportScanResult".into(),
                    sig: "(Landroid/bluetooth/le/ScanResult;)V".into(),
                    fn_ptr: adapter_report_scan_result as *mut c_void,
                },
                NativeMethod {
                    name: "onConnectionStateChanged".into(),
                    sig: "(Ljava/lang/String;Z)V".into(),
                    fn_ptr: adapter_on_connection_state_changed as *mut c_void,
                },
            ],
        )?;
        jni_utils::classcache::find_add_class(env, "com/bleat/bleat_core/android/impl/Peripheral")?;
        jni_utils::classcache::find_add_class(env, "com/bleat/bleat_core/android/impl/ScanFilter")?;
        jni_utils::classcache::find_add_class(
            env,
            "com/bleat/bleat_core/android/impl/NotConnectedException",
        )?;
        jni_utils::classcache::find_add_class(
            env,
            "com/bleat/bleat_core/android/impl/PermissionDeniedException",
        )?;
        jni_utils::classcache::find_add_class(
            env,
            "com/bleat/bleat_core/android/impl/UnexpectedCallbackException",
        )?;
        jni_utils::classcache::find_add_class(
            env,
            "com/bleat/bleat_core/android/impl/UnexpectedCharacteristicException",
        )?;
        jni_utils::classcache::find_add_class(
            env,
            "com/bleat/bleat_core/android/impl/NoSuchCharacteristicException",
        )?;
    }
    Ok(())
}

pub fn global_jvm() -> &'static JavaVM {
    GLOBAL_JVM.get().expect(
        "Droidplug has not been initialized. Please initialize it with bleat_core::platform::init().",
    )
}

impl From<::jni::errors::Error> for crate::Error {
    fn from(err: ::jni::errors::Error) -> Self {
        Self::Other(Box::new(err))
    }
}

extern "C" fn adapter_report_scan_result(env: JNIEnv, obj: JObject, scan_result: JObject) {
    let _ = super::adapter::adapter_report_scan_result_internal(&env, obj, scan_result);
}

extern "C" fn adapter_on_connection_state_changed(
    env: JNIEnv,
    obj: JObject,
    addr: JString,
    connected: jboolean,
) {
    let _ =
        super::adapter::adapter_on_connection_state_changed_internal(&env, obj, addr, connected);
}
