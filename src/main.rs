use clap::Parser;
use dotenvy::dotenv;
use std::env;
use std::fs;
use anyhow::{anyhow, Result};

mod api;
use api::{call_felo_api, FeloResponseData};

const MAX_QUERY_LENGTH: usize = 2000;
const MIN_QUERY_LENGTH: usize = 1;

/// A command-line interface for the Felo API
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The query to send to the Felo API
    query: String,

    /// Your Felo API key
    #[arg(short, long)]
    api_key: Option<String>,

    /// Path to a file containing your Felo API key
    #[arg(long)]
    api_key_file: Option<String>,

    /// Output the full JSON response
    #[arg(long)]
    json: bool,

    /// Output the raw content (answer field)
    #[arg(long)]
    raw: bool,

    /// Enable debug mode
    #[arg(long)]
    debug: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let args = Args::parse();

    if args.query.len() > MAX_QUERY_LENGTH || args.query.len() < MIN_QUERY_LENGTH {
        return Err(anyhow!(
            "Query length must be between {} and {} characters (got {}).",
            MIN_QUERY_LENGTH,
            MAX_QUERY_LENGTH,
            args.query.len()
        ));
    }

    let api_key = if let Some(key) = args.api_key.clone() {
        key
    } else if let Some(file_path) = args.api_key_file.clone() {
        fs::read_to_string(&file_path)
            .map_err(|e| anyhow!("Failed to read API key from file '{}': {}", file_path, e))?
            .trim()
            .to_string()
    } else {
        env::var("FELO_API_KEY")
            .map_err(|_| anyhow!("API key not found. Please set FELO_API_KEY environment variable, use --api-key flag, or specify --api-key-file."))?
    };

    if args.debug {
        eprintln!(":: Running in debug mode ::");
        eprintln!("Query: {}", args.query);
        eprintln!("API Key: ...{}", &api_key[api_key.len().saturating_sub(4)..]);
        eprintln!("JSON output: {}", args.json);
        eprintln!("Raw output: {}", args.raw);
    }

    let response_data = call_felo_api(&api_key, &args.query).await?;

    if args.debug {
        eprintln!(":: API Response Data ::");
        eprintln!("{:#?}", response_data);
    }
    
    handle_response(response_data, &args)?;

    Ok(())
}

fn handle_response(response_data: FeloResponseData, args: &Args) -> Result<()> {
    if args.json {
        let json_string = serde_json::to_string_pretty(&response_data)?;
        println!("{}", json_string);
    } else if args.raw {
        println!("{}", response_data.answer);
    } else {
        // Default formatted output
        println!("{}", response_data.answer);

        if let Some(resources) = response_data.resources {
            if let Some(resources_array) = resources.as_array() {
                if !resources_array.is_empty() {
                    println!("\n--- Resources ---");
                    for (i, resource) in resources_array.iter().enumerate() {
                        let link = resource["link"].as_str().unwrap_or("N/A");
                        let title = resource["title"].as_str().unwrap_or("N/A");
                        println!("{}. {} ({})", i + 1, title, link);
                    }
                }
            }
        }
    }
    Ok(())
}

