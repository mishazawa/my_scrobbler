use std::fmt;

#[derive(Debug)]
pub enum ScrobbleManagerError {
    MissingCredential(&'static str),
    KeyringError(keyring::Error),
    ScrobblerError(rustfm_scrobble::ScrobblerError), // adjust to your crate's actual error type
}

impl fmt::Display for ScrobbleManagerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::MissingCredential(key) => write!(f, "{key} is not set. Check config.env file."),
            Self::KeyringError(e) => write!(f, "Can't read password from keychain: {e}"),
            Self::ScrobblerError(e) => write!(f, "Can't authorize on Last.fm: {e}"),
        }
    }
}

impl std::error::Error for ScrobbleManagerError {}

impl From<keyring::Error> for ScrobbleManagerError {
    fn from(e: keyring::Error) -> Self {
        Self::KeyringError(e)
    }
}
