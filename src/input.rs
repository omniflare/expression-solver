use std::{fs, io};

pub fn import_from_path (path : &str) -> Result<String, io::Error>{ 
    let contents = fs::read_to_string(path)?;
    
    Ok(contents)
}
