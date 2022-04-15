//! Config parser
//! 
//! This library is made to read config files and parse them onto a HashMap<String, String> type.
//! 
//! # Syntax
//! Following file is content of `/home/user/test.conf` file:
//! ```text
//! * This is a comment line
//! * Each line which begin with asterisk (*) are comment lines
//! 
//! * Network settings
//! port = 2022     // Everything after " //" are comments
//! threads=8     // Not required to write space before and after "=" sign
//! 
//! %include /home/user/test-security.conf
//! %include /home/user/test-user.conf
//! ```
//! 
//! The %include statement results to read those files too and parse them. 
//! 
//! Content of `/home/user/test-security.conf` file:
//! ```text
//! username = vajk
//! api_key = ASDKJAKLSDNKJA()=FNK+JKLKOL+Ü)CSA
//! ```
//! 
//! Always the latest setting stay for same option. Content of `/home/user/test-user.conf`:
//! ```text
//! port = 3800
//! ```
//! 
//! # Example for call
//! ```
//! fn main() {
//!     let config = onlyati_config::read_config("/home/user/test.conf");
//!     if let Ok(conf) = &config {
//!         for (key, value) in conf {
//!             println!("[{}] - [{}]", key, value);
//!         }
//!     }
//! 
//!     if let Err(e) = &config {
//!         println!("Error: {}", e);
//!     }
//! }
//! ```
//! 
//! Output of program above:
//! ```text
//! [username] - [vajk]
//! [port] - [3800]
//! [threads] - [8]
//! [api_key] - [ASDKJAKLSDNKJA()=FNK+JKLKOL+Ü)CSA]
//! ```

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
        Err(_) => return Err(format!("Error during reading: {}", config_path)),
    }

    // Create HashMap
    let mut config: HashMap<String, String> = HashMap::new();

    let mut config_lines = config_string.lines();
    let mut line = config_lines.next();

    let mut line_number: i32 = 0;

    // Getting read lines
    while line != None {
        line_number = line_number + 1;
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
                if &point[0..8] == "%include" {
                    let mut alt_lines = point.split_whitespace();
                    let _ = alt_lines.next();
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
                            return Err(format!("{}:{} -> Only absolute path be specify behind %include statement: {}", config_path, line_number, point));
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
