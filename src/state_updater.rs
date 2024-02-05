use std::sync::Arc;
use std::sync::Mutex;

use tokio::sync::broadcast::Sender;
use tokio::sync::broadcast::Receiver;


use crate::animation::models::AnimationServiceMode;
use crate::app::ClippitGptAppShared;
use crate::{actions::DispatchActions};

pub struct StateUpdater{
    app_state: Arc<Mutex<ClippitGptAppShared>>,
    sender: Sender<DispatchActions>,
    app_ctx: egui::Context
}

impl StateUpdater{
    pub fn new(
        app_state: Arc<Mutex<ClippitGptAppShared>>, 
        sender: Sender<DispatchActions>,
        app_ctx: egui::Context
    ) -> Self {
        StateUpdater{ 
            app_state,
            sender,
            app_ctx
        }
    }

    pub fn start(&self) {

        let sender = self.sender.clone();
        let mut receiver: Receiver<DispatchActions> = sender.subscribe();
        
        let app_state: Arc<Mutex<ClippitGptAppShared>> = self.app_state.clone();
        let ctx = self.app_ctx.clone();

        tokio::spawn(async move {
            loop {
                let v = receiver.recv().await.unwrap();
                let mut state = app_state.lock().unwrap();
                
                match v {
                    DispatchActions::NewFrameToRender => ctx.request_repaint() , 
                    DispatchActions::AskQuestion(_question) => {
                        state.question_field = "".to_owned();
                        state.mode = AnimationServiceMode::Active;
                        ctx.request_repaint();
                    },
                    DispatchActions::RespondToQuestion(answer) => {
                        state.mode = AnimationServiceMode::Idle;
                        state.answers.push(answer);
                        ctx.request_repaint();
                    },
                    DispatchActions::QuestionTextChanged(txt) => {
                        state.question_field = txt;
                        ctx.request_repaint();
                    }
                    DispatchActions::NewAnimationStarted(ani) => {
                        state.current_animation = ani;
                        ctx.request_repaint();
                    }
                }
            }

        });

    }
}