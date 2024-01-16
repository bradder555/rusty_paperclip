use std::time::UNIX_EPOCH;
use image::{DynamicImage, Pixel};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,

    value: f32,
    #[serde(skip)]
    image: Option<DynamicImage>
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            image: None
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        egui_extras::install_image_loaders(&cc.egui_ctx);
        

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        /*
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        */

        let mut this:TemplateApp = Default::default();
        let im = image::io::Reader::open("./assets/clippy spritesheet.png")
            .unwrap()
            .decode()
            .unwrap();

        let image_buffer = im.to_rgba8();
        let mut out_buffer = image_buffer.clone();
        for p in out_buffer.pixels_mut(){
            let mut chans = p.channels_mut();
            if (chans[0] == 255) && (chans[2] == 255) {
                chans[3] = 0
            }
        }

        this.image = Some(
            DynamicImage::from(out_buffer)
        );
        this
    }
}

fn get_image_block(
    im: &DynamicImage,
    block_size: &[usize; 2],
    offset_x: usize, 
    offset_y: usize
) -> Vec<u8>{
    let offset_x = offset_x * 4; // offset is in pixels
    let offset_y = offset_y;
    
    let block_width = block_size[0];
    let block_height = block_size[1];

    let bytes_per_row = block_width * 4;
    let bytes_per_orig_row = (im.width() * 4) as usize;

    let image_buffer = im.to_rgba8();
    let pixels = image_buffer.as_flat_samples();
    let pixels = pixels.as_slice();

    let mut r_buffer: Vec<u8> =  vec![255; bytes_per_row * block_height];

    for row in 0..block_height{
        let r_start = row * bytes_per_row;
        // i don't understand why the block_width needs to be multiplied by 4 again
        // but you only get 1/4 the image if you don't
        // thanks immutability and scope!
        let block_width = block_width * 4;
        let r_end = r_start + block_width;

        let i_start = ((row + offset_y) * bytes_per_orig_row) + offset_x;
        let i_end = i_start + block_width;

        r_buffer[r_start..r_end].copy_from_slice(&pixels[i_start..i_end]);
    }

    r_buffer
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui
        ctx.request_repaint_after(std::time::Duration::from_millis(20));
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

        egui::CentralPanel::default().show(ctx, |ui| {
            // The top panel is often a good place for a menu bar
            if self.image.is_some() {
                let t_inc = (
                    std::time::SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_millis() / 100
                ) as usize;

                let max_cycle = (t_inc).rem_euclid(22 * 41);
                let vert_i = max_cycle / 22;
                let hoz_i = (max_cycle).rem_euclid(22);
                //dbg!(max_cycle, vert_i, hoz_i);
                let image = self.image.as_ref().unwrap();
                let block_size = [124usize, 93usize];
                let imblock = get_image_block(
                    &image,
                    &block_size,
                    hoz_i * 124,
                    vert_i * 93
                );

                let ci = egui::ColorImage::from_rgba_unmultiplied(block_size, &imblock);
                let t = ui.ctx().load_texture("clippy_spritesheet", ci, Default::default());
                ui.add(egui::Image::from_texture(&t.clone()));
            }
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

            
        });
    }
}