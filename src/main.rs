extern crate cipher_two;

fn main() {

    println!("Encoding \"Hello, world!\" gives: ");
    cipher_two::print_out(cipher_two::encode("Hello World!"));
}


