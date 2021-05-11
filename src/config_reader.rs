use serde_json::from_reader;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use crate::models::ConfigFile;

pub fn read_config(path: String) -> Result<ConfigFile, Box<dyn Error>> {
    let file = File::open(Path::new(&path))?;
    let reader = BufReader::new(file);
    let config: ConfigFile = from_reader(reader)?;
    Ok(config)
}
