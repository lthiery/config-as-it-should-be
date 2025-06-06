mod config;

use ::config::{Config, Environment, File};
use clap::{Parser, Subcommand};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub gateway: Gateway,
}

#[derive(Debug, Deserialize)]
pub struct Gateway {
    pub address: String,
    pub port: u16,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the config file
    #[arg(short, long, default_value = "config.toml")]
    config_path: String,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run the application
    Run,
}

fn main() {
    let args = Args::parse();

    let config_builder = Config::builder()
        .add_source(File::with_name(&args.config_path))
        .add_source(Environment::with_prefix("APP").separator("__"))
        .build()
        .unwrap();

    let settings: config::Config = config_builder.try_deserialize().unwrap();
    println!("{:?}", settings);
}
