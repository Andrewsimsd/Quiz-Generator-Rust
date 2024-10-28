use crate::question_bank::QuestionBank;
use crate::answer_key::AnswerKey;
use std::collections::HashMap;

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
pub struct Question {
    pub id: String,
    pub qtype: String,
    pub text: String,
    pub image: Option<String>,
    pub choices: Option<Choices>,
    pub answer: Option<String>,
}

pub struct Choices {
    pub choice: Vec<Choice>,
}

pub struct Choice {
    pub correct: Option<bool>,
    pub text: String,
}