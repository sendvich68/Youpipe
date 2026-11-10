pub mod app;

#[cfg(not(target_os = "android"))]
pub mod player;

#[cfg(target_os = "android")]
pub mod android;
