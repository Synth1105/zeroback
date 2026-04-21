use clap::Parser;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct Config {
    pub target: Target,
    pub request: Request,
}

#[derive(Deserialize, Clone)]
pub struct Target {
    pub url: String,
    pub method: String,
    pub port: String,
    pub timeout: i32,
}

#[derive(Deserialize, Default)]
pub struct Request {
    #[serde(default)]
    pub header: HashMap<String, String>,
    #[serde(default)]
    pub body: HashMap<String, serde_json::Value>,
}

impl Request {
    pub fn header_string(&self) -> String {
        self.header
            .iter()
            .map(|(k, v)| format!("{}: {}", k, v))
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn body_string(&self) -> Option<String> {
        if self.body.is_empty() {
            None
        } else {
            let json = serde_json::to_string(&self.body).ok()?;
            Some(json)
        }
    }
}

impl Config {
    pub fn header_string(&self) -> String {
        self.request.header_string()
    }
    pub fn body_string(&self) -> Option<String> {
        self.request.body_string()
    }
    pub fn target(&self) -> Target {
        self.target.clone()
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub config: String,
}

pub fn parse_header(raw_headers: &str) -> Result<HeaderMap, String> {
    let mut headers = HeaderMap::new();

    for line in raw_headers.lines() {
        let trimmed_line = line.trim().trim_end_matches(',');

        if trimmed_line.is_empty() {
            continue;
        }

        if let Some((key, value)) = trimmed_line.split_once(':') {
            let key_trimmed = key.trim();
            let value_trimmed = value.trim();

            let header_name = HeaderName::from_bytes(key_trimmed.as_bytes())
                .map_err(|e| format!("invalid header name '{}': {}", key_trimmed, e))?;

            let header_value = HeaderValue::from_str(value_trimmed)
                .map_err(|e| format!("invalid header value '{}': {}", value_trimmed, e))?;
            headers.append(header_name, header_value);
        } else {
            return Err(format!("syntax error: '{}'", trimmed_line));
        }
    }

    Ok(headers)
}

pub fn parse_config(file: &str) -> Result<Config, String> {
    toml::from_str(file).map_err(|err| format!("Problem parsing config: {err}"))
}
