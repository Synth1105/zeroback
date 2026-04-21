use clap::Parser;
use std::error::Error;
use std::{fs, env};
use zeroback::{self, Args};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let arg = Args::parse();

    let config = arg.config;

    let config_content =
        fs::read_to_string(&config).map_err(|e| format!("Failed to read config: {}", e))?;

    let config = zeroback::parse_config(&config_content)?;

    let header_str = config.header_string();

    let headers = zeroback::parse_header(&header_str)?;

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
