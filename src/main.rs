use std::fs;
use std::collections::HashMap;
use log::{LevelFilter, info, error};

// Initialize the logger at the module level
pub fn init_logger() {
    env_logger::builder()
        .filter_level(LevelFilter::Info)
        .init();
}


fn main() {
    // Call the init_logger function before any log messages are emitted
    init_logger();

    info!("Reading student data from text file....");

    let result = read_students_txt();
    match result {
        Ok((this_student_vec, this_student_hash)) => info!("Success"),
        Err(msg) => error!("{}", msg),
    }
}


fn read_students_txt() -> Result <(Vec<(String, u8, String)>,HashMap<String, u8>), String> {
    let mut this_student_vec = Vec::<(String, u8, String)>::new();
    let mut this_student_hash = HashMap::<String, u8>::new();

    // read the file
    info!("Reading text file...");
    let contents = fs::read_to_string("students.txt").map_err(|e| e.to_string())?;


    // iterate through the file line by line
    for line in contents.lines() {
        // create a fields string slice
        // split the line
        let fields: Vec<&str> = line.split(", ").collect();
        // get the fields in each column?;
        let name = fields.get(0).ok_or("Failed to get name")?;
        let age = fields.get(1).ok_or("age field no found")?.parse::<u8>().map_err(|e| e.to_string())?;
        let dob = fields.get(2).ok_or("Failed to get dob")?;
        // add them to the vec and add name and age to the hash map
        // vec is expcecting tuple of type (string, u8, String)
        let this_student = (name.to_string(), age, dob.to_string());
        this_student_vec.push(this_student);
        // k: String, V: u8
        this_student_hash.insert(name.to_string(), age);
    }
    // Ok result expects tuple Vec, Hash
    Ok((this_student_vec, this_student_hash))
}