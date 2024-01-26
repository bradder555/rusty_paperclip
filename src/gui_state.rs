use super::models::QuestionResponse;

pub struct GuiState {
    pub waiting: bool,
    pub current_question: String,
    pub past_question_answers: Vec<QuestionResponse>
}