use anyhow::{Ok, Result};
use clap::Args;
use async_std::task;

use crate::utils;


#[derive(Debug, Args)]
#[command(version, about, long_about = None)]
pub struct UpdateArgs{
    #[arg(short = 'n', help = "some sample number")]
    number: u8,
}

pub fn main(args: &UpdateArgs, cfg: &mut utils::config::AppConfig) -> Result<()> {
    println!("command args: {:?}", cfg);
    task::block_on(async_job(args.number));
    Ok(())
}

async fn async_job(val: u8) -> u8 {
    println!("result: {}", 5*val);
    5*val
}