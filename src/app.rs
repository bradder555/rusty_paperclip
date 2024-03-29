use std::collections::HashMap;

use std::env::current_exe;
use std::fs;
use std::path::PathBuf;
use std::time::Duration;
use std::sync::Arc;
use std::sync::Mutex;
use crate::actions::DispatchActions;
use crate::animation::models::AnimationServiceMode;
use crate::animation::service::AnimationService;
use crate::assistant::AssistantService;
use crate::models::AppConfig;
use crate::models::QuestionResponse;
use crate::state_updater::StateUpdater;


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
use tokio::sync::broadcast::Sender;

#[derive(Clone)]
pub struct ClippitGptAppShared{
    pub question_field: String,
    pub mode: AnimationServiceMode,
    pub answers: Vec<QuestionResponse>,
    pub current_animation: String 
}

pub struct ClippitGptApp {
    state: Arc<Mutex<ClippitGptAppShared>>,
    animations: HashMap<String, AnimationService>,
    mpmc_channel: Sender<DispatchActions>
}

impl ClippitGptApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let current_exe_path = PathBuf::from(current_exe().unwrap());
        let exe_folder = current_exe_path.as_path().parent().unwrap();
        let config_path = exe_folder.join("config.yaml");
        let config = fs::read_to_string(config_path).expect("config.yaml file not found!");
        let config: AppConfig = serde_yaml::from_str(&config).expect("unable to parse config file!");

        let (sndr, _) = broadcast::channel::<DispatchActions>(50);
        
        let shared = Arc::new(
            Mutex::new(
                ClippitGptAppShared {
                    question_field: "".to_owned(),
                    mode: AnimationServiceMode::Idle,
                    answers: Vec::new(),
                    current_animation: "".to_owned()
                }
            )
        );

        let config_data = include_str!("../assets/animations.yaml");
        let image_data = include_bytes!("../assets/clippy.png");
        let image_data = image_data.to_vec();
        
        let mut clippit_animation = AnimationService::new(
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

        let mut ass_service = AssistantService::new(
            config.open_ai_api_key.to_owned(),
            config.assistant_id.to_owned(),
            sndr.clone()
        );

        ass_service.start();

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

        let app = ClippitGptApp{
            state: shared.clone(),
            animations: ani,
            mpmc_channel: sndr.clone()
        };

        StateUpdater::new(
            app.state.clone(),
            sndr.clone(),
            cc.egui_ctx.clone()
        ).start();

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
        let mut state;
        {
            let state_ = self.state.lock().unwrap();
            state = state_.clone();
            drop(state_);
        }
        
        //let mut state = self.state.lock().unwrap();
        let sender = &self.mpmc_channel;
        
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
                            ui.vertical(|ui|{
                                ui.label(state.current_animation.to_owned());
                                ui.label(
                                    if state.mode == AnimationServiceMode::Idle {
                                        "Clippy Idle".to_owned() 
                                    } else {
                                        "Clippy Active".to_owned() 
                                    });
                            });
                            
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
                      
            ui.label("Ask ClippitGPT Something:");
            ui.horizontal(|ui| {
                ui.add_enabled(
                    if state.mode == AnimationServiceMode::Idle {true} else {false}, 
                    |ui: &mut Ui| {
                        let txt = ui.text_edit_singleline(&mut state.question_field  );
                        txt.ctx.input(|i|{
                            if i.key_pressed(egui::Key::Enter) {
                                sender.send(DispatchActions::AskQuestion(state.question_field.to_owned())).expect("couldn't ask question!");
                            }
                        });
                        if txt.changed(){
                            sender.send(DispatchActions::QuestionTextChanged(state.question_field.clone())).expect("couldn't update text");
                        }
                        txt
                    }
                );
                ui.add_enabled(
                    if state.mode == AnimationServiceMode::Idle {true} else {false}, 
                    |ui: &mut Ui| {
                        let btn = ui.button("Ask!");
                        if btn.clicked(){
                            sender.send(DispatchActions::AskQuestion(state.question_field.to_owned())).expect("couldn't ask question!");
                        };
                        btn
                    }
                );
            });

            ui.add_space(20.0);
            ui.label("Conversation History:");
            ui.add(Separator::default());

            egui::ScrollArea::both()
            .hscroll(false)
            .show(ui, |ui|{
                for qr in state.answers.iter().rev(){
                    ui.label(&qr.question);
                    ui.colored_label(Color32::RED, &qr.answer);
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
