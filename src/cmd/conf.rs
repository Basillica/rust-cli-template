use anyhow::{Ok, Result};
use clap::Args;

use crate::utils;


#[derive(Debug, Args)]
#[command(version, about, long_about = None)]
pub struct ConfigArgs{}

pub fn main(args: &ConfigArgs, cfg: &mut utils::config::AppConfig) -> Result<()> {
    println!("command args: {:?}", args);
    let full_path = confy::get_configuration_file_path("myserv-cli", None);
    println!("config file full_path: {:?}", full_path);
    println!("app-config: {:?}", cfg);
    println!("redacted db conn str: {:?}", redact_str(&cfg.db_conn_str));
    Ok(())
}

fn redact_str(text: &str) -> String {
    let mut result =  String::with_capacity(text.len());
    result.extend((0..std::cmp::max(text.len() -6, 0)).map(|_| '*'));
    result.push_str(&text[std::cmp::max(text.len() - 6, 0)..]);
    result
}