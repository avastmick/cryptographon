extern crate cipher_one;

fn main() {
    println!("Encoding \"Hello, world!\" gives: ");
    cipher_one::print_out(cipher_one::encode("Hello World!"));
}
