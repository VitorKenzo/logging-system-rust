use std::io;
use logging_system::{JSONLogger, BinLogger};
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

    // creating two different logger objects, one to test JSON and one to test bin
    // JSON logger object
    let logger_json = JSONLogger::new("json_test.log");
    
    // Bin logger object associated with our type Dummy
    let logger_bin: BinLogger<Dummy> = BinLogger::new("bin_test.log");
    
    // just so that objects have different ids
    let mut i = 0;

    loop {
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

        // Mini menu
        println!();
        println!("Choose from these options bleow:");
        println!("1 - Write objects as JSON onto file");
        println!("2 - Recover objects from JSON format file");
        println!("3 - Write objects in Binary format onto a file");
        println!("4 - Recover objects from binary format file");
        println!("5 - Exit the application");

        // getting input
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => {
                if num == 1 || num == 3 {
                    i = i + 1;
                }
                num
            },
            Err(_) => continue,
        } ;

        match choice {
            1 => {
                // Using the writing function
                println!("Writing object as JSON...");
                match logger_json.write_data(&dummy) {
                    Ok(_) => println!("Succeded in writing in the file."),
                    Err(e) => println!("Something went wrong: {e}"),
                };
            },
            2 => {
                    // Using the retrieving function
                    let objects = logger_json.retrieve_iterator().unwrap();

                    // checking the contents
                    let mut dummies: Vec<Dummy> = Vec::new();
                    for item in objects {
                        match item {
                            Ok(json_item) => {
                                if let Ok(dummy) = serde_json::from_value::<Dummy>(json_item){
                                    dummies.push(dummy);
                                } else {
                                    eprintln!("Error deserializing JSON item, terminating deserialization");
                                    break;
                                }
                            }
                            Err(err) => {
                                eprintln!("Error retrieving JSON item: {:?}", err);
                            }
                        }
                    }

                    // printing the retrieved objects
                    for dummy in dummies {
                        println!("Dummy found! {:#?}", dummy);
                    }
            },
            3 => {
                println!("Writing object in binary format...");
                match logger_bin.write_data(&dummy) {
                    Ok(_) => println!("Succeded in writing in the file."),
                    Err(e) => println!("Something went wrong: {e}"),
                };
            },
            4 => {
                let objects = logger_bin.retrieve_iterator().unwrap();

                for object in objects {
                    println!("Dummy found! {:#?}", object);
                }
            },
            5 => break,
            _ => println!("Not a valid command!"),
        };
    }

}
