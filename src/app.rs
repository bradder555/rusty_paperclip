use std::time::{SystemTime, Duration};
use egui::{ColorImage, TextureHandle};
use crate::animation::models::SpriteSheetInfo;

pub struct TemplateApp {
    label: String,
    value: f32,
    sprite_sheet_info: SpriteSheetInfo,
    image: TextureHandle
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {

        let image = image::io::Reader::open("./assets/clippy.png")
        .expect("problem loading image")
        .decode()
        .expect("problem decoding image, panic");

        let im_buff = image.to_rgba8();
        let pix = im_buff.as_flat_samples();
        let ci = ColorImage::from_rgba_unmultiplied([im_buff.width() as _, im_buff.height() as _], pix.as_slice());
        let tex = cc.egui_ctx.load_texture("clippit_sprite_sheet", ci, Default::default());

        TemplateApp{
            label: "hello is label".to_owned(),
            value: 3.0,
            sprite_sheet_info: SpriteSheetInfo{
                columns: 27,
                rows: 34
            },
            image: tex
        }
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
        let increment = increment / 100;
        let total_cycle : usize = increment.rem_euclid(902);
        let hoz_pos = increment.rem_euclid(22);
        let vert_pos = total_cycle / 22;
        //dbg!(hoz_pos + (vert_pos * 22));
        
        egui::CentralPanel::default().show(ctx, |ui| {
                let im_size = self.image.size_vec2();

                let clippy_width = (im_size.x as usize) / self.sprite_sheet_info.columns;
                let clippy_height = (im_size.y as usize) / self.sprite_sheet_info.rows;
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
                        egui::Image::from_texture(&self.image)
                    );
                });

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
            ui.ctx().request_repaint_after(Duration::from_millis(10));
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
