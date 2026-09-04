use std::{collections::HashMap, env, fs};

use crate::utils::ParserError;

pub fn parse_config() -> Result<HashMap<String, String>, ParserError> {
    let my_path = env::home_dir()
        .map(|a| a.join(".config").join("my_scrobbler").join("config.env"))
        .ok_or(ParserError::MissingHomeDirectory)?;

    let contents = fs::read_to_string(my_path)?;

    let mut credentials = HashMap::new();

    for line in contents.lines() {
        let line = line.trim();

        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let Some((key, val)) = line.split_once('=') else {
            return Err(ParserError::MalformedLine(line.to_string()));
        };

        credentials.insert(
            key.trim().to_string(),
            val.trim().trim_matches('"').to_string(),
        );
    }

    Ok(credentials)
}
