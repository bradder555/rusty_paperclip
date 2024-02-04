#[derive(Clone, Debug)]
pub struct QuestionResponse {
    pub question: String,
    pub answer: String 
}


#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct AppConfig{
    pub open_ai_api_key: String,
    pub assistant_id: String
}