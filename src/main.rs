use std::env;

mod filesys;

static TEMP: &'static str = "Test string";

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Calling tools with args: {:#?}", args);
    filesys::create_file("test_file.txt".to_string(), TEMP.to_string());
}
