use std::path::PathBuf;

use clap::{Parser, ValueEnum};
use dotenv_parser::parse_dotenv;

#[derive(Parser)]
pub struct Args {
    pub path: Option<PathBuf>,

    #[clap(short, long, default_value = "toml")]
    pub output_format: OutputFormat,
}

#[derive(ValueEnum, Default, PartialEq, Eq, Clone, Copy, Debug)]
pub enum OutputFormat {
    #[default]
    Toml,
    Json,
}

fn main() {
    let Args {
        path,
        output_format,
    } = Args::parse();

    let path = path.unwrap_or_else(|| ".env".into());

    let env_text = std::fs::read_to_string(&path).expect("Failed to open .env");
    let envs = parse_dotenv(&env_text).expect("Failed to parse .env");

    let output = match output_format {
        OutputFormat::Toml => toml::to_string(&envs).expect("Failed to serialize to TOML."),
        OutputFormat::Json => {
            serde_json::to_string_pretty(&envs).expect("Failed to serialize to JSON.")
        }
    };
    println!("{output}");
}
