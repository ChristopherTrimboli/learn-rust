use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

const JSON_PATH: &str = "src/json_file.json";

fn main() -> io::Result<()> {
    let file_exists = Path::new(JSON_PATH)
        .try_exists()
        .expect("Can't check existence of file does_not_exist.txt");
    if file_exists {
        let mut f = File::open(JSON_PATH)?;
        let mut buffer = [0; 10];

        let n = f.read(&mut buffer)?;

        println!("The bytes: {:?}", &buffer[..n]);
    } else {
        println!("File does not exist");
    }
    Ok(())
}
