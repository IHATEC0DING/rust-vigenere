mod operations; //Operations module

use std::io::{self, Write}; // I/O
use operations::{vigenere_encrypt, vigenere_decrypt}; // Fn import

fn main() {
    loop {
        println!("Choose operation: (1-encode) (2-decode) (3-exit)");

        // op choice read
        let mut operation = String::new();
        io::stdin().read_line(&mut operation).unwrap();
        let operation = operation.trim(); // deleting whitespaces

        match operation {
            "1" => {
                // text read
                println!("Enter text to encode:");
                let mut text = String::new();
                io::stdin().read_line(&mut text).unwrap();

                // key read
                println!("Enter key:");
                let mut key = String::new();
                io::stdin().read_line(&mut key).unwrap();

                // text encrypting
                let output = vigenere_encrypt(text.trim(), key.trim());
                println!("Encoded output: {}", output);
            }
            "2" => {
                // text reading
                println!("Enter text to decode:");
                let mut text = String::new();
                io::stdin().read_line(&mut text).unwrap();

                // key reading
                println!("Enter key:");
                let mut key = String::new();
                io::stdin().read_line(&mut key).unwrap();

                // text decrypting
                let output = vigenere_decrypt(text.trim(), key.trim());
                println!("Decoded output: {}", output);
            }
            "3" => {
                println!("Exiting...");
                break;
            }
            _ => {
                println!("Invalid option, please choose 1, 2, or 3.");
            }
        }
    }
}
