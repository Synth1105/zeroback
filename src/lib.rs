use clap::Parser;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde::Deserialize;
use std::collections::HashMap;
use std::{env, error::Error, fs};

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

pub async fn run(args: Args) -> Result<(), Box<dyn Error>> {
    let config_content =
        fs::read_to_string(&args.config).map_err(|e| format!("Failed to read config: {}", e))?;

    let config = parse_config(&config_content)?;

    let header_str = config.header_string();

    let headers = parse_header(&header_str)?;

    let body = config.body_string();

    let target = config.target();
    let mut url = reqwest::Url::parse(&target.url)
        .map_err(|e| format!("Failed to parse URL '{}': {}", target.url, e))?;
    if !target.port.is_empty() {
        let port = target
            .port
            .parse()
            .map_err(|_| format!("Invalid port number: {}", target.port))?;
        url.set_port(Some(port)).expect("Failed to set port");
    }

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .timeout(std::time::Duration::from_secs(target.timeout as u64))
        .build()?;

    let request_builder = match target.method.as_str() {
        "GET" => client.get(url.clone()),
        "POST" => {
            let mut req = client.post(url.clone());
            if let Some(body) = &body {
                req = req.body(body.clone());
            }
            req
        }
        "PUT" => {
            let mut req = client.put(url.clone());
            if let Some(body) = &body {
                req = req.body(body.clone());
            }
            req
        }
        "PATCH" => {
            let mut req = client.patch(url.clone());
            if let Some(body) = &body {
                req = req.body(body.clone());
            }
            req
        }
        "DELETE" => client.delete(url.clone()),
        _ => {
            eprintln!("Unsupported method: {}", target.method);
            std::process::exit(1);
        }
    };

    let response = request_builder.send().await?;

    let status = response.status();
    let headers = response.headers().clone();
    let body = response.text().await?;
    if env::var("ZB_DEBUG").is_ok() {
        println!("Status: {}", status);
        println!("Headers:");
        for (key, value) in headers.iter() {
            println!(
                "  {}: {}",
                key.as_str(),
                value.to_str().unwrap_or("<non-utf8>")
            );
        }
        println!("Body:");
    }
    println!("{}", body);

    Ok(())
}
