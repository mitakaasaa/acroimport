mod importer;
mod models;
mod pdf;
use crate::importer::import::run_import;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();

        match args.get(1).map(|s| s.as_str()) {
            Some("/n") => println!("Denys Stamat - Kate Lagotin - Malcolm Apaloo"),
            Some("/imp") => run_import()?,
            Some("/exp") => println!("Exportiert!"),
            _ => println!("Ungültiger Parameter"),
        }
    Ok(())
}
