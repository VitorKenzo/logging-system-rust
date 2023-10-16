use logging_system::Logger;
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
    
    // creating a logger object
    let logger = Logger::new("test_logger.log");

    // let's check if the file is open
    let open = logger.file.metadata().is_ok();
    if open {
        println!("File open!");
    } else {
        println!("File is closed!");
    }

    // creating a for loop so that we can check how the write is happening
    for i in 0..5 {

        // creating generic objects to test the functions
        let objects = vec![
            DummyObjects { a: i, b: i},
            DummyObjects { a: i, b: i},
            DummyObjects { a: i, b: i},
        ];    
        
        let dummy = Dummy {
            id: i,
            comment: "test".to_string(),
            objects: objects,
        };
    
        // Using the writing function
        match logger.write_data_into_file(&dummy) {
            Ok(_) => println!("Succeded in writing in the file"),
            Err(e) => println!("Something went wrong: {e}"),
        };

    }

    // let's check if the file is open
    let open = logger.file.metadata().is_ok();
    if open {
        println!("File open!");
    } else {
        println!("File is closed!");
    }

    // Using the retrieving function
    let objects = logger.retrieve_iterator_from_log().unwrap();

    // checking the contents
    let mut dummies: Vec<Dummy> = Vec::new();
    for item in objects {
        let json_item = item.unwrap();
        //println!("Got {:#?}", json_item);
        
        dummies.push(serde_json::from_value::<Dummy>(json_item).unwrap())
    }

    // printing the retrieved objects
    for dummy in dummies {
        println!("Dummy found! {:#?}", dummy);
    }

    // let's check if the file is open
    let open = logger.file.metadata().is_ok();
    if open {
        println!("File open!");
    } else {
        println!("File is closed!");
    }

}
