use crate::{models::QuestionResponse, animation};

use animation::models::FrameInfo;

pub enum DispatchActions {
    AskQuestion(String),
    RespondToQuestion(QuestionResponse),
    UpdateFrame(FrameInfo)
}