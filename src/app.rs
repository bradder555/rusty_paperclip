use std::collections::HashMap;

use std::time::Duration;
use std::sync::Arc;
use std::sync::Mutex;
use crate::actions::DispatchActions;
use crate::animation::models::AnimationServiceMode;
use crate::animation::service::AnimationService;


use egui::Color32;
use egui::Id;
use egui::Layout;
use egui::Margin;
use egui::Rounding;
use egui::Sense;
use egui::Separator;
use egui::Stroke;
use egui::Ui;
use egui::ViewportCommand;
use egui_extras::Size;

use egui_extras::StripBuilder;
use tokio::sync::broadcast;

#[derive(Clone)]
pub struct ClippitGptAppShared{
    question_field: String,
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
                    question_field: "".to_owned(),
                    mode: AnimationServiceMode::Idle
                }
            )
        );

        let config_data = include_str!("../assets/animations.yaml");
        let image_data = include_bytes!("../assets/clippy.png");
        let image_data = image_data.to_vec();
        
        let clippit_animation = AnimationService::new(
            cc.egui_ctx.clone(),
            config_data.to_owned(),
            image_data,
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
                    ctx_rp.request_repaint();
                }
            }
        ); 
        let mut visuals = egui::Visuals::dark().clone();
        visuals.override_text_color = Some(Color32::WHITE);
        cc.egui_ctx.set_visuals(visuals); 

        app
    }
}

impl eframe::App for ClippitGptApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array() // Make sure we don't paint anything behind the rounded corners
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        //let mut state = self.state.lock().unwrap().deref_mut().clone();
        let mut state = self.state.lock().unwrap();

        let panel_frame = egui::Frame {
            fill: Color32::from_rgba_premultiplied(0, 0, 0, 180),
            stroke: Stroke::new(1.0, Color32::BLACK),
            rounding: Rounding::same(30.0),
            inner_margin: Margin::same(30.0),
            outer_margin: Margin::same(10.0),
            ..Default::default()
        };

        egui::CentralPanel::default().frame(panel_frame).show(ctx, |ui| {
            if ui.interact(ui.max_rect(), Id::new("window-drag"), Sense::click()).is_pointer_button_down_on() {
                ui.ctx().send_viewport_cmd(ViewportCommand::StartDrag);
            }
            
            let clippit_animation = self.animations.get("clippit").unwrap();
            ui.horizontal(|ui|{
                StripBuilder::new(ui)
                    .size(Size::remainder())
                    .size(Size::exact(100.0))
                    .horizontal(|mut strip|{
                        strip.cell(|ui|{
                            ui.label(clippit_animation.get_current_animation_name());
                        });
                        strip.strip(|builder|{
                            builder
                                .size(Size::exact(100.0))
                                .vertical(|mut strip|{
                                    strip.cell(|ui|{
                                        clippit_animation.render_animation(ui);
                                    })
                                });
                        });
                    })
                
            });
            
            /*
            if ui.button("switch_mode").clicked(){
                state.mode = 
                    match state.mode {
                        AnimationServiceMode::Idle => AnimationServiceMode::Active,
                        AnimationServiceMode::Active => AnimationServiceMode::Idle,
                    };
                    dbg!(&state.mode);
                clippit_animation.set_mode(state.mode.clone());
            }
            */
            
            ui.label("Ask ClippitGPT Something:");
            ui.horizontal(|ui| {
                ui.add_enabled(
                    if state.mode == AnimationServiceMode::Idle {true} else {false}, 
                    |ui: &mut Ui| {
                        ui.text_edit_singleline(&mut state.question_field  )
                    }
                );
                ui.add_enabled(
                    if state.mode == AnimationServiceMode::Idle {true} else {false}, 
                    |ui: &mut Ui| {
                        ui.button("Ask!")
                    }
                );
            });

            ui.label("Conversation History:");
            egui::ScrollArea::both()
            .hscroll(false)
            .show(ui, |ui|{
                for i in 1..20  {
                    ui.label(format!("i'm a question {}", i));
                    ui.colored_label(Color32::RED, format!("i'm a response {}", i));
                    ui.add(Separator::default());
                    ui.add_space(10.0);
                }
                
            });
            
            ui.with_layout(Layout::left_to_right(egui::Align::Max), |ui| {
                egui::warn_if_debug_build(ui);
            }); 
            
        });
        
    }
}
