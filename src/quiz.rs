use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Quiz {
    #[serde(rename = "question")]
    pub questions: Vec<Question>,
}

#[derive(Debug, Deserialize)]
pub struct Question {
    #[serde(rename = "id", default)]
    pub id: String,
    #[serde(rename = "type", default)]
    pub qtype: String,
    pub text: String,
    #[serde(rename = "image", default)]
    pub image: Option<String>,
    #[serde(rename = "choices", default)]
    pub choices: Option<Choices>,
    #[serde(rename = "answer", default)]
    pub answer: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Choices {
    #[serde(rename = "choice")]
    pub choice: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
pub struct Choice {
    #[serde(rename = "correct", default)]
    pub correct: Option<bool>,
    #[serde(rename = "$value")]
    pub text: String,
}