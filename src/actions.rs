use crate::{models::QuestionResponse, animation};

use animation::models::FrameInfo;

#[derive(Clone, Debug)]
pub enum DispatchActions {
    AskQuestion(String),
    RespondToQuestion(QuestionResponse),
    UpdateFrame(FrameInfo)
}