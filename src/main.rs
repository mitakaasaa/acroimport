mod importer;
mod models;
mod pdf;

fn main() -> anyhow::Result<()> {
    importer::run_import()?;
    Ok(())
}
