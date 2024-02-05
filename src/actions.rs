use crate::models::QuestionResponse;

#[derive(Clone, Debug)]
pub enum DispatchActions {
    AskQuestion(String),
    RespondToQuestion(QuestionResponse),
    QuestionTextChanged(String),
    NewAnimationStarted(String),
    NewFrameToRender
}