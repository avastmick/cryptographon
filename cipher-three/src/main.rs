use std::env;
extern crate cipher_three;

//  Fails on reading args from commandline...
fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() == 4 {
        // Should be looking to encode of decode
        let key = &args[2];
        let message = &args[3];
        if args[1] == "decode" {
            // Need to pass the key location to the decode func
            cipher_three::print_out(cipher_three::decode(key.as_str(), message.as_str()));
        } else {
            // Print usage message
            cipher_three::print_out(cipher_three::encode(key.as_str(), message.as_str()));
        }
    } else if args.len() == 3 {
        if args[1] == "new" {
            // Must be create new key
            let key = &args[2];
            cipher_three::print_out(cipher_three::create_keyfile(key.as_str()));
        } else {
            // Must be wrong
            cipher_three::usage();
        }
    } else {
        // Print usage message
        cipher_three::usage();
    }
}
