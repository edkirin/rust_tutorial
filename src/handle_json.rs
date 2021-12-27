use std::fs::File;
use std::io::prelude::*;
use serde::{Deserialize, Serialize};
// use serde_json::Result;


#[derive(Serialize, Deserialize)]
struct Person {
    craft: String,
    name: String,
}

#[derive(Serialize, Deserialize)]
struct PeopleInSpace {
    message: String,
    number: i32,
    people: Vec<Person>,
}


fn read_from_file(filename: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(filename)?;
    let mut data = String::new();
    file.read_to_string(&mut data);
    Ok(data)
}


fn create_from_json(json_data: &str) -> PeopleInSpace {
    println!("{}", json_data);

    let pis: PeopleInSpace = serde_json::from_str(json_data).unwrap();
    pis
}


pub fn handle_json_demo() {
    let json_data: String =
        read_from_file("demodata/people-in-space.json")
            .expect("Error reading file!"); // FIXME: error catching not working;

    let pis: PeopleInSpace = create_from_json(&json_data);

    println!("{}", pis.number);
    println!("{}", pis.message);

    for person in pis.people {
        println!("{}: {}", person.craft, person.name);
    }
}
