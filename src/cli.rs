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

        /// verbose cryptanalysis output
        #[arg(short, long)]
        verbose: bool,
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
        Command::Crack {
            file,
            corpus,
            verbose,
        } => {
            let data = fs::read(&file)?;
            let dict = load_dictionary(&corpus)?;

            let mut best: Option<(usize, u8, u8)> = None;
            for (a, b, plain) in brute_force(&data) {
                let score = score_english(&plain, &dict);

                if verbose {
                    println!("trying a={a:2} b={b:2} :: score = {score}");
                }

                if best.is_none_or(|(top, ..)| score > top) {
                    best = Some((score, a, b));
                }
            }

            match best {
                Some((score, a, b)) => {
                    println!("+++++++++++++++++++++++++++++");
                    println!("a: {a}, b: {b} (score {score})");
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
