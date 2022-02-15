//! Library for finding guesses for Wordle
#![warn(clippy::pedantic)]
#![warn(missing_docs)]

/// Helper Utils
pub mod utils;

use anyhow::Result;
use itertools::zip;
use rayon::prelude::*;
use std::include_bytes;
use tracing::instrument;

/// Type alias for the dictionary for guess checking
pub type Dictionary = Vec<Vec<String>>;

/// Loads and returns dictionary from files
///
/// # Arguments
/// * `dict_file` - dictionary file to load
#[instrument]
fn load_dict(dict_file: Vec<u8>) -> Result<Dictionary> {
    let dict_string = String::from_utf8(dict_file)?;
    let dict: Dictionary = dict_string.split("\r\n").map(word_to_vec).collect();
    Ok(dict)
}

/// Decompose word string into vector of the letters in the word
///
/// # Arguments
/// * `word` - word to decompose
#[instrument]
fn word_to_vec(word: &str) -> Vec<String> {
    word.chars().map(|c| c.to_string()).collect()
}

/// Check if a guess is valid
///
/// # Arguments
/// * `state` - the state of the word
/// * `guess` - the guess to check validity for
/// * `excluded_letters` - letters that have already been excluded
/// * `unplaced_letters` - letters that are in the word but have not been placed
/// *  `excluded_placements` - placements of letters in `unplaced_letters` that have been excluded
#[instrument]
fn valid_guess(
    state: &[String],
    guess: &[String],
    excluded_letters: &[String],
    unplaced_letters: &[String],
    excluded_placements: &[Vec<String>],
) -> bool {
    for letters in zip(state, guess) {
        if letters.0 == "_" {
            if excluded_letters.contains(letters.1) && !unplaced_letters.contains(letters.1) {
                return false;
            }
        } else if letters.0 != letters.1 {
            return false;
        }
    }
    for letter in unplaced_letters {
        if !guess.contains(letter) {
            return false;
        }
    }
    for eps in excluded_placements {
        for ep in zip(eps, guess) {
            if ep.0 != "_" && ep.0 == ep.1 {
                return false;
            }
        }
    }
    true
}

/// Generate a vector of valid guesses
/// # Arguments
/// * `state` - State of the guess
/// * `excluded_letters` - letters that have already been excluded
/// * `unplaced_letters` - letters that are in the word but have not been placed
/// /// *  `excluded_placements` - placements of letters in `unplaced_letters` that have been excluded
#[instrument]
pub fn generate_guesses(
    mut state: String,
    mut excluded_letters: Vec<String>,
    mut unplaced_letters: Vec<String>,
    mut excluded_placements: Vec<String>,
) -> Result<Vec<String>> {
    excluded_letters = excluded_letters
        .into_iter()
        .map(|s| s.to_lowercase())
        .collect();
    unplaced_letters = unplaced_letters
        .into_iter()
        .map(|s| s.to_lowercase())
        .collect();
    excluded_placements = excluded_placements
        .into_iter()
        .map(|s| s.to_lowercase())
        .collect();
    let dict_file = include_bytes!("../dictionary/five_letter_words.txt").to_vec();
    let dict: Dictionary = load_dict(dict_file)?;
    state = state.to_lowercase();
    let format_state = word_to_vec(state.as_str());
    let excluded_placements_state = excluded_placements
        .into_iter()
        .map(|s| word_to_vec(s.as_str()))
        .collect::<Vec<Vec<String>>>();
    let valid_guesses = dict
        .into_par_iter()
        .filter(|word| {
            valid_guess(
                &format_state,
                word,
                &excluded_letters,
                &unplaced_letters,
                &excluded_placements_state,
            )
        })
        .map(|g| g.join("").to_uppercase())
        .collect::<Vec<String>>();
    Ok(valid_guesses)
}

// todo add more unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_guesses() -> Result<()> {
        let state = String::from("C__NE");
        let excluded_letters = vec![
            "D".to_string(),
            "E".to_string(),
            "U".to_string(),
            "O".to_string(),
            "G".to_string(),
        ];
        let unplaced_letters = vec!["A".to_string()];
        let excluded_placements = vec!["_A__".to_string()];
        let gussess = generate_guesses(
            state,
            excluded_letters,
            unplaced_letters,
            excluded_placements,
        )?;
        println!("{:?}", gussess);
        Ok(())
    }
}
