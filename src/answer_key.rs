use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AnswerKey {
    #[serde(rename = "question")]
    questions: Vec<Question>,
}

#[derive(Debug, Deserialize)]
pub struct Question {
    #[serde(rename = "answer")]
    answer: String,
}