use log::{info, warn, debug, error};
use fern::Dispatch;
use chrono::Local;

mod answer_key;
mod question_bank;
mod quiz;
mod Quiz;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    setup_logger().expect("Failed to initialize logger");
    info!("This is an info message.");
    warn!("This is a warning.");
    error!("This is an error!");
    let question_bank= question_bank::QuestionBank::from_xml_file(r"C:\Users\andre\Documents\GitHub\Quiz-Generator-Rust\question_bank\question_bank.xml")?;
    let answer_key = answer_key::AnswerKey::from_xml_file(r"C:\Users\andre\Documents\GitHub\Quiz-Generator-Rust\answer_keys\answer_key_1.xml")?;
    let quiz = quiz::Quiz::new(&question_bank, &answer_key);

    // Print the parsed data
    //println!("{:#?}", question_bank);
    //println!("{:#?}", answer_key);

    Ok(())
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
