extern crate rand;
extern crate serde_json;

use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::collections::HashMap;
use std::collections::HashSet;
use rand::{thread_rng, Rng};

const CODE_LEN: usize = 4;

fn get_rand_code() -> String {
    let mut rng = thread_rng();
    let n: u32 = rng.gen_range(0, 9999);
    let key = format!("{:04}", n);

    String::from(key)
}

/// Creates a new key (CODE) file for the given name
pub fn create_keyfile(name: &str) -> String {
    let alpha = "abcdefghijklmnopqrstuvwxyz";
    let mut keycodes = HashMap::new();
    let mut codes = HashSet::new();
    let mut count = 0;
    while count < alpha.len() {
        // Loop here and check for collisions
        let gencode = get_rand_code();
        if !codes.contains(&gencode) {
            codes.insert(gencode.to_string());
            let key = alpha.chars().nth(count).unwrap().to_string();
            keycodes.insert(key, gencode.to_string());
            count += 1;
        }
    }
    // Create filepath
    let _path = [name, "_keycode"].join("");
    let path = Path::new(_path.as_str());
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("Couldn't create {}: {}", display, why.description()),
        Ok(file) => file,
    };
    // JSON serialisation
    let j = serde_json::ser::to_string_pretty(&keycodes);

    match file.write_all(j.unwrap().as_bytes()) {
        Err(why) => panic!("Couldn't write to {}: {}", display, why.description()),
        Ok(_) => println!("Successfully wrote to {}", display),
    }
    // Return path to new file
    String::from(path.file_name().unwrap().to_str().unwrap())
}

/// Gets a keyfile for a given filename
pub fn get_keyfile(filename: &str) -> HashMap<String, String> {
    println!("Getting keyfile for {}", filename);
    let mut f = File::open(filename).expect("File not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("Something went wrong reading the file");

    let json = serde_json::from_str(&contents);
    let keycodes: HashMap<String, String> = json.unwrap();

    keycodes
}

/// Encode the given message
///
/// assert_eq!("11481249678067805698 10695698668367809533", encode("Hello World"));
pub fn encode(keyfilename: &str, msg: &str) -> String {
    let mut secret = String::new();
    let keycodes = get_keyfile(keyfilename);
    // Iterate through the message
    for c in msg.chars() {
        if c.is_whitespace() {
            secret.push(c);
        } else {
            // get code and add to secret
            let mut key = String::new();
            key.push(c);
            let mut code = String::new();
            if keycodes.contains_key(key.to_lowercase().as_str()) {
                code.push_str(keycodes.get(key.to_lowercase().as_str()).unwrap());
            }
            if code.trim() == "" {
                secret = String::from("Encoding failed! Please use only alphabetical values!");
                break;
            }
            secret += code.as_str();
        }
    }
    secret
}

/// Decode the given message
///
/// assert_eq!("hello world", decode("11481249678067805698 10695698668367809533"));
pub fn decode(keyfilename: &str, secret: &str) -> String {
    let mut msg = String::new();
    let mut code = String::new();
    let keycodes = get_keyfile(keyfilename);
    println!(
        "
    Got secret: {}",
        secret
    );
    for c in secret.chars() {
        if !c.is_whitespace() {
            code.push(c);
            // If we have the right length code
            if code.len() == CODE_LEN {
                // Look up the key from value
                let mut key = String::new();
                for (_key, val) in keycodes.iter() {
                    if val == &code {
                        key.push_str(_key);
                    }
                }
                if key.trim() == "" {
                    msg = String::from("Nothing to decode. Bad code!");
                    break;
                }
                msg += key.as_str();
                // Reset
                code.clear();
            }
        } else {
            // Ignore from code, but add to message
            msg.push(c);
            // Reset
            code.clear();
        }
    }
    msg
}

pub fn usage() {
    println!(
        "
    **********************************************************
    Usage: cipher-one [-d] key message
        
            to decode, use \'-d\'
    **********************************************************"
    );
}

pub fn print_out(msg: String) {
    println!(
        "
    **********************************************************
    {}
    **********************************************************",
        msg
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_create_keyfile() {
        assert_eq!("test-two_keycode", create_keyfile("test-two"));
    }
    #[test]
    fn test_encode() {
        assert_eq!(
            "18900578042804288297 83098297384004287936",
            encode("test_keycode", "Hello World")
        );
    }
    // Should fail if non-alphabetical values are given
    #[test]
    fn test_bad_encode() {
        assert_eq!(
            "Encoding failed! Please use only alphabetical values!",
            encode("test_keycode", "Some garbage like aswqer qr2r232!@")
        );
    }
    #[test]
    fn test_decode() {
        assert_eq!(
            "hello world",
            decode("test_keycode", "18900578042804288297 83098297384004287936")
        );
    }
    // Should fail if non-code entered
    #[test]
    fn test_bad_decode() {
        assert_eq!(
            "Nothing to decode. Bad code!",
            decode("test_keycode", "123817247 123487129471100 12341281")
        );
    }
}
