use crate::question_bank::QuestionBank;
use crate::answer_key::AnswerKey;

pub struct Quiz {
    questions: Vec<Question>,
}
impl Quiz {
    // Constructor to create a new Quiz instance
    pub fn new(question_bank: &QuestionBank, answer_key: &AnswerKey) -> Self {
        let questions: Vec<Question> = Vec::new();
        let zipped: Vec<_> = (*question_bank).questions.iter().zip((*answer_key).questions.iter()).collect();
        for (question, answer) in zipped{
            println!("{:#?}", question);
        }
        Quiz{questions}
    }
    // Method to serialize the struct to an XML file
    /* fn save_to_xml(&self, file_path: &str) -> io::Result<()> {
        // Serialize the struct to an XML string
        let xml_data = to_string(&self)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

        // Write the XML data to a file
        let mut file = File::create(file_path)?;
        file.write_all(xml_data.as_bytes())?;

        Ok(())
    } */
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