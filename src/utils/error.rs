use std::fmt;

#[derive(Debug)]
pub enum ParserError {
    MissingHomeDirectory,
    IoError(std::io::Error),
    MalformedLine(String),
}

impl From<std::io::Error> for ParserError {
    fn from(e: std::io::Error) -> Self {
        ParserError::IoError(e)
    }
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParserError::MissingHomeDirectory => {
                write!(f, "[e] Error: Could not havigate home dir.")
            }
            ParserError::IoError(error) => {
                write!(f, "[e] Error: Could not load config.env file. {error}")
            }
            ParserError::MalformedLine(k) => {
                write!(f, "[e] Error: Malformed line in config.env file. {k}")
            }
        }
    }
}
