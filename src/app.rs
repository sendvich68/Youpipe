use egui;

pub struct YouPipeApp {
    pub video_url: String,
    pub status: String,
    pub is_playing: bool,
}

impl Default for YouPipeApp {
    fn default() -> Self {
        Self {
            video_url: "https://commondatastorage.googleapis.com/gtv-videos-bucket/sample/BigBuckBunny.mp4".to_owned(),
            status: "Готов к воспроизведению".to_owned(),
            is_playing: false,
        }
    }
}

impl YouPipeApp {
    pub fn ui(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🎬 YouPipe");
            ui.separator();

            ui.label("Введите URL видео:");
            ui.text_edit_singleline(&mut self.video_url);

            ui.horizontal(|ui| {
                if ui.button("▶ Воспроизвести").clicked() {
                    self.play();
                }

                #[cfg(not(target_os = "android"))]
                {
                    if ui.button("⏸ Пауза").clicked() {
                        crate::player::pause();
                        self.is_playing = false;
                        self.status = "Пауза".to_owned();
                    }
                    if ui.button("▶ Продолжить").clicked() {
                        crate::player::resume();
                        self.is_playing = true;
                        self.status = "Продолжение".to_owned();
                    }
                    if ui.button("⏹ Стоп").clicked() {
                        crate::player::stop();
                        self.is_playing = false;
                        self.status = "Остановлено".to_owned();
                    }
                    if ui.button("📂 Выбрать файл").clicked() {
                        if let Some(path) = rfd::FileDialog::new()
                            .add_filter("Video", &["mp4", "mkv", "webm", "avi"])
                            .pick_file()
                        {
                            let uri = format!("file://{}", path.display());
                            self.video_url = uri.clone();
                            match crate::player::play(&uri) {
                                Ok(_) => {
                                    self.status = format!("Playing: {}", uri);
                                    self.is_playing = true;
                                }
                                Err(e) => self.status = format!("Ошибка: {e}"),
                            }
                        }
                    }
                }
            });

            ui.separator();
            ui.label(format!("Статус: {}", self.status));

            if self.is_playing {
                ui.label("🔊 Воспроизведение активно");
            }
        });
    }

    pub fn play(&mut self) {
        #[cfg(not(target_os = "android"))]
        {
            match crate::player::play(&self.video_url) {
                Ok(_) => {
                    self.status = format!("Playing: {}", self.video_url);
                    self.is_playing = true;
                }
                Err(e) => self.status = format!("Ошибка: {e}"),
            }
        }

        #[cfg(target_os = "android")]
        {
            match crate::android::open_video(&self.video_url) {
                Ok(_) => {
                    self.status = format!("Отправлено во внешний плеер: {}", self.video_url);
                    self.is_playing = true;
                }
                Err(e) => self.status = format!("Ошибка: {e}"),
            }
        }
    }
}
