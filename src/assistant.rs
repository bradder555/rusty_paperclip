

use std::{borrow::BorrowMut, clone, time::Duration};

use openai_dive::v1::{api::Client, endpoints::assistants::messages::Messages, resources::{assistant::{assistant::Assistant, message::{CreateMessageParameters, MessageContent, MessageRole}, run::{CreateRunParameters, RunStatus}, thread::CreateThreadParameters}, shared::ListParameters}};
use tokio::sync::broadcast::Sender;

use crate::{actions::DispatchActions, models::QuestionResponse};

pub struct AssistantService {
    client: Client,
    sender: Sender<DispatchActions>,
    running: bool,
    assistant_id: String
}

impl AssistantService {
    /// Called once before the first frame.
    pub fn new(
        api_key: String, 
        assistant_id: String,
        sndr : Sender<DispatchActions>
    ) -> Self {

        let mut client = Client::new(api_key.to_string());

        let assistant_service = AssistantService{
            client: client.to_owned(),
            sender: sndr.to_owned(),
            running: false,
            assistant_id: assistant_id.to_owned()
        };

        assistant_service
    }

    pub fn start(&mut self){
        if self.running {return}
        self.running = true;

        let sender = self.sender.clone();
        let assistant_id = self.assistant_id.to_owned();
        let client = self.client.clone();

        tokio::spawn(async move {
            let mut receiver = sender.subscribe();

            let assistant;
            // keep trying on network issue
            loop {
                let ret = client.assistants().retrieve(&assistant_id).await;
                if let Ok(ret) = ret{
                    assistant = ret;
                    break;
                }
                dbg!(ret.unwrap_err());
                tokio::time::sleep(Duration::from_secs(3)).await;
            }

            // don't import as Threads, could cause confusion with Thread
            let _thread;
            loop {
                let ret = client
                    .assistants()
                    .threads()
                    .create(
                        CreateThreadParameters{
                            messages: None, 
                            metadata:None 
                        }
                    ).await;
                
                if let Ok(ret) = ret{
                    _thread = ret;
                    break;
                }
                dbg!(ret.unwrap_err());
                tokio::time::sleep(Duration::from_secs(3)).await;
            }

            loop{
                let action = receiver.recv().await;
                if action.is_err(){ continue; }
                let action = action.unwrap();
                let question = match action {
                    DispatchActions::AskQuestion(question) => question,
                    _ => continue
                };

                let msg;
                loop {
                    let ret = client
                        .assistants()
                        .messages()
                        .create(
                            &_thread.id,
                            CreateMessageParameters{
                                role: MessageRole::User,
                                content: question.to_owned(),
                                file_ids: None,
                                metadata: None,
                            }
                        ).await;
                    
                    if let Ok(ret) = ret{
                        msg = ret;
                        break;
                    }
                    dbg!(ret.unwrap_err());
                    tokio::time::sleep(Duration::from_secs(3)).await;
                }

                let mut run;
                loop {
                    let ret = client
                        .assistants()
                        .runs()
                        .create(
                            &_thread.id, 
                            CreateRunParameters { 
                                assistant_id: assistant.id.to_owned(), 
                                model: None, 
                                instructions: None, 
                                tools: None
                            }
                        ).await;
                    
                    if let Ok(ret) = ret{
                        run = ret;
                        break;
                    }
                    dbg!(ret.unwrap_err());
                    tokio::time::sleep(Duration::from_secs(3)).await;
                }

                loop {
                    let ret = client
                        .assistants()
                        .runs()
                        .retrieve(&_thread.id, &run.id)
                        .await;
                    
                    if let Ok(ret) = ret{
                        run = ret.clone();
                        if run.status == RunStatus::Completed {break}
                    } else {
                        dbg!(ret.unwrap_err());
                    }
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }

                let mut msgs;
                loop {
                    let ret = 
                        client
                        .assistants()
                        .messages()
                        .list(
                            &_thread.id,
                            Some(ListParameters {
                                limit: Some(1), 
                                order: Some("desc".to_owned()), 
                                after: None,
                                before: None
                            })
                        ).await;
                    
                    if let Ok(ret) = ret{
                        msgs = ret;
                        break;
                    } else {
                        dbg!(ret.unwrap_err());
                    }
                    tokio::time::sleep(Duration::from_secs(3)).await;
                }

                let message = msgs.data.get(0).expect("Something went wrong, message is none!");
                let content = message.content.get(0).expect("message has no content");
                let text = match content {
                    MessageContent::ImageFile(_) => panic!("Not expecting an image file"),
                    MessageContent::Text(text) => &text.text.value
                };

                let _ = sender.send(DispatchActions::RespondToQuestion(QuestionResponse{
                    question: question,
                    answer: text.to_owned()
                }));
            }
        });        
    }
}