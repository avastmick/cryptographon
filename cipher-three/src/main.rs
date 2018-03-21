extern crate cipher_three;
extern crate clap;
extern crate colored;

use colored::Colorize;
use clap::{App, Arg};

fn main() {
    let matches = App::new("Cryptographon Cipher Three")
        .version("0.1.0")
        .author("Mick <avastmick@outlook.com>")
        .about("Convert plain text into ciphertext using a simple substitution cipher")
        .arg(
            Arg::with_name("KEY")
                .help("The name of the key file")
                .required(true),
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
        println!(
            "\tCiphertext: {}",
            cipher_three::encode(matches.value_of("KEY").unwrap(), plaintext,).yellow()
        );
    } else if let Some(ciphertext) = matches.value_of("decode") {
        println!(
            "\tPlaintext: {}",
            cipher_three::decode(matches.value_of("KEY").unwrap(), ciphertext,).purple()
        );
    } else if matches.is_present("new") {
        println!(
            "\tNew keyfile name: {}",
            cipher_three::create_keyfile(matches.value_of("KEY").unwrap(),).green()
        );
    } else {
        panic!("Nothing in the message!")
    }
}

extern crate assert_cli;

#[cfg(test)]
mod tests {

    use assert_cli::Assert;

    #[test]
    fn test_help() {
        Assert::main_binary()
            .with_args(&["--help"])
            .succeeds()
            .unwrap();
    }

    #[test]
    fn test_fail_no_params() {
        Assert::main_binary().fails().unwrap();
    }

    #[test]
    fn test_key_create() {
        Assert::main_binary()
            .with_args(&["test", "--new"])
            .succeeds()
            .stdout()
            .contains("test_keycode")
            .unwrap();
    }
}
