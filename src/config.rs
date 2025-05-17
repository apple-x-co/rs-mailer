use std::fs::read_to_string;
use anyhow::{Result, anyhow};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Destination {
    pub user: String,
    pub domain: String,
    pub name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub from: Destination,
    pub to: Vec<Destination>,
    pub cc: Option<Vec<Destination>>,
    pub bcc: Option<Vec<Destination>>,
    pub subject: String,
    pub body: String,
    pub html: Option<String>,
}


const JSON_SCHEMA_BYTES: &'static [u8] = include_bytes!("../schema/schema.json");

pub fn parse(config_path: &String) -> Result<Config> {
    let schema = serde_json::from_slice(JSON_SCHEMA_BYTES)?;
    let validator = jsonschema::validator_for(&schema)?;
    let json_string = read_to_string(config_path)?;
    let json: serde_json::Value = serde_json::from_str(&json_string)?;

    if !validator.validate(&json).is_ok() {
        return Err(anyhow!("JSON validation failed".to_string()));
    }

    let config = serde_json::from_str(&json_string)?;

    Ok(config)
}