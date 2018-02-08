extern crate rand;
extern crate serde_json;

use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::collections::HashMap;
use std::collections::HashSet;
use rand::{thread_rng, Rng};

const CODE_LEN: usize = 6;
const LEXICON: [char; 100] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
    'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B',
    'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U',
    'V', 'W', 'X', 'Y', 'Z', '!', '"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.',
    '/', ':', ';', '<', '=', '>', '?', '@', '[', '\\', ']', '^', '_', '`', '{', '|', '}', '~', ' ',
    '\t', '\n', '\r', '\x0b', '\x0c',
];

fn get_rand_code() -> String {
    let mut rng = thread_rng();
    let n: u32 = rng.gen_range(0, 999999);
    let key = format!("{:06}", n);

    key
}

/// Creates a new key (CODE) file for the given name
pub fn create_keyfile(name: &str) -> String {
    let lexicon = LEXICON;
    let mut keycodes = HashMap::new();
    let mut codes = HashSet::new();
    let mut count = 0;
    while count < lexicon.len() {
        // Loop here and check for collisions
        let gencode = get_rand_code();
        if !codes.contains(&gencode) {
            codes.insert(gencode.to_string());
            let key = lexicon[count].to_string();
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
/// `assert_eq!("867329260437960514960514180364036524194068180364438195960514789640104968", encode("test3_keycode", "Hello World!"));`
pub fn encode(keyfilename: &str, msg: &str) -> String {
    let mut secret = String::new();
    let keycodes = get_keyfile(keyfilename);
    // Iterate through the message
    for c in msg.chars() {
        // get code and add to secret
        let mut key = String::new();
        key.push(c);
        let mut code = String::new();
        if keycodes.contains_key(key.as_str()) {
            code.push_str(&keycodes[key.as_str()]);
        }
        if code.trim() == "" {
            secret = String::from("Encoding failed! Please use only ascii values!");
            break;
        }
        secret += code.as_str();
    }
    secret
}

/// Decode the given message
///
/// `assert_eq!("Hello World!", decode("test3_keycode","867329260437960514960514180364036524194068180364438195960514789640104968"));`
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
        code.push(c);
        // If we have the right length code
        if code.len() == CODE_LEN {
            // Look up the key from value
            let mut key = String::new();
            for (_key, val) in &keycodes {
                if val == &code {
                    key.push_str(_key);
                }
            }
            if key == "" {
                msg = String::from("Nothing to decode. Bad code!");
                break;
            }
            msg += key.as_str();
            // Reset
            code.clear();
        }
    }
    msg
}

pub fn usage() {
    println!(
        "usage: main.py [-h] {{encode,decode,new}} keyfile [message]

Encodes or decodes secret messages

positional arguments:
  {{encode,decode,new}}  States whether the message should be encoded or
                       decoded, or whether a new key should be created.
  keyfile              The encryption key file to use, or the name of a new
                       key file (e.g. alice-bob)
  message              The message to encode / decode, (must be in single \'quotes\')

optional arguments:
  -h, --help           show this help message and exit"
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
        assert_eq!("test-three_keycode", create_keyfile("test-three"));
    }
    #[test]
    fn test_encode() {
        assert_eq!(
            "867329260437960514960514180364036524194068180364438195960514789640104968",
            encode("test3_keycode", "Hello World!")
        );
    }
    // Should fail if non-ascii values are given
    #[test]
    fn test_bad_encode() {
        assert_eq!(
            "Encoding failed! Please use only ascii values!",
            encode("test3_keycode", "\x7f \x7f")
        );
    }
    #[test]
    fn test_decode() {
        assert_eq!(
            "Hello World!",
            decode(
                "test3_keycode",
                "867329260437960514960514180364036524194068180364438195960514789640104968"
            )
        );
    }
    // Should fail if non-code entered
    #[test]
    fn test_bad_decode() {
        assert_eq!(
            "Nothing to decode. Bad code!",
            decode("test3_keycode", "123817247 123487129471100 12341281")
        );
    }
}
