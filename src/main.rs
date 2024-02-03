#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release




#[tokio::main(flavor = "multi_thread", worker_threads = 6)]
async fn main() -> eframe::Result<()> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_min_inner_size([400.0, 400.0])
            //.with_min_inner_size([438.0, 300.0])
            .with_always_on_top()
            //.with_position(Pos2::new(100.0,100.0))
            .with_close_button(false)
            .with_decorations(false)
            .with_transparent(true)
            .with_drag_and_drop(true)
            ,
        ..Default::default()
    };
    eframe::run_native(
        "Clippit Gpt",
        native_options,
        Box::new(|cc| Box::new(clippit_gpt::ClippitGptApp::new(cc))),
    )
}
