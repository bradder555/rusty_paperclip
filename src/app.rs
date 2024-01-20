use std::time::{SystemTime, Duration};

use egui::{ColorImage, TextureHandle};

pub struct AnimationFrame{
    duration: usize,
    column: usize,
    row: usize
}

pub struct AnimationInfo{
    name: String,
    frames: Vec<AnimationFrame>
}

pub struct AnimationConfig{
    idle_animations: Vec<AnimationInfo>,
    action_animations: Vec<AnimationInfo>
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
//#[derive(serde::Deserialize, serde::Serialize)]
//#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,

    //#[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,
    increment: usize,
    no_sprite_cols: usize,
    no_sprite_rows: usize,
    image: Option<TextureHandle>
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            increment: 0,
            no_sprite_cols: 27,
            no_sprite_rows: 34,
            image: Option::None
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        //egui_extras::install_image_loaders(&cc.egui_ctx);

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        /*
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        */

        let mut this:TemplateApp = Default::default();
        let image = image::io::Reader::open("./assets/clippy.png")
            .expect("problem loading image")
            .decode()
            .expect("problem decoding image, panic");

        let im_buff = image.to_rgba8();
        let pix = im_buff.as_flat_samples();
        let ci = ColorImage::from_rgba_unmultiplied([im_buff.width() as _, im_buff.height() as _], pix.as_slice());
        let tex = cc.egui_ctx.load_texture("clippit_sprite_sheet", ci, Default::default());
        this.image = Some(tex);
        this
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        //eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });
        
        let increment = std::time::SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis() as usize ;
        let increment = increment / 200;
        let total_cycle : usize = increment.rem_euclid(902);
        let hoz_pos = increment.rem_euclid(22);
        let vert_pos = total_cycle / 22;
        //dbg!(hoz_pos + (vert_pos * 22));
        
        egui::CentralPanel::default().show(ctx, |ui| {
                let im = self.image.as_ref().expect("failed to load texture");
                let im_size = im.size_vec2();

                let clippy_width = (im_size.x as usize) / self.no_sprite_cols;
                let clippy_height = (im_size.y as usize) / self.no_sprite_rows;
                let vert_scroll_off = 1.0 + (vert_pos * clippy_height) as f32;
                let hoz_scroll_off = 1.0 + (hoz_pos * clippy_width) as f32;
                //dbg!(hoz_scroll_off);

                egui::ScrollArea::both()
                    .max_height((clippy_height - 5) as f32 )
                    .max_width((clippy_width - 5) as f32 )
                    .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
                    .enable_scrolling(false)
                    .auto_shrink(false)
                    .vertical_scroll_offset(vert_scroll_off)
                    .horizontal_scroll_offset(hoz_scroll_off)
                    .show(ui, |ui|{
                    ui.add(
                        egui::Image::from_texture(im)
                    );
                });
            self.increment += 1;

            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("eframe template");

            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(&mut self.label);
            });

            ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0).text("value"));
            if ui.button("Increment").clicked() {
                self.value += 1.0;
            }

            ui.separator();

            ui.add(egui::github_link_file!(
                "https://github.com/emilk/eframe_template/blob/master/",
                "Source code."
            ));

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
            ui.ctx().request_repaint_after(Duration::from_millis(50));
        });
        
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
