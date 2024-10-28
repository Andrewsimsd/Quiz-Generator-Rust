mod answer_key;
mod question_bank;
mod quiz;
mod Quiz;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let question_bank= question_bank::QuestionBank::from_xml_file(r"C:\Users\andre\Documents\GitHub\Quiz-Generator-Rust\question_bank\question_bank.xml")?;
    let answer_key = answer_key::AnswerKey::from_xml_file(r"C:\Users\andre\Documents\GitHub\Quiz-Generator-Rust\answer_keys\answer_key_1.xml")?;
    let quiz = quiz::Quiz::new(&question_bank, &answer_key);

    // Print the parsed data
    //println!("{:#?}", question_bank);
    //println!("{:#?}", answer_key);

    Ok(())
}
