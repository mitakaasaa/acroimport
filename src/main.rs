mod importer;
mod models;
mod pdf;
use crate::importer::import::run_import;

fn main() -> anyhow::Result<()> {
    run_import()?;
    Ok(())
}
