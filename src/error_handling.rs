use std::fs::File;
use std::io::prelude::*;


const FILENAME: &str = "demodata/people-in-space.json";


fn read_file(filename: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(filename)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    Ok(data)
}


pub fn error_handling_demo() {
    match read_file(FILENAME) {
        Ok(file_contents) => {
            println!("{}", file_contents);
        },
        Err(err) => {
            println!("ERROR: {}", err);
        },
    }
}
