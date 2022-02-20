//! Lambda function to generate word guesses
#![warn(clippy::pedantic)]
#![warn(missing_docs)]

use lambda_http::{service_fn, run, Error, IntoResponse, Request, Response};
use serde::{Deserialize, Serialize};
use serde_json::{to_string, from_slice};
use tracing::instrument;
use wordle_solver::{generate_guesses, utils::setup_tracing};

/// Request made to the lambda function
#[derive(Debug, Serialize, Deserialize)]
struct RequestBody {
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
struct ResponseBody {
    /// suggested words
    word_suggestions: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    setup_tracing();
    run(service_fn(handler_fn)).await?;
    Ok(())
}

#[instrument]
async fn handler_fn(event: Request) -> Result<impl IntoResponse, Error> {
    let body = event.body();
    let request_body: RequestBody = from_slice(body)?;
    let guesses = generate_guesses(
        request_body.current_state,
        request_body.excluded_letters,
        request_body.unplaced_letters,
        request_body.excluded_placements,
    )?;
    let response_body = to_string(&ResponseBody {
        word_suggestions: guesses,
    })?;
    Ok(Response::builder()
        .status(200)
        .body(response_body)
        .expect("Failed"))
}
