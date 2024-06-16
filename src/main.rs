use anyhow::{Ok, Result};
use clap::{Parser, Subcommand};

mod cmd;
mod utils;

#[derive(Parser)]
#[command(version, about, long_about = None )]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init(cmd::init::InitArgs),
    Update(cmd::update::UpdateArgs),
    Configure(cmd::conf::ConfigArgs),

}
fn main() -> Result<()> {
    let mut cfg: utils::config::AppConfig = confy::load("rust-cli", None)?;
    let cli = Cli::parse();
    match &cli.command {
        Commands::Init(args) => {
            cmd::init::main(args, &mut cfg)?;
        },
        Commands::Update(args) => {
            cmd::update::main(args, &mut  cfg)?;
        },
        Commands::Configure(args) => {
            cmd::conf::main(args, &mut cfg)?;
        },
    }
    Ok(())
}
