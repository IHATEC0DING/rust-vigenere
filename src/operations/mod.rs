pub fn vigenere_encrypt(text: &str, key: &str) -> String {
    let key = key.to_uppercase();
    let mut result = String::new();
    let mut key_iter = key.chars().cycle(); // copying the key and cycling it over and over

    for c in text.chars() {
        if c.is_ascii_alphabetic() {
            let is_upper = c.is_ascii_uppercase();
            let offset = if is_upper { b'A' } else { b'a' };
            let key_char = key_iter.next().unwrap();
            let shift = (key_char as u8 - b'A') as u8;

            let encrypted = ((c as u8 - offset + shift) % 26 + offset) as char;
            result.push(encrypted);
        } else {
            result.push(c); // Symbols out of alphab stay the same
        }
    }

    result // Result return
}

pub fn vigenere_decrypt(text: &str, key: &str) -> String {
    let key = key.to_uppercase();
    let mut result = String::new();
    let mut key_iter = key.chars().cycle(); // Key copying

    for c in text.chars() {
        if c.is_ascii_alphabetic() {
            let is_upper = c.is_ascii_uppercase();
            let offset = if is_upper { b'A' } else { b'a' };
            let key_char = key_iter.next().unwrap();
            let shift = (key_char as u8 - b'A') as u8;

            let decrypted = ((c as u8 - offset + 26 - shift) % 26 + offset) as char;
            result.push(decrypted);
        } else {
            result.push(c); // Symbols out of alphab stay the same
        }
    }

    result // Result return
}

