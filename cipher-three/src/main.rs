extern crate cipher_three;
extern crate clap;

use clap::{App, Arg};

fn main() {
    let matches = App::new("Cryptographon Cipher Three")
        .version("0.1.0")
        .author("Mick <avastmick@outlook.com>")
        .about("Convert plain text into ciphertext using a simple substitution cipher")
        .arg(
            Arg::with_name("KEY")
                .help("The name of the key file")
                .required(true)
        )
        .arg(
            Arg::with_name("encode")
                .short("e")
                .long("encode")
                .value_name("MESSAGE")
                .help("The MESSAGE to encode")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("decode")
                .short("d")
                .long("decode")
                .value_name("CODE")
                .help("The CODE message to decode")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("new")
                .short("n")
                .long("new")
                .help("Create a new KEY file")
                .conflicts_with_all(&["encode", "decode"])
                .required_unless_one(&["encode", "decode"]),
        )
        .get_matches();

    if let Some(plaintext) = matches.value_of("encode") {
        cipher_three::print_out(cipher_three::encode(
            matches.value_of("KEY").unwrap(),
            plaintext,
        ));
    } else if let Some(ciphertext) = matches.value_of("decode") {
        cipher_three::print_out(cipher_three::decode(
            matches.value_of("KEY").unwrap(),
            ciphertext,
        ));
    } else if matches.is_present("new") {
        cipher_three::print_out(cipher_three::create_keyfile(
            matches.value_of("KEY").unwrap(),
        ));
    } else {
        panic!("Nothing in the message!")
    }
}
