use std::fs;
use std::str;

const ALPHABET_SIZE: u8 = 26;

fn main() -> anyhow::Result<()> {
    let data_file = "data/The_Open_Window.txt";
    let data_bytes = fs::read(data_file)?;

    let a = 7;
    let b = 25;
    let cipher_bytes = encrypt(&data_bytes, a, b)?;
    let cipher_text = str::from_utf8(&cipher_bytes)?;
    println!("{cipher_text}");
    Ok(())
}

fn encrypt(src: &[u8], a: u8, b: u8) -> anyhow::Result<Vec<u8>, anyhow::Error> {
    let mut cipher_bytes = Vec::new();
    for c in src {
        if !c.is_ascii_alphabetic() {
            cipher_bytes.push(*c);
            continue;
        }


        let c = {
            if c.is_ascii_uppercase() {
                let c = c.wrapping_mul(a).wrapping_add(b).rem_euclid(ALPHABET_SIZE);
                c + b'A'
            } else { 
                let c = c.wrapping_mul(a).wrapping_add(b).rem_euclid(ALPHABET_SIZE);
                c + b'a'
            }
        };
        cipher_bytes.push(c);
    }

    Ok(cipher_bytes)
}
