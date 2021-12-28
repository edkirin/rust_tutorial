use std::fs::File;
use std::io::prelude::*;
use serde::{Deserialize, Serialize};

type Error = Box<dyn std::error::Error>;

const JSON_DATA_FILE: &str = "demodata/people-in-space.json";


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
    file.read_to_string(&mut data)?;
    Ok(data)
}


fn create_data_from_json(json_data: &str) -> Result<PeopleInSpace, Error> {
    let value = serde_json::from_str(json_data)?;
    Ok(value)
}


pub fn handle_json_demo() {
    match read_from_file(JSON_DATA_FILE) {
        Ok(json_data) => {
            match create_data_from_json(&json_data) {
                Ok(pis) => {
                    println!("Number of people in space: {}", pis.number);
                    println!("Message: {}", pis.message);

                    for person in pis.people {
                        println!("{}: {}", person.craft, person.name);
                    }
                }
                Err(err) => {
                    println!("Error parsing json: {}", err);
                }
            }
        }
        Err(err) => {
            println!("Error reading file '{}': {}", JSON_DATA_FILE, err);
        }
    }
}
