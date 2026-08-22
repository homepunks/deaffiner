use deaffiner::crypto::{ALPHABET_SIZE, decrypt, encrypt};
use num_modular::ModularUnaryOps;
use std::fs;

fn main() -> anyhow::Result<()> {
    let data_file = "data/The_Open_Window.txt";
    let data_bytes = fs::read(data_file)?;

    let a = 7;
    let b = 25;
    let cipher_bytes = encrypt(&data_bytes, a, b);
    if let Some(a_inv) = a.invm(&ALPHABET_SIZE) {
        let plain_bytes = decrypt(&cipher_bytes, a_inv, b);
        assert_eq!(plain_bytes, data_bytes);
        println!("ROUND TRIP SUCCEEDED");
    } else {
        println!("ROUND TRIP FAILED");
    }

    Ok(())
}
