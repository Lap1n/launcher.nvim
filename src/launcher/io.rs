use std::{fs, io};

use super::LaunchConfig;

pub fn load_json(path: &str) -> Result<LaunchConfig, io::Error> {
    let file = match fs::File::open(path) {
        Ok(file) => file,
        Err(error) => {
            print!("Error opening file: {}", error);
            return Err(error);
        }
    };
    let json = match serde_json::from_reader::<fs::File, LaunchConfig>(file) {
        Ok(json) => json,
        Err(error) => {
            print!("Error parsing file: {}", error);
            return Err(io::Error::new(io::ErrorKind::Other, error));
        }
    };
    return Ok(json);
}
