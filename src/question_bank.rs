use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;
use serde_xml_rs::from_reader;

#[derive(Debug, Deserialize)]
pub struct QuestionBank {
    #[serde(rename = "question")]
    pub questions: Vec<Question>,
}
impl QuestionBank {
    // Function to create a Quiz instance from an XML file
    pub fn from_xml_file(file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // Read the XML file
        let file: File = File::open(file_path)?;
        let reader: BufReader<File> = BufReader::new(file);

        // Parse the XML file into the Quiz struct
        let bank: QuestionBank = from_reader(reader)?;

        Ok(bank)
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