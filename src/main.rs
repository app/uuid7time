use std::io::{self, BufRead};
use uuid::Uuid;
use chrono::{TimeZone, Utc};
use clap::Parser;
use serde::Serialize;

/// Extract timestamp from UUID version 7
#[derive(Parser, Debug)]
#[command(name = "uuid7time")]
#[command(version)]
#[command(about = "Extract timestamps from UUID version 7", long_about = None)]
struct Cli {
    /// UUID(s) to extract timestamp from
    #[arg(value_name = "UUID")]
    uuids: Vec<String>,
    
    /// Output format: iso, unix, unix-ms, json
    #[arg(short, long, value_name = "FORMAT", default_value = "iso")]
    format: String,
    
    /// Output unix timestamp in seconds (shortcut for --format unix)
    #[arg(short = 'u', long, conflicts_with = "format")]
    unix: bool,
    
    /// Output unix timestamp in milliseconds (shortcut for --format unix-ms)
    #[arg(short = 'U', long, conflicts_with = "format")]
    unix_ms: bool,
    
    /// Output JSON format (shortcut for --format json)
    #[arg(short = 'j', long, conflicts_with = "format")]
    json: bool,
    
    /// Suppress error messages
    #[arg(short, long)]
    quiet: bool,
}

#[derive(Debug, Clone, PartialEq)]
enum OutputFormat {
    Iso,
    Unix,
    UnixMs,
    Json,
}

impl OutputFormat {
    fn from_cli(cli: &Cli) -> Result<Self, String> {
        if cli.unix {
            Ok(OutputFormat::Unix)
        } else if cli.unix_ms {
            Ok(OutputFormat::UnixMs)
        } else if cli.json {
            Ok(OutputFormat::Json)
        } else {
            match cli.format.to_lowercase().as_str() {
                "iso" => Ok(OutputFormat::Iso),
                "unix" => Ok(OutputFormat::Unix),
                "unix-ms" => Ok(OutputFormat::UnixMs),
                "json" => Ok(OutputFormat::Json),
                _ => Err(format!("Unknown format: {}. Use: iso, unix, unix-ms, json", cli.format)),
            }
        }
    }
}

#[derive(Serialize)]
struct JsonOutput {
    uuid: String,
    timestamp_ms: i64,
    timestamp_sec: i64,
    iso8601: String,
    rfc3339: String,
}

/// Extract timestamp in milliseconds from UUID v7
fn extract_timestamp_ms(uuid: &Uuid) -> Result<i64, String> {
    // UUIDv7: first 6 bytes = 48-bit timestamp in milliseconds
    let bytes = uuid.as_bytes();
    let ts_ms = u64::from_be_bytes([
        0, 0,
        bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5]
    ]);
    
    Ok(ts_ms as i64)
}

/// Format timestamp according to specified output format
fn format_timestamp(uuid_str: &str, ts_ms: i64, format: &OutputFormat) -> Result<String, String> {
    let dt = Utc.timestamp_millis_opt(ts_ms)
        .single()
        .ok_or_else(|| "Timestamp out of range".to_string())?;
    
    match format {
        OutputFormat::Iso => {
            Ok(dt.to_rfc3339_opts(chrono::SecondsFormat::Millis, true))
        },
        OutputFormat::Unix => {
            Ok((ts_ms / 1000).to_string())
        },
        OutputFormat::UnixMs => {
            Ok(ts_ms.to_string())
        },
        OutputFormat::Json => {
            let output = JsonOutput {
                uuid: uuid_str.to_string(),
                timestamp_ms: ts_ms,
                timestamp_sec: ts_ms / 1000,
                iso8601: dt.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
                rfc3339: dt.to_rfc3339_opts(chrono::SecondsFormat::Millis, false),
            };
            serde_json::to_string(&output)
                .map_err(|e| format!("JSON serialization error: {}", e))
        },
    }
}

/// Process a single UUID
fn process_uuid(uuid_str: &str, format: &OutputFormat) -> Result<String, String> {
    let uuid = Uuid::parse_str(uuid_str.trim())
        .map_err(|e| format!("Invalid UUID: {}", e))?;
    
    let ts_ms = extract_timestamp_ms(&uuid)?;
    format_timestamp(uuid_str.trim(), ts_ms, format)
}

fn main() {
    let cli = Cli::parse();
    
    // Determine output format
    let format = match OutputFormat::from_cli(&cli) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };
    
    // Collect UUIDs from args or stdin
    let uuid_inputs: Vec<String> = if !cli.uuids.is_empty() {
        cli.uuids.clone()
    } else {
        // Read from stdin
        let stdin = io::stdin();
        let reader = stdin.lock();
        reader.lines()
            .filter_map(|line| line.ok())
            .filter(|line| !line.trim().is_empty())
            .collect()
    };
    
    if uuid_inputs.is_empty() {
        if !cli.quiet {
            eprintln!("Error: No UUID provided. Use --help for usage information.");
        }
        std::process::exit(1);
    }
    
    // Process each UUID
    let mut had_error = false;
    for uuid_str in uuid_inputs {
        match process_uuid(&uuid_str, &format) {
            Ok(output) => println!("{}", output),
            Err(e) => {
                if !cli.quiet {
                    eprintln!("Error: {}", e);
                }
                had_error = true;
            }
        }
    }
    
    if had_error {
        std::process::exit(1);
    }
}
