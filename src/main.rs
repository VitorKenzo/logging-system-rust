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
    
    // creating generic objects to test the functions
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

    // Using the writing function
    match write_data_into_file("test1.log", &dummy) {
        Ok(_) => println!("Succeded in writing in the file"),
        Err(e) => println!("Something went wrong: {e}"),
    };

    // Using the retrieving function
    let objects = retrieve_from_log_n(&"test1.log".to_string()).unwrap();

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

}
