use serde::Deserialize;
use serde_xml_rs::from_reader;
use std::fs::File;
use std::io::BufReader;

#[derive(Debug, Deserialize)]
struct Quiz {
    #[serde(rename = "question")]
    questions: Vec<Question>,
}

#[derive(Debug, Deserialize)]
struct Question {
    #[serde(rename = "id", default)]
    id: String,
    #[serde(rename = "type", default)]
    qtype: String,
    text: String,
    #[serde(rename = "image", default)]
    image: Option<String>,
    #[serde(rename = "choices", default)]
    choices: Option<Choices>,
    #[serde(rename = "answer", default)]
    answer: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Choices {
    #[serde(rename = "choice")]
    choice: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    #[serde(rename = "correct", default)]
    correct: Option<bool>,
    #[serde(rename = "$value")]
    text: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open the XML file
    let file = File::open(r"C:\Users\andre\OneDrive\Documents\GitHub\Quiz-Generator-Rust\Quiz1.xml")?;
    let reader = BufReader::new(file);

    // Parse the XML file into the Quiz struct
    let quiz: Quiz = from_reader(reader)?;

    // Print the parsed data
    println!("{:#?}", quiz);

    Ok(())
}
