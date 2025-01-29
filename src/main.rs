use std::path::PathBuf;

use clap::{Parser, ValueEnum};
use const_format::{formatcp, str_index};
use dotenv_parser::parse_dotenv;

const VERSION: &str = formatcp!(
    "{} ({})",
    env!("CARGO_PKG_VERSION"),
    str_index!(env!("VERGEN_GIT_SHA"), 0..8),
);

#[derive(Parser)]
#[clap(version = VERSION)]
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

    let paths = path
        .map(|path| Box::new([path].into_iter()) as Box<dyn Iterator<Item = PathBuf>>)
        .unwrap_or_else(|| Box::new([".env", ".envrc"].map(PathBuf::from).into_iter()));

    for path in paths {
        let Ok(env_text) = std::fs::read_to_string(&path) else {
            continue;
        };

        let envs = parse_dotenv(&env_text).expect("Failed to parse .env");

        let output = match output_format {
            OutputFormat::Toml => toml::to_string(&envs).expect("Failed to serialize to TOML."),
            OutputFormat::Json => {
                serde_json::to_string_pretty(&envs).expect("Failed to serialize to JSON.")
            }
        };
        println!("{output}");

        break;
    }
}
