use logging_system::{write_data_into_file, retrieve_from_log_n};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct DummyObjects {
    a: u32,
    b: u32
}

#[derive(Deserialize, Serialize, Debug)]
struct Dummy {
    id: u32,
    comment: String,
    objects: Vec<DummyObjects>,
}

fn main() {
    
    let objects = vec![
        DummyObjects { a: 12, b: 10},
        DummyObjects { a: 8, b: 9},
        DummyObjects { a: 7, b: 8},
    ];

    let dummy = Dummy {
        id: 8,
        comment: "test".to_string(),
        objects: objects,
    };

    //using the writing function
    match write_data_into_file("test1.log", &dummy) {
        Ok(_) => println!("Succeded in writing in the file"),
        Err(e) => println!("Something went wrong: {e}"),
    };

    retrieve_from_log_n(&"teste".to_string(), Some(5));
}
