//! In this library we pretend to create a crate to be able to log different objects into a file
//! Ultimately we are going to have two big functionalities: when to write and one to read from a given file

use std::{fs, fs::File, io::{Write, BufWriter, BufReader}};
use serde::Serialize;
use serde_json::{Deserializer, Value, Error};

/// Write the data object passed into the specified file
/// 
/// This function takes as paramater a Serializable data and properly writes it into a file
/// that is specified in the path paramater.
/// 
/// In case the file does not exist it automatically creates it and then writes in it, and in case it does exist,
/// it appends the current object in the file.
/// 
///  # Panics
/// 
/// This function will panic in any case of an IO error
pub fn write_data_into_file<T: Serialize>(path: &str, data: T) -> std::io::Result<()> {

    let file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)?;

    let mut file = BufWriter::new(file);

    let data_string = serde_json::to_string(&data);

    file.write_all(&data_string.unwrap().as_bytes())?;
    file.flush()?;

    Ok(())
    
}


/// Function to retrieve all objects from a file
/// 
/// This function takes as a paramater a path to a file that holds JSON objects and then retrieves
/// all the objects and return them inside an iterator to the function caller.
/// 
/// Apperantly the function is robust enough to even be able to read an empty file and return with no problems, but so far
/// we don't really know how many objecst are retrieved from the file
/// 
///  # Panics
/// 
/// This function panics in case the file does not exist.
/// 
/// TODO: see if its actually possible to retrieve n objects only
pub fn retrieve_from_log_n(path: &str) -> Result<impl Iterator<Item = Result<Value, Error>>, std::io::Error> {

    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // this can be a one liner
    let deserilizer = Deserializer::from_reader(reader);
    // the type of the below variable is serde_json::StreamDeserializer<'_, serde_json::de::IoRead<BufReader<File>>, Value>
    // more information here https://docs.rs/serde_json/latest/serde_json/struct.StreamDeserializer.html
    let iterator = deserilizer.into_iter::<Value>();
    
    Ok(iterator)

}