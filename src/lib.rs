//! In this library we pretend to create a crate to be able to log different objects into a file
//! Ultimately we are going to have two big functionalities: one to write and one to read from a given file

use std::{fs, fs::File, io::{Write, BufWriter, BufReader}};
use serde::Serialize;
use serde_json::{Deserializer, Value as JSON_Value, Error};
//use rmpv::Value as RMP_Value;
//use rmp_serde::decode::Deserializer as RMP_Deserializer;

pub struct Logger {
    pub file: File,
    //file: File,
    file_name: String,
}

impl Logger {

    /// Creates the Logger object that has a reference for the File object created and the name given to the file
    /// 
    /// This function will create the new file and attatch a file object to it that will be used to write in the file.
    /// The reading function only uses the path to retrieve from the file.
    /// If the file already exists it only opens it in append mode so that you can keep writing on it.
    /// 
    /// # Panics
    /// 
    /// It panics with it`s not able to create the file
    pub fn new(path: &str) -> Logger {

        // creating the new log file
        let file = fs::OpenOptions::new().create(true).append(true).open(path);

        Logger { 
            file: file.unwrap(),
            file_name: path.to_string(),  
        }
    
    }

    /// Write the data object passed into the specified file
    /// 
    /// This function takes as paramater a Serializable data and properly writes it into a file
    /// that was created in the logger creation.
    /// 
    /// The file is sure to exist and the function always append to the file, even with multiple executions
    /// 
    ///  # Panics
    /// 
    /// This function will panic in any case of an IO error
    pub fn write_data<T: Serialize>(&self, data: T) -> std::io::Result<()> {
        
        let mut writer = BufWriter::new(&self.file);

        // the to_vec funciton will give us a byte vector that can go directly to the writer
        let data_string = serde_json::to_vec(&data);

        writer.write_all(&data_string.unwrap())?;
        writer.flush()?;

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
    pub fn retrieve_iterator(&self) -> Result<impl Iterator<Item = Result<JSON_Value, Error>>, std::io::Error> {

        let file = File::open(&self.file_name)?;
        let reader = BufReader::new(file);

        // this can be a one liner
        let deserilizer = Deserializer::from_reader(reader);
        // the type of the below variable is serde_json::StreamDeserializer<'_, serde_json::de::IoRead<BufReader<File>>, Value>
        // more information here https://docs.rs/serde_json/latest/serde_json/struct.StreamDeserializer.html
        let iterator = deserilizer.into_iter::<JSON_Value>();
        
        Ok(iterator)
    }
}

pub struct BinLogger {
    file: File,
    file_name:  String,
}

impl BinLogger {

    pub fn new(path: &str) -> BinLogger {
        // creating the new log file
        let file = fs::OpenOptions::new().create(true).append(true).open(path);

        BinLogger { 
            file: file.unwrap(),
            file_name: path.to_string(),  
        }    
    }

    pub fn write_data<T: Serialize>(&self, data: &T) -> std::io::Result<()> {
        
        let mut writer = BufWriter::new(&self.file);
    
        let bytes = rmp_serde::to_vec_named(&data);

        writer.write_all(&bytes.unwrap())?;
        writer.flush()?;

        Ok(())
        
    }

    pub fn retrieve_iterator(&self) {
        
        println!("TODO! {}", self.file_name);

    }

}