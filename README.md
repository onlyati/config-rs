# Config parser

This library is made to read config files and parse them onto a HashMap<String, String> type.

## Syntax
Following file is content of `/home/user/test.conf` file:
```
* This is a comment line
* Each line which begin with asterisk (*) are comment lines

* Network settings
port = 2022     // Everything after " //" are comments
threads=8     // Not required to write space before and after "=" sign

%include /home/user/test-security.conf
%include /home/user/test-user.conf
```

The %include statement results to read those files too and parse them. 

Content of `/home/user/test-security.conf` file:
```
username = vajk
api_key = ASDKJAKLSDNKJA()=FNK+JKLKOL+Ü)CSA
```

Always the latest setting stay for same option. Content of `/home/user/test-user.conf`:
```
port = 3800
```

## Example for call
```rust
fn main() {
        let config = onlyati_config::read_config("/home/user/test.conf");
        if let Ok(conf) = &config {
                for (key, value) in conf {
                        println!("[{}] - [{}]", key, value);
                }
        }

        if let Err(e) = &config {
                println!("Error: {}", e);
        }
}
```

Output of program above:
```
[username] - [vajk]
[port] - [3800]
[threads] - [8]
[api_key] - [ASDKJAKLSDNKJA()=FNK+JKLKOL+Ü)CSA]
```
