use clap::{Parser, Subcommand};
use std::error::Error;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli{
    /// Optional name to operate on
    pub name: Option<String>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub debug: u8, 

    #[command(subcommand)]
    command: Option<Commands>
}
enum
fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    for _ in 0..=cli.count{
        println!("cli name is {:?}", cli.name);
    }
    Ok(())
}
