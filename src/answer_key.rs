use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;
use serde_xml_rs::from_reader;

#[derive(Debug, Deserialize)]
pub struct AnswerKey {
    #[serde(rename = "question")]
    questions: Vec<Question>,
}
impl AnswerKey {
    // Function to create an AnswerKey instance from an XML file
    pub fn from_xml_file(file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // Read the XML file
        let file: File = File::open(file_path)?;
        let reader: BufReader<File> = BufReader::new(file);

        // Parse the XML file into the Quiz struct
        let answer_key: AnswerKey = from_reader(reader)?;

        Ok(answer_key)
    }
}
#[derive(Debug, Deserialize)]
pub struct Question {
    #[serde(rename = "answer")]
    answer: String,
}