use std::fs;

use num_modular::ModularUnaryOps;

const ALPHABET_SIZE: u8 = 26;
const M: u32 = ALPHABET_SIZE as u32;

fn main() -> anyhow::Result<()> {
    let data_file = "data/The_Open_Window.txt";
    let data_bytes = fs::read(data_file)?;

    let a = 7;
    let b = 25;
    let cipher_bytes = encrypt(&data_bytes, a, b)?;
    if let Some(a_inv) = a.invm(&ALPHABET_SIZE) {
        let plain_bytes = decrypt(&cipher_bytes, a_inv, b)?;
        assert_eq!(plain_bytes, data_bytes);
        println!("ROUND TRIP SUCCEEDED");
    } else {
        println!("ROUND TRIP FAILED");
    }

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
        let x = u32::from(c - base);
        let enc = ((u32::from(a) * x + u32::from(b)) % M) as u8;
        cipher_bytes.push(enc + base);
    }

    Ok(cipher_bytes)
}

fn decrypt(src: &[u8], a_inv: u8, b: u8) -> anyhow::Result<Vec<u8>> {
    let mut plain_bytes = Vec::new();
    for &c in src {
        if !c.is_ascii_alphabetic() {
            plain_bytes.push(c);
            continue;
        }
        let base = if c.is_ascii_uppercase() { b'A' } else { b'a' };
        let x = u32::from(c - base);
        let dec = (u32::from(a_inv) * ((x + M - u32::from(b) % M) % M) % M) as u8;
        plain_bytes.push(dec + base);
    }

    Ok(plain_bytes)
}
