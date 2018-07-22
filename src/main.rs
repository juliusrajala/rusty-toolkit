use std::env;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;

static TEMP: &'static str = "Test string";

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Calling tools with args: {:#?}", args);
    create_file("test_file.txt".to_string(), TEMP.to_string());
}

fn create_file(target: String, content: String) {
    let file_path = "prototype/".to_owned() + &target;
    let path = Path::new(&file_path);
    let display = path.display();
    let mut file = match File::create(&path) {
        Err(why) => panic!("Couldnt create {}: {}",
                            display,
                            why),
        Ok(file) => file,
    };

    match file.write_all(&content.as_bytes()) {
        Err(why) => panic!("Couldn't write to file {}: {}",
                            display,
                            why),
        Ok(_) => println!("Succesfully wrote to file {}", display),
    }
}