mod tests;

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    plain
        .chars()
        .filter_map(|c| {
            if c.is_alphabetic() {
                Some(atbash(c.to_ascii_lowercase()))
            } else if c.is_digit(10) {
                Some(c)
            } else {
                None
            }
        })
        .enumerate()
        .map(|(i, c)| {
            if (i + 1) % 5 == 0 {
                format!("{:2}", c)
            } else {
                String::from(c)
            }
        })
        .collect::<String>()
        .trim_end()
        .to_string()
}

fn atbash(c: char) -> char {
    ('a'..='z')
        .nth(25 - ('a'..='z').position(|d| d == c).unwrap())
        .unwrap()
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher
        .chars()
        .filter_map(|c| {
            if c.is_alphabetic() {
                Some(atbash(c.to_ascii_lowercase()))
            } else if c.is_digit(10) {
                Some(c)
            } else {
                None
            }
        })
        .collect()
}
