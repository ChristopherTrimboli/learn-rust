use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

const JSON_PATH: &str = "src/json_file.json";

#[derive(Serialize, Deserialize, Debug)]
struct Json {
    message: String,
}

fn write_json(json: Json) -> Result<(), Box<dyn Error>> {
    let file = File::create(JSON_PATH)?;
    serde_json::to_writer(file, &json)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_exists = Path::new(JSON_PATH)
        .try_exists()
        .expect("Can't check existence of file does_not_exist.txt");
    if file_exists {
        let new_json = Json {
            message: "I love the world".to_string(),
        };
        write_json(new_json)?;
        let file = File::open(JSON_PATH)?;
        let reader = BufReader::new(file);
        let json: Json = serde_json::from_reader(reader)?;
        println!("{:?}", json.message);
        Ok(())
    } else {
        println!("File does not exist");
        Err("File does not exist".into())
    }
}
