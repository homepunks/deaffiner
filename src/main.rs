use std::fs;
use std::str;

const ALPHABET_SIZE: u32 = 26;

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

fn encrypt(src: &[u8], a: u8, b: u8) -> anyhow::Result<Vec<u8>> {
    let mut cipher_bytes = Vec::new();
    for &c in src {
        if !c.is_ascii_alphabetic() {
            cipher_bytes.push(c);
            continue;
        }
        let base = if c.is_ascii_uppercase() { b'A' } else { b'a' };
        let x = c - base;
        let enc = ((u32::from(a) * u32::from(x) + u32::from(b)) % ALPHABET_SIZE) as u8;
        cipher_bytes.push(enc + base);
    }

    Ok(cipher_bytes)
}
