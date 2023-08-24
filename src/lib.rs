//! In this library we pretend to create a crate to be able to log different objects into a file
//! Ultimately we are going to have two big functionalities: when to write and one to read from a given file

use std::{fs, io::{Write, BufWriter, BufReader}};

use serde::{Serialize, Deserialize};


/// # Function to write into the log
/// We are going to assume that we must write into a file one specefic object per time
/// TODO: turn a paramater a generic type deserializable, so that we deserielize it here and write it on the file
pub fn write_data_into_file<T: Serialize>(path: &str, data: T) -> std::io::Result<()> {

    let file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)?;

    let mut file = BufWriter::new(file);

    let data_string = serde_json::to_string(&data);

    //trying to write everything in one go
    file.write_all(&data_string.unwrap().as_bytes())?;

    file.flush()?;

    Ok(())
}


/// # Function to retrieve an especific amount of objects from given log path
/// this function to be modular enough for us to set the amount of objects we want to retreive
/// ideally we should be able to give a default good value to n if nothing is passed
/// TODO: see if its actually possible to retrieve n objects only
/// TODO: see what the function should return
/// TODO: a first version to just return the iterator
pub fn retrieve_from_log_n(path: &str, n: Option<u32>) {//-> serde_json::StreamDeserializer<'_, serde_json::de::IoRead<BufReader<File>>, serde_json::Value> {
    println!("{} {}", path, n.unwrap_or(4));
    println!("TODO");
}