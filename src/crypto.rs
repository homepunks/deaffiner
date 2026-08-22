use num_modular::ModularUnaryOps;

pub const ALPHABET_SIZE: u8 = 26;
const M: u32 = ALPHABET_SIZE as u32;

pub fn encrypt(src: &[u8], a: u8, b: u8) -> anyhow::Result<Vec<u8>> {
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

pub fn decrypt(src: &[u8], a_inv: u8, b: u8) -> anyhow::Result<Vec<u8>> {
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

pub fn brute_force(src: &[u8], a_inv: u8, b: u8) -> anyhow::Result<Vec<Vec<u8>>> {
    let results = Vec::new(); 


    Ok(results)
}

fn affine_keys() -> impl Iterator<Item = (u8, u8)> {
    (1..M).filter_map(|a| a.invm(&M)).flat_map(|a_inv| (0..M).map(move |b| (a_inv as u8 , b as u8)))
}
