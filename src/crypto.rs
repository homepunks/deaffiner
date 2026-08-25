use num_modular::{ModularCoreOps, ModularUnaryOps};

pub const ALPHABET_SIZE: u8 = 26;

fn affine_map(src: &[u8], mul: u8, add: u8) -> Vec<u8> {
    const M: u16 = ALPHABET_SIZE as u16;
    src.iter()
        .map(|&c| {
            if !c.is_ascii_alphabetic() {
                return c;
            }
            let base = if c.is_ascii_uppercase() { b'A' } else { b'a' };
            let x = u16::from(c - base);
            let y = (u16::from(mul) * x + u16::from(add)) % M;
            y as u8 + base
        })
        .collect()
}

pub fn encrypt(src: &[u8], a: u8, b: u8) -> Vec<u8> {
    affine_map(src, a, b)
}

pub fn decrypt(src: &[u8], a: u8, b: u8) -> Vec<u8> {
    let a_inv = a.invm(&ALPHABET_SIZE).expect("a must be coprime with 26");
    let add = a_inv.mulm(b, &ALPHABET_SIZE).negm(&ALPHABET_SIZE);
    affine_map(src, a_inv, add)
}

pub fn brute_force(src: &[u8]) -> Vec<(u8, u8, Vec<u8>)> {
    affine_keys()
        .map(|(a_inv, b)| (a_inv, b, decrypt(src, a_inv, b)))
        .collect()
}

fn affine_keys() -> impl Iterator<Item = (u8, u8)> {
    (1..ALPHABET_SIZE)
        .filter(|a| a.invm(&ALPHABET_SIZE).is_some())
        .flat_map(|a| (0..ALPHABET_SIZE).map(move |b| (a, b)))
}
