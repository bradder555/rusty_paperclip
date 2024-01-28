use std::collections::HashMap;

use std::ops::DerefMut;
use std::time::Duration;
use std::sync::Arc;
use std::sync::Mutex;
use crate::actions::DispatchActions;
use crate::animation::models::AnimationServiceMode;
use crate::animation::service::AnimationService;
use egui::Color32;
use egui::Id;
use egui::Sense;
use egui::Stroke;
use egui::ViewportCommand;
use egui::Visuals;
use tokio::sync::broadcast;

#[derive(Clone)]
pub struct ClippitGptAppShared{
    label: String,
    value: f32,
    mode: AnimationServiceMode
}

pub struct ClippitGptApp {
    state: Arc<Mutex<ClippitGptAppShared>>,
    animations: HashMap<String, AnimationService>
}


impl ClippitGptApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {

        let (sndr, _) = broadcast::channel::<DispatchActions>(50);
        let mut receiver = sndr.subscribe();

        let ctx = cc.egui_ctx.clone();
        tokio::spawn(
            async move {
                loop{
                    let v = receiver.recv().await.unwrap();
                    
                    match v {
                        DispatchActions::UpdateFrame => ctx.request_repaint(),
                        DispatchActions::AskQuestion(question) => println!("asked {}", question),
                        DispatchActions::RespondToQuestion(answer) => println!("{:?}",answer)
                    }
                }
            }
        );
        
        let shared = Arc::new(
            Mutex::new(
                ClippitGptAppShared {
                    label: "Ask me something!".to_owned(),
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

        let app = ClippitGptApp{
            state: shared.clone(),
            animations: ani
        };

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

impl eframe::App for ClippitGptApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array() // Make sure we don't paint anything behind the rounded corners
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        //let mut state = self.state.lock().unwrap().deref_mut().clone();
        let mut state = self.state.lock().unwrap();

        let panel_frame = egui::Frame {
            fill: Color32::TRANSPARENT,
            stroke: Stroke::new(10.0, Color32::TRANSPARENT),
            ..Default::default()
        };

        egui::CentralPanel::default().frame(panel_frame).show(ctx, |ui| {
            if ui.interact(ui.max_rect(), Id::new("window-drag"), Sense::click()).is_pointer_button_down_on() {
                ui.ctx().send_viewport_cmd(ViewportCommand::StartDrag);
            }
            
            let clippit_animation = self.animations.get("clippit").unwrap();
            ui.horizontal_centered(|ui|{
                clippit_animation.render_animation(ui);
            });
            if ui.button("switch_mode").clicked(){
                state.mode = 
                    match state.mode {
                        AnimationServiceMode::Idle => AnimationServiceMode::Active,
                        AnimationServiceMode::Active => AnimationServiceMode::Idle,
                    };
                    dbg!(&state.mode);
                clippit_animation.set_mode(state.mode.clone());
            }

            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading(clippit_animation.get_current_animation_name());

            ui.horizontal(|ui| {
            
                ui.text_edit_singleline(&mut state.label);
            });

            egui::warn_if_debug_build(ui);
        });
        
    }
}
