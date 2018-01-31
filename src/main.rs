extern crate cipher_two;

fn main() {
    // println!("Encoding \"Hello, world!\" gives: ");
    // cipher_two::print_out(cipher_two::encode("test_keycode", "Hello World"));
    cipher_two::print_out(cipher_two::decode(
        "test_keycode",
        "18900578042804288297 83098297384004287936",
    ));
    // cipher_two::create_keyfile("test-two");
    // println!("Got this {:?}", cipher_two::get_keyfile("test-two_keycode"));
}
