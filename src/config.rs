use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Server address
    #[clap(short, long, default_value = "127.0.0.1")]
    pub address: String,

    /// Server port
    #[clap(short, long, default_value = "8080")]
    pub port: u16,

    /// Media path
    #[clap(short, long, default_value = "./media")]
    pub media_path: PathBuf,
}
