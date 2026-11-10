use gstreamer as gst;
use gstreamer::prelude::*;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Player {
    pipeline: Arc<Mutex<Option<gst::Pipeline>>>,
}

impl Player {
    pub fn new() -> Self {
        gst::init().expect("Failed to init GStreamer");
        Self {
            pipeline: Arc::new(Mutex::new(None)),
        }
    }

    /// Запустить воспроизведение видео по URI (file:// или http(s)://)
    pub fn play(&self, uri: &str) -> Result<(), String> {
        self.stop();

        let pipeline = gst::Pipeline::new(None);

        let playbin = gst::ElementFactory::make("playbin")
            .build()
            .map_err(|e| format!("Failed to create playbin: {e}"))?;

        playbin.set_property("uri", uri);

        pipeline
            .add(&playbin)
            .map_err(|e| format!("Failed to add playbin: {e}"))?;

        pipeline
            .set_state(gst::State::Playing)
            .map_err(|e| format!("Failed to start playback: {e}"))?;

        *self.pipeline.lock().unwrap() = Some(pipeline);
        Ok(())
    }

    pub fn pause(&self) {
        if let Some(p) = self.pipeline.lock().unwrap().as_ref() {
            let _ = p.set_state(gst::State::Paused);
        }
    }

    pub fn resume(&self) {
        if let Some(p) = self.pipeline.lock().unwrap().as_ref() {
            let _ = p.set_state(gst::State::Playing);
        }
    }

    pub fn stop(&self) {
        let mut guard = self.pipeline.lock().unwrap();
        if let Some(p) = guard.take() {
            let _ = p.set_state(gst::State::Null);
        }
    }
}
