use std::collections::HashMap;
use std::ops::DerefMut;

use std::time::{Duration};
use std::sync::Arc;
use std::sync::Mutex;
use crate::actions::DispatchActions;
use crate::animation::models::AnimationServiceMode;
use crate::animation::service::AnimationService;
use tokio::sync::broadcast;

#[derive(Clone)]
pub struct TemplateAppShared{
    label: String,
    value: f32,
    mode: AnimationServiceMode
}

pub struct TemplateApp {
    state: Arc<Mutex<TemplateAppShared>>,
    animations: HashMap<String, AnimationService>
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
                    mode: AnimationServiceMode::Idle
                }
            )
        );

        let clippit_animation = AnimationService::new(
            cc.egui_ctx.clone(),
            "./assets/animations.yaml", 
            "./assets/clippy.png",
            sndr.clone()
        );
        clippit_animation.start();
        let mut ani : HashMap<String, AnimationService> = HashMap::new();
        ani.insert(
            "clippit".to_string(),
            clippit_animation
        );

        let app = TemplateApp{
            state: shared.clone(),
            animations: ani
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

        //let mut state = self.state.lock().unwrap().deref_mut().clone();
        let mut state = self.state.lock().unwrap();
        
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
                
            let clippit_animation = self.animations.get("clippit").unwrap();
            clippit_animation.render_animation(ui);

            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading(clippit_animation.get_current_animation_name());

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

            if ui.button("switch_mode").clicked(){
                state.mode = 
                    match state.mode {
                        AnimationServiceMode::Idle => AnimationServiceMode::Active,
                        AnimationServiceMode::Active => AnimationServiceMode::Idle,
                    };
                    dbg!(&state.mode);
                clippit_animation.set_mode(state.mode.clone());
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
