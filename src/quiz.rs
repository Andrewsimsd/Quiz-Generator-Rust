use crate::question_bank::QuestionBank;
use crate::answer_key::AnswerKey;
use std::collections::HashMap;
use serde::Deserialize;

pub struct Quiz {
    questions: Vec<Question>,
}
impl Quiz {
    // Constructor to create a new Quiz instance
    pub fn new(question_bank: &QuestionBank, answer_key: &AnswerKey) -> Self {
        let questions: Vec<Question> = Vec::new();
        let mut map = HashMap::new();
        map.insert('A', 0);
        map.insert('B', 1);
        map.insert('C', 2);
        map.insert('D', 3);
        let zipped: Vec<_> = (*question_bank).questions.iter().zip((*answer_key).questions.iter()).collect();
        for (question, answer) in zipped{
            println!("{:#?}", question);
        }
        Quiz{questions}
    }
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