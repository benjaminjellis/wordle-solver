//! Lambda function to generate word guesses
#![warn(clippy::pedantic)]
#![warn(missing_docs)]

use lambda_runtime::{handler_fn, Context, Error};
use serde::{Deserialize, Serialize};
use tracing::instrument;
use wordle_solver::{generate_guesses, utils::setup_tracing};

/// Request made to the lambda function
#[derive(Debug, Serialize, Deserialize)]
struct Request {
    /// current state of the guess
    current_state: String,
    /// letters that have been excluded by previous guesses
    excluded_letters: Vec<String>,
    /// letters that are in the word but not placed yet
    unplaced_letters: Vec<String>,
    /// placements of letters that have been ruled out
    excluded_placements: Vec<String>,
}

/// Response from the lambda function
#[derive(Debug, Serialize, Deserialize)]
struct Response {
    /// suggested words
    word_suggestions: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    setup_tracing();
    let func = handler_fn(handler);
    lambda_runtime::run(func).await?;
    Ok(())
}

#[instrument]
async fn handler(event: Request, _: Context) -> Result<Response, Error> {
    let guesses = generate_guesses(
        event.current_state,
        event.excluded_letters,
        event.unplaced_letters,
        event.excluded_placements,
    )?;
    Ok(Response {
        word_suggestions: guesses,
    })
}
