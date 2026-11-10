use gstreamer as gst;
use gstreamer::prelude::*;
use std::sync::{Arc, Mutex, OnceLock};

static PIPELINE: OnceLock<Arc<Mutex<Option<gst::Pipeline>>>> = OnceLock::new();

fn pipeline_slot() -> &'static Arc<Mutex<Option<gst::Pipeline>>> {
    PIPELINE.get_or_init(|| {
        gst::init().expect("Failed to init GStreamer");
        Arc::new(Mutex::new(None))
    })
}

pub fn play(uri: &str) -> Result<(), String> {
    stop();

    let slot = pipeline_slot();
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

    *slot.lock().unwrap() = Some(pipeline);
    Ok(())
}

pub fn pause() {
    if let Some(p) = pipeline_slot().lock().unwrap().as_ref() {
        let _ = p.set_state(gst::State::Paused);
    }
}

pub fn resume() {
    if let Some(p) = pipeline_slot().lock().unwrap().as_ref() {
        let _ = p.set_state(gst::State::Playing);
    }
}

pub fn stop() {
    let mut guard = pipeline_slot().lock().unwrap();
    if let Some(p) = guard.take() {
        let _ = p.set_state(gst::State::Null);
    }
}
