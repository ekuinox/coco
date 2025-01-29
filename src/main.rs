use std::path::PathBuf;

use clap::Parser;
use const_format::{formatcp, str_index};
use dotenv_parser::parse_dotenv;

/// デフォルトで検索するパスのリスト
const DEFAULT_ENV_FILES: &[&str] = &[".env", ".envrc"];

const VERSION: &str = formatcp!(
    "{} ({})",
    env!("CARGO_PKG_VERSION"),
    str_index!(env!("VERGEN_GIT_SHA"), 0..8),
);

/// .env ファイルを TOML に変換する
#[derive(Parser)]
#[clap(version = VERSION)]
pub struct Args {
    /// ファイルへのパスを指定する
    /// 指定のない場合、 `.env`, `.envrc` の順に検索して使用する
    pub path: Option<PathBuf>,
}

fn main() {
    let Args { path } = Args::parse();

    let paths = path
        .map(|path| Box::new([path].into_iter()) as Box<dyn Iterator<Item = PathBuf>>)
        .unwrap_or_else(|| Box::new(DEFAULT_ENV_FILES.iter().map(From::from).into_iter()));

    for path in paths {
        if !path.exists() {
            continue;
        }

        let env_text = std::fs::read_to_string(&path).expect("Failed to read file.");

        let envs = parse_dotenv(&env_text).expect("Failed to parse .env");

        let output = toml::to_string(&envs).expect("Failed to serialize to TOML.");

        println!("{output}");

        // TOML として出力できた段階で終了
        break;
    }
}
