use num_modular::ModularUnaryOps;
use std::fs;
use deaffiner::crypto::{encrypt, brute_force, ALPHABET_SIZE};
use deaffiner::scoring::{load_dictionary, score_english};

fn main() -> anyhow::Result<()> {
    let data_file = "data/The_Open_Window.txt";
    let data_bytes = fs::read(data_file)?;
    let dict = load_dictionary()?;

    let (a, b) = (7, 25);
    let cipher_bytes = encrypt(&data_bytes, a, b);

    let best = brute_force(&cipher_bytes)
        .into_iter()
        .max_by_key(|(_, _, sample)| score_english(sample, &dict));

    match best {
        Some((a_inv_key, b_key, _)) => {
            println!("a_inv: {a_inv_key} (expected {:?})", a.invm(&ALPHABET_SIZE).unwrap());
            println!("b:     {b_key} (expected {b})");
        },
        None => println!("no candidates"),
    }

    Ok(())
}
