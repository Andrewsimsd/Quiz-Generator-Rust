use answer_key::AnswerKey;
use log::{info, warn, debug, error};
use fern::Dispatch;
use chrono::Local;
use glob::glob;
use std::error::Error;
use std::path::PathBuf;
use std::env;

mod answer_key;
mod question_bank;
mod quiz;
mod quiz_builder;

fn main() -> Result<(), Box<dyn Error>> {

    setup_logger().expect("Failed to initialize logger");
    let current_dir: PathBuf = env::current_dir()?;
    let mut question_bank_dir = current_dir.clone();
    question_bank_dir.push("question_bank");
    question_bank_dir.push("question_bank.xml");
    let mut answer_key_dir = current_dir.clone();
    answer_key_dir.push("answer_keys");

    let question_bank= question_bank::QuestionBank::new(question_bank_dir)?;
    let answer_keys: Vec<AnswerKey> = get_answer_keys_from_dir(answer_key_dir)?;
    
    let quiz_builder = quiz_builder::QuizBuilder::new(question_bank, answer_keys);
    

    // Print the current working directory
    info!("Done.");
    Ok(())
}

fn get_answer_keys_from_dir(dir: PathBuf) -> Result<Vec<answer_key::AnswerKey>, Box<dyn Error>> {
    let mut answer_keys: Vec<answer_key::AnswerKey> = Vec::new();
    
    let files: Vec<PathBuf> = get_answer_key_filepaths(dir)?; 

    if files.is_empty() {
        warn!("No answer key files found.");
    } else {
        // Map each file path (String) to an AnswerKey and store them in `answer_keys`
        answer_keys = files.into_iter()
            .map(answer_key::AnswerKey::new)
            .filter_map(Result::ok)  // Only keep Ok values, discard Err
            .collect();
    }
    Ok(answer_keys)
}


fn get_answer_key_filepaths(dir: PathBuf) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    // Define the pattern: look for "answer_key_*.xml" in the specified directory
    let mut pattern = dir.clone();
    pattern.push("answer_key_*.xml");
    
    let mut matching_files = Vec::new();

    // Iterate over the glob results, returning an error if glob fails
    for entry in glob(&pattern.to_str().unwrap())? {
        match entry {
            Ok(path) => matching_files.push(path),
            Err(e) => return Err(Box::new(e)), // Return the error if file access fails
        }
    }
    
    Ok(matching_files) // Return the vector of matching files on success
}


fn setup_logger() -> Result<(), fern::InitError> {
    Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Info) // Adjust log level here
        .chain(std::io::stdout())       // Print to stdout
        .chain(fern::log_file("output.log")?) // Write to a file
        .apply()?;
    Ok(())
}
