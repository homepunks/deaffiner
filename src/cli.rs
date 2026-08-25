use crate::crypto::{ALPHABET_SIZE, brute_force, decrypt, encrypt};
use crate::scoring::{load_dictionary, score_english};
use clap::{Parser, Subcommand};
use num_modular::ModularUnaryOps;
use std::fs;
use std::path;

/// affine cipher tool
#[derive(Parser)]
#[command(version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// encrypt a file
    Encrypt {
        /// input file
        file: path::PathBuf,
        /// multiplier, must be coprime with 26
        #[arg(short, long, value_parser = parse_multiplier)]
        a: u8,
        /// shift, 0-25
        #[arg(short, long, value_parser = clap::value_parser!(u8).range(0..26))]
        b: u8,
    },

    /// decrypt a file with a known key
    Decrypt {
        /// input file
        file: path::PathBuf,

        /// multiplier, must be coprime with 26
        #[arg(short, long, value_parser = parse_multiplier)]
        a: u8,

        /// shift, 0-25
        #[arg(short, long, value_parser = clap::value_parser!(u8).range(0..26))]
        b: u8,
    },

    /// recover the key by scoring all possible candidates
    Crack {
        /// input file
        file: path::PathBuf,

        /// corpus data dir for cryptanalysis
        #[arg(long, default_value = "data/corpus")]
        corpus: path::PathBuf,
    },
}

pub fn process_cli(cli: Cli) -> anyhow::Result<()> {
    match cli.command {
        Command::Encrypt { file, a, b } => {
            let data = fs::read(&file)?;
            let out = encrypt(&data, a, b);
            println!("{}", String::from_utf8_lossy(&out));
        }
        Command::Decrypt { file, a, b } => {
            let data = fs::read(&file)?;
            let out = decrypt(&data, a, b);
            println!("{}", String::from_utf8_lossy(&out));
        }
        Command::Crack { file, corpus } => {
            let data = fs::read(&file)?;
            let dict = load_dictionary(&corpus)?;
            let best = brute_force(&data)
                .into_iter()
                .max_by_key(|(_, _, sample)| score_english(sample, &dict));

            match best {
                Some((a, b, _)) => {
                    println!("a: {a}, b: {b}");
                }
                None => println!("no candidates"),
            }
        }
    }

    Ok(())
}

fn parse_multiplier(s: &str) -> Result<u8, String> {
    let a: u8 = s.parse().map_err(|_| format!("`{s}` is not a number"))?;
    a.invm(&ALPHABET_SIZE).map(|_| a).ok_or_else(|| {
        format!("{a} is not coprime with 26 - valid values: 1 3 5 7 9 11 15 17 19 21 23 25")
    })
}
