use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error(
        "failed to extract any tex node starting from the tex content \"{}...\"",
        .0.as_str()[..16].to_string()
    )]
    UnknownTexNode(String),

    #[error("there exists a mismatched group in the tex content")]
    GroupMismatch,

    #[error("there exists a mismatched environment in the tex content")]
    EnvironmentMismatch
}