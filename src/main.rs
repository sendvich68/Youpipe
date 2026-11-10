mod player;

use eframe::egui;
use player::Player;

struct YouPipeApp {
    video_url: String,
    status: String,
    player: Player,
    is_playing: bool,
}

impl Default for YouPipeApp {
    fn default() -> Self {
        Self {
            video_url: "https://commondatastorage.googleapis.com/gtv-videos-bucket/sample/BigBuckBunny.mp4".to_owned(),
            status: "Готов к воспроизведению".to_owned(),
            player: Player::new(),
            is_playing: false,
        }
    }
}

impl eframe::App for YouPipeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🎬 YouPipe");
            ui.separator();

            ui.label("Введите URL видео:");
            ui.text_edit_singleline(&mut self.video_url);

            ui.horizontal(|ui| {
                if ui.button("▶ Воспроизвести").clicked() {
                    match self.player.play(&self.video_url) {
                        Ok(_) => {
                            self.status = format!("Playing: {}", self.video_url);
                            self.is_playing = true;
                        }
                        Err(e) => self.status = format!("Ошибка: {e}"),
                    }
                }

                if ui.button("⏸ Пауза").clicked() {
                    self.player.pause();
                    self.is_playing = false;
                    self.status = "Пауза".to_owned();
                }

                if ui.button("▶ Продолжить").clicked() {
                    self.player.resume();
                    self.is_playing = true;
                    self.status = "Продолжение".to_owned();
                }

                if ui.button("⏹ Стоп").clicked() {
                    self.player.stop();
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
                        match self.player.play(&uri) {
                            Ok(_) => {
                                self.status = format!("Playing: {}", uri);
                                self.is_playing = true;
                            }
                            Err(e) => self.status = format!("Ошибка: {e}"),
                        }
                    }
                }
            });

            ui.separator();
            ui.label(format!("Статус: {}", self.status));

            if self.is_playing {
                ui.label("🔊 Воспроизведение активно (аудио/видео через GStreamer)");
            }
        });
    }
}

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
        Box::new(|_cc| Box::<YouPipeApp>::default()),
    )
}
