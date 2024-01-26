use std::ops::DerefMut;

use std::time::{SystemTime, Duration};
use std::sync::Arc;
use std::sync::Mutex;
use egui::{ColorImage, TextureHandle};
use crate::actions::DispatchActions;
use crate::animation::models::SpriteSheetInfo;
use crate::animation::service::AnimationService;
use tokio::sync::broadcast;

#[derive(Clone)]
pub struct TemplateAppShared{
    label: String,
    value: f32,
    sprite_sheet_info: SpriteSheetInfo
}

pub struct TemplateApp {
    state: Arc<Mutex<TemplateAppShared>>,
    image: TextureHandle
}

fn load_image_as_color_image(filepath:&str) -> ColorImage {
    let image = image::io::Reader::open(filepath)
    .expect("problem loading image")
    .decode()
    .expect("problem decoding image, panic");

    let im_buff = image.to_rgba8();
    let pix = im_buff.as_flat_samples();
    ColorImage::from_rgba_unmultiplied([im_buff.width() as _, im_buff.height() as _], pix.as_slice())
}



impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {

        let (sndr, _) = broadcast::channel::<DispatchActions>(50);
        let mut receiver = sndr.subscribe();

        tokio::spawn(
            async move {
                let v = receiver.recv().await.unwrap();
                dbg!(v);
            }
        );
        
        let clippit_animation = AnimationService::new(
            "./assets/animations.yaml", 
            sndr.subscribe(), 
            sndr.clone()
        );

        let more_senders = sndr.clone();
        tokio::spawn(
            async move {
                tokio::time::sleep(Duration::from_millis(5000)).await;
                let _ = more_senders.send(
                    DispatchActions::AskQuestion("What is my purpose?".to_owned())
                );
            }
        );

        let shared = Arc::new(
            Mutex::new(
                TemplateAppShared {
                    label: "hello is label".to_owned(),
                    value: 3.0,
                    sprite_sheet_info: SpriteSheetInfo{
                        columns: 27,
                        rows: 34
                    },
                }
            )
        );

        let app = TemplateApp{
            state: shared.clone(),
            image: cc.egui_ctx.load_texture(
                "clippit_sprite_sheet", 
                load_image_as_color_image("./assets/clippy.png"), 
                Default::default()
            )
        };

        let ctx_rp = cc.egui_ctx.clone();
        tokio::spawn(
            async move {
                loop {
                    tokio::time::sleep(Duration::from_millis(50)).await;
                    ctx_rp.request_repaint();
                }
            }
        );

        let ctx_rp = cc.egui_ctx.clone();
        let _shared = shared.clone();
        tokio::spawn(
            async move {
                loop {
                    tokio::time::sleep(Duration::from_millis(2000)).await;
                    let mut _shared = _shared.lock().unwrap();
                    _shared.value += 1.0;
                    ctx_rp.request_repaint();
                }
            }
        );

        app
    }
}

impl eframe::App for TemplateApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        let mut state = self.state.lock().unwrap().deref_mut().clone();
        
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
        
        let im_size = self.image.size_vec2();
        let total_cycle : usize = increment.rem_euclid(state.sprite_sheet_info.columns * state.sprite_sheet_info.rows);
        let hoz_pos = increment.rem_euclid(state.sprite_sheet_info.columns);
        let vert_pos = total_cycle / state.sprite_sheet_info.columns;
        
        egui::CentralPanel::default().show(ctx, |ui| {
                

                let clippy_width = (im_size.x as usize) / state.sprite_sheet_info.columns;
                let clippy_height = (im_size.y as usize) / state.sprite_sheet_info.rows;
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
                ui.text_edit_singleline(&mut state.label);
            });

            
            ui.add(egui::Slider::new(&mut state.value, 0.0..=10.0).text("value"));
            /*
            if ui.button("Increment").clicked() {
                self.value += 1.0;
            }
            */

            ui.separator();

            ui.add(egui::github_link_file!(
                "https://github.com/emilk/eframe_template/blob/master/",
                "Source code."
            ));

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
            //ui.ctx().request_repaint_after(Duration::from_millis(10));
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
