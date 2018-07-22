use std::env;
use std::fs::File;
use std::path::Path;

static temp: &'static str = "Test string";

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Calling tools with args: {:#?}", args);
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

    match file.write_all(temp.as_bytes()) {
        Err(why) => panic!("Couldn't write to file {}: {}",
                            display,
                            why),
        Ok(file) => println!("Succesfully wrote to file {}", display),
    }
}