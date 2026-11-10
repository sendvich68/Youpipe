use jni::objects::{JObject, JString, JValue};
use jni::JavaVM;
use std::sync::OnceLock;

// Храним JVM
static JVM: OnceLock<JavaVM> = OnceLock::new();
// Храним Activity (используем Mutex, так как JObject не Send/Sync по умолчанию)
static ACTIVITY: OnceLock<std::sync::Mutex<Option<JObject<'static>>>> = OnceLock::new();

pub fn set_jvm(vm: JavaVM) {
    let _ = JVM.set(vm);
}

pub fn set_activity(activity: JObject<'static>) {
    let lock = ACTIVITY.get_or_init(|| std::sync::Mutex::new(None));
    *lock.lock().unwrap() = Some(activity);
}

pub fn get_activity() -> Option<JObject<'static>> {
    let lock = ACTIVITY.get()?;
    let guard = lock.lock().unwrap();
    guard.as_ref().map(|obj| unsafe { JObject::from_raw(obj.as_raw()) })
}

/// Открывает видео URI через системный Intent ACTION_VIEW.
pub fn open_video(uri: &str) -> Result<(), String> {
    let vm = JVM.get().ok_or("JVM not initialized")?;
    let mut env = vm.attach_current_thread().map_err(|e| e.to_string())?;

    // Получаем текущую Activity
    let activity = get_activity().ok_or("Activity not ready")?;

    // Uri.parse(uri)
    let uri_class = env.find_class("android/net/Uri").map_err(|e| e.to_string())?;
    let uri_str: JString = env.new_string(uri).map_err(|e| e.to_string())?.into();
    let uri_obj = env
        .call_static_method(
            uri_class,
            "parse",
            "(Ljava/lang/String;)Landroid/net/Uri;",
            &[JValue::Object(&uri_str)],
        )
        .map_err(|e| e.to_string())?
        .l()
        .map_err(|e| e.to_string())?;

    // Intent intent = new Intent(Intent.ACTION_VIEW, uri);
    let intent_class = env.find_class("android/content/Intent").map_err(|e| e.to_string())?;
    let action_view: JString = env
        .new_string("android.intent.action.VIEW")
        .map_err(|e| e.to_string())?
        .into();
    let intent = env
        .new_object(
            intent_class,
            "(Ljava/lang/String;Landroid/net/Uri;)V",
            &[JValue::Object(&action_view), JValue::Object(&uri_obj)],
        )
        .map_err(|e| e.to_string())?;

    // intent.setDataAndType(uri, "video/*")
    let mime: JString = env.new_string("video/*").map_err(|e| e.to_string())?.into();
    env.call_method(
        &intent,
        "setDataAndType",
        "(Landroid/net/Uri;Ljava/lang/String;)Landroid/content/Intent;",
        &[JValue::Object(&uri_obj), JValue::Object(&mime)],
    )
    .map_err(|e| e.to_string())?;

    // activity.startActivity(intent)
    env.call_method(
        &activity,
        "startActivity",
        "(Landroid/content/Intent;)V",
        &[JValue::Object(&intent)],
    )
    .map_err(|e| format!("startActivity failed: {e}"))?;

    Ok(())
}
