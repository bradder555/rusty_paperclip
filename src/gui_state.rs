use super::models::QuestionResponse;

pub struct GuiState {
    waiting: bool,
    current_question: String,
    past_question_answers: Vec<QuestionResponse>
}