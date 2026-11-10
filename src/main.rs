#![cfg_attr(target_os = "android", no_main)]

use youpipe::app::YouPipeApp;

#[cfg(not(target_os = "android"))]
fn main() -> eframe::Result<()> {
    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([720.0, 480.0])
            .with_title("YouPipe"),
        ..Default::default()
    };
    eframe::run_native(
        "YouPipe",
        options,
        Box::new(|_cc| Box::new(YouPipeApp::default())),
    )
}

// ================== ANDROID ==================
#[cfg(target_os = "android")]
mod android_entry {
    use android_activity::{AndroidApp, MainEvent, PollEvent};
    use jni::objects::JObject;
    use jni::JavaVM;
    use youpipe::app::YouPipeApp;
    use std::sync::OnceLock;

    static APP_STATE: OnceLock<YouPipeApp> = OnceLock::new();

    pub fn android_main(app: AndroidApp) {
        env_logger::init();

        // Сохраняем JVM
        let vm = unsafe { JavaVM::from_raw(app.vm_as_ptr() as *mut _) }.unwrap();
        youpipe::android::set_jvm(vm);

        // Сохраняем Activity
        let activity = unsafe { JObject::from_raw(app.activity_as_ptr() as *mut _) };
        youpipe::android::set_activity(activity);

        // Инициализируем наше состояние
        let state = APP_STATE.get_or_init(|| YouPipeApp::default());

        loop {
            let mut events = vec![];
            app.poll_events(Some(std::time::Duration::from_millis(16)), |event| {
                events.push(event);
            });

            for event in events {
                match event {
                    PollEvent::Main(MainEvent::InitWindow { .. }) => {
                        // Как только окно создано — запускаем видео!
                        let _ = youpipe::android::open_video(&state.video_url);
                    }
                    PollEvent::Main(MainEvent::Destroy) => return,
                    _ => {}
                }
            }
        }
    }
}

#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: android_activity::AndroidApp) {
    android_entry::android_main(app);
}
