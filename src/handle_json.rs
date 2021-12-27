use std::fs::File;
use std::io::prelude::*;


struct PersonInSpace {
    craft: String,
    name: String,
}

struct PeopleInSpace {
    message: String,
    number: i32,
    people: Vec<PersonInSpace>,
}


impl PeopleInSpace {
    pub fn read_from_file(&self, filename: &str) -> std::io::Result<()> {
        let mut file = File::open(filename);
        let mut contents = String::new();
        file.read_to_string(&mut contents);
        return contents;
    }
}


// fn read_people_in_space_from_file() -> std::io::Result<()> {
//     const DATA_FILE: &str = "demodata/people-in-space.json";
//     println!("Reading file {}", DATA_FILE);
//
//     let mut file = File::open(DATA_FILE)?;
//     let mut contents = String::new();
//     file.read_to_string(&mut contents)?;
//     println!("{}", contents);
//
//     Ok(())
// }


pub fn handle_json_demo() {
    let pis = PeopleInSpace::new();
    pis.read_from_file("demodata/people-in-space.json");
}
