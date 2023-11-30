use std::time::SystemTime;

use egui::TextureHandle;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
//#[derive(serde::Deserialize, serde::Serialize)]
//#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,

    //#[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,
    increment: usize,
    sprite_sheet: Option<TextureHandle>
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            increment: 0,
            sprite_sheet: None
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        //egui_extras::install_image_loaders(&cc.egui_ctx);
        
        // The top panel is often a good place for a menu bar:
        let image = image::io::Reader::open("./assets/clippy spritesheet.png")
        .expect("oops!")
        .decode()
        .expect("oops!");
        let mut image_bytes = image.as_bytes().to_vec().clone();
        for x in 0..(image_bytes.len()/4) {
            let x = x * 4;
            if image_bytes[x + 0] == 255 && image_bytes[x + 2] == 255 {
                image_bytes[x + 3] = 0;
            }
        }
        let ci = egui::ColorImage::from_rgba_unmultiplied([image.width() as _, image.height() as _ ], &image_bytes);
        let t = cc.egui_ctx.load_texture("clippy_sprite_sheet", ci, Default::default());
        
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        /*
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        */

        let mut this:TemplateApp = Default::default();
        this.sprite_sheet = Some(t);
        this
    }
}

fn get_image_block(
    im: &image::DynamicImage, 
    block_size: &[usize; 2],
    offset_x: usize, 
    offset_y: usize
) -> Vec<u8>{
    let im = im.clone();
    let block_size = block_size.clone();
    let offset_x = offset_x * 4;
    let offset_y = offset_y * 4;
    
    let block_width = block_size[0];
    let block_height = block_size[1];
    
    let actual_width = im.width();
    let width_byte_count: usize = actual_width as usize * 4;

    let image_buffer = im.to_rgba8();
    let pixels = image_buffer.as_flat_samples();
    let pixels = &pixels.as_slice();

    let mut r_buffer: Vec<u8> =  vec![0; 4 * block_height * block_width];
    let mut r_buffer_off : usize;
    let mut i_buffer_off : usize;

    for by in 0..block_height{
        r_buffer_off = by * block_width * 4;
        i_buffer_off = (by + offset_y) * width_byte_count + offset_x;
        r_buffer[r_buffer_off..][..block_width].copy_from_slice(&pixels[i_buffer_off..(i_buffer_off + block_width)]);
    }

    r_buffer.clone()
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
        let increment = increment / 1000;
        let total_cycle : usize = increment.rem_euclid(902);
        let hoz_pos = increment.rem_euclid(22);
        let vert_pos = total_cycle / 22;
        //dbg!(hoz_pos + (vert_pos * 22));
        
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.sprite_sheet.is_some(){
                let sprite_sheet = &self.sprite_sheet.as_ref().unwrap();
                let ss = sprite_sheet.size();
                let clippy_width = ss[0] / 22;
                let clippy_height = ss[1] / 41;
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
                    ui.add(egui::Image::from_texture(*sprite_sheet));
                });
            }
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
            ui.ctx().request_repaint();
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
