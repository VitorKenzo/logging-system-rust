//! In this library we pretend to create a crate to be able to log different objects into a file
//! Ultimately we are going to have two big functionalities: one to write and one to read from a given file
//! We will give to ways to log the objects. One will be in the human-readable JSON format and the other
//! will be using the binary format MessagePackr

use std::{fs, fs::File, io::{Write, BufWriter, BufReader, ErrorKind}, marker::{self, PhantomData}};
use serde::{Serialize, Deserialize, de::DeserializeOwned};
use serde_json::{Deserializer as JSON_Deserializer, Value as JSON_Value, Error as JSONError};
use rmp_serde;
use crc32fast;

/// A basic logger for serializing and deserializing data to and from a JSON log file.
///
/// The `JSONLogger` struct allows you to log data to a JSON file. It provides methods for
/// creating a new logger, writing data to the log file, and retrieving an iterator
/// over the logged items.
/// 
/// It can be used to serialize any struct
/// for example as long as you derive the Serialize and Deserialize traits from serde such as
/// 
/// #[derive(Deserialize, Serialize)]
/// Struct test {...}
///
/// # Examples
///
/// ```
/// use logging_system::JSONLogger;
///
/// // Create a new Logger for logging JSON values to a file named "json_test.log".
/// let logger_json = JSONLogger::new("json_test.log");
///
/// // Write data to the log file.
/// match logger_json.write_data(&dummy) {
///     Ok(_) => println!("Succeded in writing in the file."),
///     Err(e) => println!("Something went wrong: {e}"),
//  };
///
/// // Retrieve an iterator over the logged items.
/// let iterator = logger_json.retrieve_iterator().expect("Failed to retrieve iterator");
/// 
/// let mut dummies: Vec<Dummy> = Vec::new();
/// for item in objects {
///     let json_item = item.unwrap();
///     dummies.push(serde_json::from_value::<Dummy>(json_item).unwrap())
/// }
/// 
/// ```
pub struct JSONLogger {
    //pub file: File,
    /// The file handle for the log file.
    file: File,
    /// The name of the log file.
    file_name: String,
}

impl JSONLogger {

    /// Creates a new `Logger` instance for logging data to the specified file.
    ///
    /// # Arguments
    ///
    /// * `path` - A string representing the file path for the log file.
    ///
    pub fn new(path: &str) -> JSONLogger {
        // creating or opening the log file
        let file = fs::OpenOptions::new().create(true).append(true).open(path);

        JSONLogger { 
            file: file.unwrap(),
            file_name: path.to_string(),  
        }
    }

    /// Writes data of the type T to the JSON log file.
    ///
    /// # Arguments
    ///
    /// * `data` - The data to be logged.
    ///
    pub fn write_data<T: Serialize>(&self, data: T) -> std::io::Result<()> {
        
        let mut writer = BufWriter::new(&self.file);

        // the to_vec funciton will give us a byte vector that can go directly to the writer
        let data_string = serde_json::to_vec(&data);

        writer.write_all(&data_string.unwrap())?;
        writer.flush()?;

        Ok(())
    }

    /// Retrieves an iterator over the logged items in the JSON log file.
    ///
    /// # Returns
    ///
    /// A `Result` containing an iterator over items of type `Result<JSON_Value, JSONError>`,
    /// or an `std::io::Error` if there was an issue reading from the log file.
    ///
    pub fn retrieve_iterator(&self) -> Result<impl Iterator<Item = Result<JSON_Value, JSONError>>, std::io::Error> {

        let file = File::open(&self.file_name)?;
        let reader = BufReader::new(file);

        // this can be a one liner
        let deserializer = JSON_Deserializer::from_reader(reader);
        // the type of the below variable is serde_json::StreamDeserializer<'_, serde_json::de::IoRead<BufReader<File>>, Value>
        // more information here https://docs.rs/serde_json/latest/serde_json/struct.StreamDeserializer.html
        let iterator = deserializer.into_iter::<JSON_Value>();
        
        Ok(iterator)
    }
}

/// A generic binary logger for serializing and deserializing data of a specified type.
///
/// The `BinLogger` struct allows you to log data of a generic type to a binary file.
/// It provides methods for creating a new logger, writing data to the log file, and
/// retrieving an iterator over the logged items.
/// 
/// It uses the standard marker crate to be able to get the generic type that is going to be
/// serialized and deserialized to and from the file. It can be used to serialize any struct
/// for example as long as you derive the Serialize and Deserialize traits from serde such as
/// 
/// #[derive(Deserialize, Serialize)]
/// Struct test {...}
/// 
/// 
/// # Examples
///
/// ```
/// use my_module::BinLogger;
///
/// // Create a new BinLogger for logging Dummy values to a file named "bin_test.log".
/// let logger_bin: BinLogger<Dummy> = BinLogger::new("bin_test.log");
///
/// // Write data to the log file.
/// logger.write_data(&42).expect("Failed to write data to log file");
///
/// // Retrieve an iterator over the logged items.
/// let iterator = logger.retrieve_iterator().expect("Failed to retrieve iterator");
/// for item in iterator {
///     println!("Logged item: {}", item);
/// }
/// ```

// The for<'a> is a generic lifetime paramater
// in this context we are putting a trait bound on T where we say that this type
// needs to have the Deserialize trait for any lifetime using the for<'a> Deserialize<'a>
pub struct BinLogger<T: Serialize + DeserializeOwned + for<'a> Deserialize<'a>>{
    // The file handle for the log file.
    file: File,
    // The name of the log file.
    file_name:  String,
    // unsafe rust!
    _data_type: marker::PhantomData<T>
}

impl<T: Serialize + DeserializeOwned + for<'a> Deserialize<'a>> BinLogger<T>{

    /// Creates a new `BinLogger` instance for logging data to the specified file.
    ///
    /// # Arguments
    ///
    /// * `path` - A string representing the file path for the log file.
    ///
    pub fn new(path: &str) -> BinLogger<T> {
        // creating the new log file
        let file = fs::OpenOptions::new().create(true).append(true).open(path);

        BinLogger { 
            file: file.unwrap(),
            file_name: path.to_string(),
            // Marker to indicate the phantom data type.
            _data_type: PhantomData::<T>
        }    
    }
    
    /// Writes data of the generic type `T` to the binary log file.
    ///
    /// # Arguments
    ///
    /// * `data` - A reference to the data to be logged.
    ///
    /// # Errors
    ///
    /// Returns an `std::io::Result` indicating success or an error if the write operation fails.
    ///
    pub fn write_data(&self, data: &T) -> std::io::Result<()> {
        
        let mut writer = BufWriter::new(&self.file);
        
        // the same functionality of the serde_json::to_vec() function
        let bytes = rmp_serde::to_vec(&data).expect("Failed to serialize");

        // here we are computing the hash based on the bytes generated by the data
        let crc = crc32fast::hash(&bytes);

        // here we are serializing the checksum so that we can write into the file
        let crc_bytes = rmp_serde::to_vec(&crc).expect("Failed to serialize the checksum");
        
        //println!("{:?}",crc);
        
        // we always will write the data and after the checksum as a u32 integer
        writer.write_all(&bytes)?;
        writer.write_all(&crc_bytes)?;
        writer.flush()?;

        Ok(())

    }

    /// Retrieves an iterator over the logged items in the binary log file.
    ///
    /// # Errors
    ///
    /// Returns an `std::io::Result` indicating success or an error if the read operation fails.
    ///
    pub fn retrieve_iterator(&self) -> Result<impl Iterator<Item = T>, std::io::Error> {
        
        let file = File::open(&self.file_name)?;
        let mut reader = BufReader::new(file);

        // Use from_fn to create an iterator directly!!!
        let iterator = std::iter::from_fn(move || {
            // here we are going to match the from_read function of rmp_serde
            match rmp_serde::from_read::<_, T>(&mut reader) {
                Ok(value) => {
                    
                    // case we get a value we serialize again to check the value in the checksum
                    let bytes = rmp_serde::to_vec(&value).expect("Failed to serialize for checksum");

                    // hashing again
                    let crc_check =  crc32fast::hash(&bytes);

                    // here we are retrieving the checksum in for the object in question
                    let crc_bytes = match rmp_serde::from_read::<_, u32>(&mut reader){
                        Ok(checksum) => checksum,
                        Err(err) => {
                            eprintln!("Failed to recover checksum: {:?}", err);
                            return None
                        },
                    };

                    // if the checksum does not match we stop deserializing
                    if !(crc_check == crc_bytes) {
                        eprintln!("Checksum failed in deserialization process, terminating early");
                        return None
                    }

                    //println!("CHECKSUM WORKED");
                    Some(value)
                },
                Err(err) => {
                    match err {
                        rmp_serde::decode::Error::InvalidMarkerRead(io_error) => {
                            if let ErrorKind::UnexpectedEof = io_error.kind() {
                                return None
                            } else {
                                eprintln!("Error in the deserialization process: {:?}", io_error);
                                return None    
                            }
                        },
                        _ => {
                            eprintln!("Error in the deserialization process: {:?}", err);
                            None
                        } 
                    }
                },
            }
        });

        // we return the iterator created
        Ok(iterator)

    }

}