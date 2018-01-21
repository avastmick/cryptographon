use std::env;
extern crate cipher_one;


//  Fails on reading args from commandline...
fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() == 3 { // Should be looking to decode
        let message = &args[2];
        if args[1] == "decode" { 
            cipher_one::print_out(cipher_one::decode(message.as_str()));
        }
        else { // Print usage message
            cipher_one::usage();
        }
    } else if args.len() ==2 {
        let message = &args[1];
        cipher_one::print_out(cipher_one::encode(message.as_str()));
    } else { // Print usage message
        cipher_one::usage();
    }
}
