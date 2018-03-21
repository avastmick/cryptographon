#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate maplit;

use std::collections::HashMap;

const CODE_LEN: usize = 4;

lazy_static! {
    static ref CODES: HashMap<&'static str, &'static str> = hashmap!{
        "a" => "6520",
        "b" => "2143",
        "c" => "3990",
        "d" => "9533",
        "e" => "1249",
        "f" => "8942",
        "g" => "1043",
        "h" => "1148",
        "i" => "2397",
        "j" => "7753",
        "k" => "6521",
        "l" => "6780",
        "m" => "0067",
        "n" => "1258",
        "o" => "5698",
        "p" => "9901",
        "q" => "9806",
        "r" => "6683",
        "s" => "6799",
        "t" => "5320",
        "u" => "3118",
        "v" => "2679",
        "w" => "1069",
        "x" => "9001",
        "y" => "5477",
        "z" => "9900"
    };
}

/// Gets the key (the source character) for a given cipher text code
fn get_key(code: &String) -> String {
    let mut key = String::new();

    for (_key, val) in CODES.iter() {
        if val == &code {
            key.push_str(_key);
        }
    }
    key
}

/// Get the code for a given key (source character)
fn get_code(key: String) -> String {
    let mut code = String::new();
    if CODES.contains_key(key.to_lowercase().as_str()) {
        code.push_str(CODES.get(key.to_lowercase().as_str()).unwrap());
    }
    code
}

/// Encode the given message
///
/// assert_eq!("11481249678067805698 10695698668367809533", encode("Hello World"));
pub fn encode(msg: &str) -> String {
    let mut secret = String::new();
    // Iterate through the message
    for c in msg.chars() {
        if c == ' ' {
            secret.push(c);
        } else {
            // get code and add to secret
            let mut key = String::new();
            key.push(c);
            secret += &get_code(key);
        }
    }
    if secret.trim() == "" {
        secret += "Encoding failed! Please use on alphabetical values!";
    }
    secret
}

/// Decode the given message
///
/// assert_eq!("hello world", decode("11481249678067805698 10695698668367809533"));
pub fn decode(secret: &str) -> String {
    let mut msg = String::new();
    let mut code = String::new();
    println!(
        "
    Got secret: {}",
        secret
    );
    for c in secret.chars() {
        if c != ' ' {
            code.push(c);
            // If we have the right length code
            if code.len() == CODE_LEN {
                // Look up the key from value
                msg += &get_key(&code);
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
    if msg.trim() == "" {
        msg += "Nothing to decode. Bad code!";
    }
    msg
}

pub fn usage() {
    println!(
        "
    **********************************************************
    Usage: cipher-one [-d] message

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
    fn test_encode() {
        assert_eq!(
            "11481249678067805698 10695698668367809533",
            encode("Hello World")
        );
    }
    #[test]
    fn test_decode() {
        assert_eq!(
            "hello world",
            decode("11481249678067805698 10695698668367809533")
        );
    }
}
