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
            // Get rid from the empty lines
            if v.trim().is_empty() {
                line = config_lines.next();
                continue;
            }

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

            // Handle special commands
            if point.len() > 8 {
                if &point[0..7] == "%include" {
                    let mut alt_lines = point.split_whitespace();
                    let alt_line = alt_lines.next();
                    let alt_line = alt_lines.next();
                    if let Some(l) = alt_line {
                        if &l[0..1] == "/" {
                            // Absolute paht
                            let alt_config = read_config(l);
                            match alt_config {
                                Ok(r) => {
                                    for (key, value) in r {
                                        config.insert(key, value);
                                    }
                                },
                                Err(e) => return Err(e),
                            }
                        }
                        else
                        {
                            // Relative path
                            let alt_config = read_config(format!("{}/{}", config_path, l).as_str());
                        }
                    }
                    else {
                        return Err(String::from("No file specified after %include statement"));
                    }
                }
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
