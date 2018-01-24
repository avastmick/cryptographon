use std::env;
extern crate cipher_two;

//  Fails on reading args from commandline...
fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() == 4 {
        // Should be looking to decode
        let key = &args[2];
        let message = &args[3];
        if args[2] == "-d" {
            // Need to pass the key location to the decode func
            cipher_two::print_out(cipher_two::decode(key.as_str(), message.as_str()));
        } else {
            // Print usage message
            cipher_two::usage();
        }
    } else if args.len() == 3 {
        let key = &args[1];
        let message = &args[2];
        cipher_two::print_out(cipher_two::encode(key.as_str(), message.as_str()));
    } else if args.len() == 2 {
        // Generate key
        let key = &args[1];
        let message = &args[2];
        cipher_two::print_out(cipher_two::encode(key.as_str(), message.as_str()));
    } else {
        // Print usage message
        cipher_two::usage();
    }
}
