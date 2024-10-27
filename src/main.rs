use serde_xml_rs::from_reader;
use std::fs::File;
use std::io::BufReader;

mod answer_key;
mod quiz;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open quiz the XML file
    let quiz_file: File = File::open(r"C:\Users\andre\Documents\GitHub\Quiz-Generator-Rust\Quizzes\Quiz1.xml")?;
    let quiz_reader: BufReader<File> = BufReader::new(quiz_file);

    // Parse the XML file into the Quiz struct
    let quiz: quiz::Quiz = from_reader(quiz_reader)?;

    // Open answer_key the XML file
    let ak_file: File = File::open(r"C:\Users\andre\Documents\GitHub\Quiz-Generator-Rust\answer_keys\answer_key_1.xml")?;
    let ak_reader: BufReader<File> = BufReader::new(ak_file);

    // Parse the XML file into the Quiz struct
    let ak: answer_key::AnswerKey = from_reader(ak_reader)?;
    // Print the parsed data
    println!("{:#?}", quiz);
    println!("{:#?}", ak);

    Ok(())
}
