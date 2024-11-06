use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;
use serde_xml_rs::from_reader;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct AnswerKey {
    #[serde(rename = "question")]
    pub questions: Vec<Question>,
}
impl AnswerKey {
    // Function to create an AnswerKey instance from an XML file
    pub fn new(file_path: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
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
    pub answer: String,
}