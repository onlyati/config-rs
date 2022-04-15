use std::collections::HashMap;
use std::path::Path;
use std::fs;

pub fn read_config(config_path: &str) -> Result<HashMap<String, String>, String> {
    // Return if file does not exist
    if !Path::new(config_path).exists() {
        return Err(String::from("Config file does not exist."));
    }

    // Read file, return None if failed
    let config_data = fs::read_to_string(config_path);
    let config_string: String;
    match config_data {
        Ok(r) => config_string = r,
        Err(e) => return Err(format!("Error during reading: {}", config_path)),
    }

    // Create HashMap
    let mut config: HashMap<String, String> = HashMap::new();

    let mut config_lines = config_string.lines();
    let mut line = config_lines.next();

    // Getting read lines
    while line != None {
        if let Some(v) = line {
            // Drop comment lines
            if &v[0..1] == "*" {
                line = config_lines.next();
                continue;
            }

            // Cut comment from the end
            let point: &str;
            if let Some(index) = v.find(" //") {
                point = &v[0..index];
            }
            else {
                point = &v[..];
            }

            // Process the pure data
            if let Some(index) = point.find("=") {
                let property = String::from(&point[0..index]).trim().to_string();
                let value = String::from(&point[index+1..]).trim().to_string();
                config.insert(property, value);
            }
        }
        

        line = config_lines.next();
    }


    return Ok(config);
}