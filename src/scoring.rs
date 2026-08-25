use std::collections::HashSet;
use std::fs;
use std::path;

fn read_corpus(data_dir: &path::Path) -> anyhow::Result<String> {
    let mut corpus = String::new();
    for entry in fs::read_dir(data_dir)? {
        let path = entry?.path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("txt") {
            continue;
        }
        corpus.push_str(&fs::read_to_string(&path)?);
        corpus.push('\n');
    }

    Ok(corpus)
}

fn clean_corpus(corpus: &str) -> HashSet<String> {
    corpus
        .split_whitespace()
        .map(|word| word.trim_matches(|c: char| !c.is_alphabetic()))
        .filter(|word| !word.is_empty())
        .map(|word| word.to_lowercase())
        .collect()
}

pub fn load_dictionary(dir: &path::Path) -> anyhow::Result<HashSet<String>> {
    let corpus = read_corpus(dir)?;
    Ok(clean_corpus(&corpus))
}

pub fn score_english(candidate: &[u8], dict: &HashSet<String>) -> usize {
    let text = String::from_utf8_lossy(candidate);
    text.split_whitespace()
        .map(|word| word.trim_matches(|c: char| !c.is_alphabetic()))
        .filter(|word| !word.is_empty())
        .filter(|word| dict.contains(&word.to_lowercase()))
        .count()
}
