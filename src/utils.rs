#[cfg(feature = "lambda")]
use tracing_subscriber;

#[cfg(feature = "lambda")]
/// Setup tracing
pub fn setup_tracing() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // this needs to be set to false, otherwise ANSI color codes will
        // show up in a confusing manner in CloudWatch logs.
        .with_ansi(false)
        .json()
        .init();
}
