use crate::{models::QuestionResponse};

#[derive(Clone, Debug)]
pub enum DispatchActions {
    AskQuestion(String),
    RespondToQuestion(QuestionResponse),
    UpdateFrame
}