use crate::question_bank::QuestionBank;
use crate::answer_key::AnswerKey;

pub struct QuizBuilder {
    question_bank: QuestionBank,
    answer_keys: Vec<AnswerKey>
}
impl QuizBuilder {
    pub fn new(question_bank: QuestionBank, answer_keys: Vec<AnswerKey>) -> Self {
        QuizBuilder {
            question_bank,
            answer_keys,
        }
    }
    pub fn generate_quizes(&mut self, num: usize, output_dir: String){
        for answer_key in &self.answer_keys{
            
        }
    }
}