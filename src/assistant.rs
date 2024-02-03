

use openai_dive::v1::api::Client;
use tokio::sync::broadcast::Sender;

use crate::{actions::DispatchActions, models::QuestionResponse};



#[derive(Clone)]
pub struct AssistantService {
    client: Client,
    sender: Sender<DispatchActions>
}

impl AssistantService {
    /// Called once before the first frame.
    pub fn new<'a,'b>(
        api_key: &'a str, 
        assistant_id: &'b str,
        sndr : Sender<DispatchActions>
    ) -> Self {     
        let client = Client::new(api_key.to_string());

        let assistant_service = AssistantService{
            client: client,
            sender: sndr
        };

        assistant_service
    }

    pub fn start(&self){

        let _client = self.client.clone();
        let sender = self.sender.clone();
        let mut receiver = self.sender.subscribe();
        tokio::spawn(async move {
            loop{
                let action = receiver.recv().await;
                if action.is_err(){ continue; }
                let action = action.unwrap();
                let question = match action {
                    DispatchActions::AskQuestion(question) => question,
                    _ => continue
                };

                let _ = sender.send(DispatchActions::RespondToQuestion(QuestionResponse{
                    question: question,
                    answer: "where is your god now?".to_owned()
                }));
            }
        });        
    }
}