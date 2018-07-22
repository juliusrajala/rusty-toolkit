use std::fs;
use std::path::Path;
use std::io;
use std::io::prelude::*;

pub fn create_dir(target: String) -> io::Result<()> {
    let path = Path::new(&target);
    fs::create_dir_all(path)
        .expect("Something wen't wrong with file creation");
    Ok(())
}

pub fn create_file(target: String, content: String) {
    let file_path = "prototype/".to_owned() + &target;
    let path = Path::new(&file_path);
    let display = path.display();
    let mut file = match fs::File::create(&path) {
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