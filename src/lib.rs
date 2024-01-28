#![warn(clippy::all, rust_2018_idioms)]

pub mod models;
pub mod animation;
pub mod actions;
pub mod gui_state;
pub mod state_updater;

mod app;
pub use app::ClippitGptApp;
