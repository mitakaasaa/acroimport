mod db;
mod exporter;
mod importer;
mod models;
mod pdf;
mod utils;

use crate::exporter::export::run_export;
use crate::importer::import::run_import;
use crate::utils::outputs::{write_help, write_incorrect_call, write_team};

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        return write_incorrect_call();
    }

    match args[1].as_str() {
        "/n" => write_team()?,
        "/imp" => run_import()?,
        "/exp" => run_export()?,
        "/help" => write_help()?,
        _ => write_incorrect_call()?,
    }

    Ok(())
}
