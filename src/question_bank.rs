use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;
use serde_xml_rs::from_reader;
use crate::quiz::Question;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct QuestionBank {
    #[serde(rename = "question")]
    pub questions: Vec<Question>,
}
impl QuestionBank {
    // Function to create a Quiz instance from an XML file
    pub fn new(file_path: PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        // Read the XML file
        let file: File = File::open(file_path)?;
        let reader: BufReader<File> = BufReader::new(file);

        // Parse the XML file into the Quiz struct
        let bank: QuestionBank = from_reader(reader)?;

        Ok(bank)
    }
}
