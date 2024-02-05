use std::sync::Arc;
use std::sync::Mutex;

use tokio::sync::broadcast::Sender;
use tokio::sync::broadcast::Receiver;

use crate::app;
use crate::app::ClippitGptAppShared;
use crate::{actions::DispatchActions, models::QuestionResponse};

pub struct StateUpdater{
    app_state: Arc<Mutex<ClippitGptAppShared>>,
    sender: Sender<DispatchActions>
}

impl StateUpdater{
    pub fn new(
        app_state: Arc<Mutex<ClippitGptAppShared>>, 
        sender: Sender<DispatchActions>
    ) -> Self {
        StateUpdater{ 
            app_state,
            sender
        }
    }

    pub fn start(&self) {

        let sender = self.sender.clone();
        let mut receiver: Receiver<DispatchActions> = sender.subscribe();
        
        let app_state: Arc<Mutex<ClippitGptAppShared>> = self.app_state.clone();

        tokio::spawn(async move {
            loop {
                let v = receiver.recv().await.unwrap();
                let state = app_state.
                match v {
                    DispatchActions::UpdateFrame => () , 
                    DispatchActions::AskQuestion(question) => (),
                    DispatchActions::RespondToQuestion(answer) => println!("{:?}",answer),
                    DispatchActions::QuestionTextChanged(txt) => app_state.
                }
            }

        });

    }
}