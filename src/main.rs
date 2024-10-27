use serde_xml_rs::from_reader;
use std::fs::File;
use std::io::BufReader;

mod answer_key;
mod quiz;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let quiz: quiz::Quiz = quiz::Quiz::from_xml_file(r"C:\Users\andre\Documents\GitHub\Quiz-Generator-Rust\Quizzes\Quiz1.xml")?;
    let answer_key: answer_key::AnswerKey = answer_key::AnswerKey::from_xml_file(r"C:\Users\andre\Documents\GitHub\Quiz-Generator-Rust\answer_keys\answer_key_1.xml")?;

    // Print the parsed data
    println!("{:#?}", quiz);
    println!("{:#?}", answer_key);

    Ok(())
}
